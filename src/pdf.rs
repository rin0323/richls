use std::fs;
use std::path::Path;

pub fn is_pdf(path: &Path) -> bool {
    path.extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("pdf"))
}

pub fn read_pdf_title(path: &Path) -> Option<String> {
    let bytes = fs::read(path).ok()?;
    let title = find_pdf_metadata_title(&bytes);
    pdf_display_label(title.as_deref())
}

/// Formats a PDF title for the INFO column, or leaves the column empty.
pub fn pdf_display_label(title: Option<&str>) -> Option<String> {
    title
        .and_then(normalize_title)
        .map(|title| format!("PDF: {title}"))
}

fn find_pdf_metadata_title(bytes: &[u8]) -> Option<String> {
    for offset in key_offsets(bytes, b"/Info").into_iter().rev() {
        let after_key = bytes[offset + b"/Info".len()..].trim_ascii_start();
        let Some(info_dictionary) = read_info_dictionary(bytes, after_key) else {
            continue;
        };
        if let Some(title) = title_from_info_dictionary(info_dictionary) {
            return Some(title);
        }
    }
    None
}

fn read_info_dictionary<'a>(bytes: &'a [u8], after_key: &'a [u8]) -> Option<&'a [u8]> {
    if after_key.starts_with(b"<<") {
        return read_dictionary_slice(after_key);
    }

    let (object, generation) = parse_indirect_reference(after_key)?;
    let object_body = find_indirect_object(bytes, object, generation)?;
    read_dictionary_slice(object_body.trim_ascii_start())
}

fn title_from_info_dictionary(dictionary: &[u8]) -> Option<String> {
    for offset in key_offsets(dictionary, b"/Title") {
        let after_key = dictionary[offset + b"/Title".len()..].trim_ascii_start();
        let Some(raw_title) = read_title_value(after_key) else {
            continue;
        };
        if let Some(title) = decode_pdf_string(&raw_title).and_then(|value| normalize_title(&value))
        {
            return Some(title);
        }
    }
    None
}

fn read_title_value(bytes: &[u8]) -> Option<Vec<u8>> {
    match bytes.first() {
        Some(b'(') => read_pdf_string(&bytes[1..]),
        Some(b'<') if !bytes.starts_with(b"<<") => read_pdf_hex_string(&bytes[1..]),
        _ => None,
    }
}

fn key_offsets(bytes: &[u8], key: &[u8]) -> Vec<usize> {
    bytes
        .windows(key.len())
        .enumerate()
        .filter_map(|(index, window)| {
            (window == key && key_has_boundary(bytes, index, key)).then_some(index)
        })
        .collect()
}

fn key_has_boundary(bytes: &[u8], index: usize, key: &[u8]) -> bool {
    bytes
        .get(index + key.len())
        .is_none_or(|byte| is_pdf_delimiter(*byte))
}

fn is_pdf_delimiter(byte: u8) -> bool {
    byte.is_ascii_whitespace() || matches!(byte, b'/' | b'<' | b'>' | b'[' | b']' | b'(' | b')')
}

fn read_dictionary_slice(bytes: &[u8]) -> Option<&[u8]> {
    let mut depth = 0usize;
    let mut index = 0usize;
    while index + 1 < bytes.len() {
        match &bytes[index..index + 2] {
            b"<<" => depth += 1,
            b">>" => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(&bytes[..index + 2]);
                }
            }
            _ => {}
        }
        index += 1;
    }
    None
}

fn parse_indirect_reference(bytes: &[u8]) -> Option<(u32, u32)> {
    let (object, rest) = parse_uint(bytes)?;
    let (generation, rest) = parse_uint(rest.trim_ascii_start())?;
    (rest.trim_ascii_start().first() == Some(&b'R')).then_some((object, generation))
}

fn parse_uint(bytes: &[u8]) -> Option<(u32, &[u8])> {
    let end = bytes.iter().position(|byte| !byte.is_ascii_digit())?;
    let value = std::str::from_utf8(&bytes[..end]).ok()?.parse().ok()?;
    Some((value, &bytes[end..]))
}

fn find_indirect_object(bytes: &[u8], object: u32, generation: u32) -> Option<&[u8]> {
    let header = format!("{object} {generation} obj");
    let start = find_bytes(bytes, header.as_bytes())? + header.len();
    let end = find_bytes(&bytes[start..], b"endobj")?;
    Some(&bytes[start..start + end])
}

fn find_bytes(bytes: &[u8], needle: &[u8]) -> Option<usize> {
    bytes
        .windows(needle.len())
        .position(|window| window == needle)
}

fn read_pdf_string(bytes: &[u8]) -> Option<Vec<u8>> {
    let mut result = Vec::new();
    let mut escaped = false;
    let mut depth = 1usize;

    for byte in bytes {
        if escaped {
            push_escaped_byte(&mut result, *byte);
            escaped = false;
            continue;
        }

        match *byte {
            b'\\' => escaped = true,
            b'(' => {
                depth += 1;
                result.push(*byte);
            }
            b')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(result);
                }
                result.push(*byte);
            }
            _ => result.push(*byte),
        }
    }

    None
}

