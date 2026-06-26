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
    let start = bytes
        .windows(marker.len())
        .position(|window| window == marker)?;
    let after_marker = &bytes[start + marker.len()..];
    let open = after_marker.iter().position(|byte| *byte == b'(')?;
    let value = read_pdf_string(&after_marker[open + 1..])?;

    decode_pdf_string(&value)
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
}
