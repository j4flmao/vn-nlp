use vn_nlp::{tokenize, TokenKind};

#[test]
fn tokenize_simple_vn_sentence() {
    let tokens = tokenize("Tôi yêu Việt Nam").unwrap();
    let words: Vec<&str> = tokens.iter().map(|t| t.text).collect();
    assert_eq!(words, vec!["Tôi", "yêu", "Việt", "Nam"]);
}

#[test]
fn tokenize_with_punctuation() {
    let tokens = tokenize("Xin chào, Việt Nam!").unwrap();
    assert!(tokens.iter().any(|t| t.kind == TokenKind::Punctuation));
}

#[test]
fn tokenize_preserves_zero_copy() {
    let input = "Xin chào Việt Nam";
    let tokens = tokenize(input).unwrap();
    for token in &tokens {
        assert_eq!(token.text, &input[token.span.start..token.span.end]);
    }
}

#[test]
fn tokenize_numbers() {
    let tokens = tokenize("Năm 2024").unwrap();
    assert!(tokens
        .iter()
        .any(|t| t.kind == TokenKind::Number && t.text == "2024"));
}

#[test]
fn tokenize_complex_vn_text() {
    let tokens = tokenize("TP.HCM có 13 triệu dân, là thành phố lớn nhất.").unwrap();
    assert!(!tokens.is_empty());
    assert!(tokens.iter().any(|t| t.kind == TokenKind::Number));
    assert!(tokens.iter().any(|t| t.kind == TokenKind::Punctuation));
    assert!(tokens.iter().any(|t| t.kind == TokenKind::Word));
}

#[test]
fn tokenize_special_chars() {
    let tokens = tokenize("email: test@example.com — xem thêm").unwrap();
    assert!(!tokens.is_empty());
}

#[test]
fn tokenize_unicode_nfd_input() {
    // NFD: "chào" as separate combining characters
    let nfd = "cha\u{0300}o";
    let tokens = tokenize(nfd).unwrap();
    assert!(!tokens.is_empty());
}

#[test]
fn tokenize_long_text() {
    let text = "Xin chào. ".repeat(1000);
    let tokens = tokenize(&text).unwrap();
    assert!(tokens.len() > 1000);
}