fn push_escaped_byte(result: &mut Vec<u8>, byte: u8) {
    let value = match byte {
        b'n' => b'\n',
        b'r' => b'\r',
        b't' => b'\t',
        b'b' => 0x08,
        b'f' => 0x0c,
        _ => byte,
    };
    result.push(value);
}

fn read_pdf_hex_string(bytes: &[u8]) -> Option<Vec<u8>> {
    let end = bytes.iter().position(|byte| *byte == b'>')?;
    let digits: Vec<u8> = bytes[..end]
        .iter()
        .copied()
        .filter(|byte| !byte.is_ascii_whitespace())
        .collect();
    let mut result = Vec::with_capacity(digits.len().div_ceil(2));

    for pair in digits.chunks(2) {
        let high = hex_value(pair[0])?;
        let low = match pair.get(1) {
            Some(value) => hex_value(*value)?,
            None => 0,
        };
        result.push((high << 4) | low);
    }

    Some(result)
}

fn hex_value(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        b'A'..=b'F' => Some(value - b'A' + 10),
        _ => None,
    }
}

fn decode_pdf_string(bytes: &[u8]) -> Option<String> {
    if bytes.starts_with(&[0xfe, 0xff]) {
        let utf16: Vec<u16> = bytes[2..]
            .chunks_exact(2)
            .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
            .collect();
        return String::from_utf16(&utf16)
            .ok()
            .filter(|value| !value.is_empty());
    }

    String::from_utf8(bytes.to_vec())
        .ok()
        .or_else(|| Some(String::from_utf8_lossy(bytes).into_owned()))
        .filter(|value| !value.is_empty())
}

fn normalize_title(title: &str) -> Option<String> {
    let normalized = title.split_whitespace().collect::<Vec<_>>().join(" ");
    (!normalized.is_empty()).then_some(normalized)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn extracts_indirect_metadata_title() {
        let bytes = b"%PDF-1.4
1 0 obj << /Title (report_final.pdf) >> endobj
2 0 obj << /Title (Generic Malware Unpacking) /Author (rin) >> endobj
trailer << /Root 3 0 R /Info 2 0 R >>";

        assert_eq!(
            find_pdf_metadata_title(bytes),
            Some("Generic Malware Unpacking".to_string())
        );
    }

    #[test]
    fn extracts_direct_metadata_title() {
        let bytes = b"%PDF-1.4
trailer << /Info << /Title (Direct Metadata Title) >> >>";

        assert_eq!(
            find_pdf_metadata_title(bytes),
            Some("Direct Metadata Title".to_string())
        );
    }

    #[test]
    fn extracts_escaped_title() {
        let bytes = br"1 0 obj << /Title (A \(Small\) Paper) >> endobj
trailer << /Info 1 0 R >>";

        assert_eq!(
            find_pdf_metadata_title(bytes),
            Some("A (Small) Paper".to_string())
        );
    }

    #[test]
    fn extracts_utf16_hex_title() {
        let bytes = b"1 0 obj << /Title <FEFF65E5672C8A9E> >> endobj
trailer << /Info 1 0 R >>";

        assert_eq!(find_pdf_metadata_title(bytes), Some("日本語".to_string()));
    }

    #[test]
    fn skips_similar_keys_and_malformed_titles() {
        assert_eq!(
            find_pdf_metadata_title(b"trailer << /Info << /TitleFont (Helvetica) >> >>"),
            None
        );
        assert_eq!(
            find_pdf_metadata_title(b"trailer << /Info << /Title <not-hex> >> >>"),
            None
        );
        assert_eq!(
            find_pdf_metadata_title(b"trailer << /Info << /Author (rin) >> >>"),
            None
        );
    }

    #[test]
    fn omits_empty_blank_and_broken_titles() {
        assert_eq!(
            find_pdf_metadata_title(b"trailer << /Info << /Title () >> >>"),
            None
        );
        assert_eq!(
            find_pdf_metadata_title(b"trailer << /Info << /Title (   ) >> >>"),
            None
        );
        assert_eq!(
            find_pdf_metadata_title(b"not a valid metadata trailer"),
            None
        );
    }

    #[test]
    fn normalizes_title_whitespace() {
        let bytes = b"trailer << /Info << /Title (  A
  Small\tPaper  ) >> >>";

        assert_eq!(
            find_pdf_metadata_title(bytes),
            Some("A Small Paper".to_string())
        );
    }

    #[test]
    fn labels_available_titles() {
        assert_eq!(
            pdf_display_label(Some("  Generic Malware Unpacking  ")),
            Some("PDF: Generic Malware Unpacking".to_string())
        );
    }

    #[test]
    fn omits_label_when_title_is_unavailable() {
        assert_eq!(pdf_display_label(None), None);
        assert_eq!(pdf_display_label(Some("  ")), None);
    }

    #[test]
    fn read_failure_leaves_title_unavailable() {
        assert_eq!(
            read_pdf_title(Path::new("/definitely/missing/title.pdf")),
            None
        );
    }
}
