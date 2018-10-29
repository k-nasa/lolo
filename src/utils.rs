pub fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '|'
}

pub fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}
