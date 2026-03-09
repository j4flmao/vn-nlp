/// Kiểm tra ký tự có phải là ký tự tiếng Việt (bao gồm dấu) không.
pub fn is_vietnamese_char(c: char) -> bool {
    c.is_alphabetic()
}

/// Kiểm tra ký tự có phải dấu câu không.
pub fn is_punctuation(c: char) -> bool {
    matches!(
        c,
        '.' | ','
            | '!'
            | '?'
            | ';'
            | ':'
            | '"'
            | '\''
            | '('
            | ')'
            | '['
            | ']'
            | '{'
            | '}'
            | '–'
            | '—'
            | '…'
            | '/'
            | '\\'
            | '-'
    )
}

/// Kiểm tra ký tự có phải số không.
pub fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}
