// takes ascii text starting with a digit and returns a parsed int + remaining slice
pub fn scan_ascii_to_u32(text: &[u8]) -> (u32, &[u8]) {
    let mut number = 0;
    let mut len = 0;
    for byte in text {
        match byte {
            b'0' ..= b'9' => {
                number *= 10;
                number += (byte - b'0') as u32;
                len += 1;
            },
            _ => break
        }
    }
    return (number, &text[len..]);
}

pub fn skip_ascii_whitespace(text: &[u8]) -> &[u8] {
    let mut len = 0;
    while len < text.len() && (text[len] as char).is_ascii_whitespace() {
        len += 1;
    }
    return &text[len..];
}

pub fn skip_to_ascii_whitespace(text: &[u8]) -> &[u8] {
    let mut len = 0;
    while len < text.len() && !(text[len] as char).is_ascii_whitespace() {
        len += 1;
    }
    return &text[len..];
}
