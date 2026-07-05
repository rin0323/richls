use std::fs;
use std::path::Path;

pub fn is_pdf(path: &Path) -> bool {
    path.extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("pdf"))
}

pub fn read_pdf_title(path: &Path) -> Option<String> {
    let bytes = fs::read(path).ok()?;
    let title = find_pdf_title(&bytes)?;
    Some(format!("PDF: {}", title.trim()))
}

fn find_pdf_title(bytes: &[u8]) -> Option<String> {
    let marker = b"/Title";
    let mut search_from = 0;

    while let Some(offset) = bytes[search_from..]
        .windows(marker.len())
        .position(|window| window == marker)
    {
        let start = search_from + offset + marker.len();
        let after_marker = bytes[start..].trim_ascii_start();

        let value = match after_marker.first() {
            Some(b'(') => read_pdf_string(&after_marker[1..]),
            Some(b'<') if !after_marker.starts_with(b"<<") => {
                read_pdf_hex_string(&after_marker[1..])
            }
            _ => None,
        };

        if let Some(title) = value.and_then(|value| decode_pdf_string(&value)) {
            return Some(title);
        }

        search_from = start;
    }

    None
}

fn read_pdf_string(bytes: &[u8]) -> Option<Vec<u8>> {
    let mut result = Vec::new();
    let mut escaped = false;
    let mut depth = 1usize;

    for byte in bytes {
        if escaped {
            result.push(*byte);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_plain_title() {
        let bytes = b"<< /Title (Generic Malware Unpacking) /Author (rin) >>";
        assert_eq!(
            find_pdf_title(bytes),
            Some("Generic Malware Unpacking".to_string())
        );
    }

    #[test]
    fn extracts_escaped_title() {
        let bytes = br"<< /Title (A \(Small\) Paper) >>";
        assert_eq!(find_pdf_title(bytes), Some("A (Small) Paper".to_string()));
    }

    #[test]
    fn extracts_utf16_hex_title() {
        let bytes = b"<< /Title <FEFF65E5672C8A9E> >>";
        assert_eq!(find_pdf_title(bytes), Some("日本語".to_string()));
    }

    #[test]
    fn skips_similar_keys_and_malformed_titles() {
        assert_eq!(find_pdf_title(b"<< /TitleFont (Helvetica) >>"), None);
        assert_eq!(find_pdf_title(b"<< /Title <not-hex> >>"), None);
        assert_eq!(find_pdf_title(b"<< /Author (rin) >>"), None);
    }
}
