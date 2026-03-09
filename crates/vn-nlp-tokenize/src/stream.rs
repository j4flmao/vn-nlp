use vn_nlp_core::{Span, Token, TokenKind, VnNlpError};

use crate::unicode;

/// Lazy iterator cho tokenization — không pre-allocate.
///
/// # Examples
/// ```
/// use vn_nlp_tokenize::TokenStream;
///
/// let stream = TokenStream::new("Xin chào Việt Nam");
/// let tokens: Vec<_> = stream.collect::<Result<Vec<_>, _>>().unwrap();
/// assert_eq!(tokens.len(), 4);
/// ```
pub struct TokenStream<'a> {
    input: &'a str,
    offset: usize,
}

impl<'a> TokenStream<'a> {
    /// Tạo token stream từ input.
    pub fn new(input: &'a str) -> Self {
        Self { input, offset: 0 }
    }
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = Result<Token<'a>, VnNlpError>;

    fn next(&mut self) -> Option<Self::Item> {
        // Skip whitespace
        while self.offset < self.input.len() {
            let remaining = &self.input[self.offset..];
            let c = remaining.chars().next()?;

            if c.is_whitespace() {
                self.offset += c.len_utf8();
            } else {
                break;
            }
        }

        if self.offset >= self.input.len() {
            return None;
        }

        let remaining = &self.input[self.offset..];
        let first_char = remaining.chars().next()?;

        // Punctuation: single char token
        if unicode::is_punctuation(first_char) {
            let start = self.offset;
            let end = start + first_char.len_utf8();
            self.offset = end;
            return Some(Ok(Token {
                text: &self.input[start..end],
                span: Span::new(start, end),
                kind: TokenKind::Punctuation,
            }));
        }

        // Number
        if unicode::is_digit(first_char) {
            let start = self.offset;
            let len: usize = remaining
                .chars()
                .take_while(|c| unicode::is_digit(*c) || *c == '.' || *c == ',')
                .map(|c| c.len_utf8())
                .sum();
            let end = start + len;
            self.offset = end;
            return Some(Ok(Token {
                text: &self.input[start..end],
                span: Span::new(start, end),
                kind: TokenKind::Number,
            }));
        }

        // Word (alphabetic + hyphen)
        if first_char.is_alphabetic() {
            let start = self.offset;
            let len: usize = remaining
                .chars()
                .take_while(|c| c.is_alphabetic() || *c == '-')
                .map(|c| c.len_utf8())
                .sum();
            let end = start + len;
            self.offset = end;
            return Some(Ok(Token {
                text: &self.input[start..end],
                span: Span::new(start, end),
                kind: TokenKind::Word,
            }));
        }

        // Unknown
        let start = self.offset;
        let end = start + first_char.len_utf8();
        self.offset = end;
        Some(Ok(Token {
            text: &self.input[start..end],
            span: Span::new(start, end),
            kind: TokenKind::Unknown,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stream_basic() {
        let tokens: Vec<_> = TokenStream::new("Xin chào")
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].text, "Xin");
        assert_eq!(tokens[1].text, "chào");
    }

    #[test]
    fn stream_empty() {
        let tokens: Vec<_> = TokenStream::new("").collect::<Result<Vec<_>, _>>().unwrap();
        assert!(tokens.is_empty());
    }

    #[test]
    fn stream_mixed() {
        let tokens: Vec<_> = TokenStream::new("Năm 2024!")
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].kind, TokenKind::Word);
        assert_eq!(tokens[1].kind, TokenKind::Number);
        assert_eq!(tokens[2].kind, TokenKind::Punctuation);
    }

    #[test]
    fn stream_lazy_iteration() {
        let mut stream = TokenStream::new("a b c");
        assert_eq!(stream.next().unwrap().unwrap().text, "a");
        assert_eq!(stream.next().unwrap().unwrap().text, "b");
        assert_eq!(stream.next().unwrap().unwrap().text, "c");
        assert!(stream.next().is_none());
    }
}
