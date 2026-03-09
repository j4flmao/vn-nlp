use vn_nlp_core::{Span, Token, TokenKind, Tokenizer, VnNlpError};

use crate::unicode;

/// Tokenizer tách theo âm tiết (syllable-based).
///
/// Phân tách văn bản tiếng Việt theo khoảng trắng, phân loại token
/// thành Word, Punctuation, Number, Whitespace.
///
/// # Examples
/// ```
/// use vn_nlp_tokenize::SyllableTokenizer;
/// use vn_nlp_core::Tokenizer;
///
/// let tokenizer = SyllableTokenizer::default();
/// let tokens = tokenizer.tokenize("Xin chào!").unwrap();
/// assert_eq!(tokens.len(), 3); // "Xin", "chào", "!"
/// ```
#[derive(Debug, Clone)]
pub struct SyllableTokenizer {
    keep_whitespace: bool,
    keep_punctuation: bool,
    _lowercase: bool,
}

impl Default for SyllableTokenizer {
    fn default() -> Self {
        Self {
            keep_whitespace: false,
            keep_punctuation: true,
            _lowercase: false,
        }
    }
}

impl SyllableTokenizer {
    /// Tạo builder để cấu hình tokenizer.
    pub fn builder() -> SyllableTokenizerBuilder {
        SyllableTokenizerBuilder::default()
    }

    /// Phân loại một đoạn text thành TokenKind.
    fn classify(text: &str) -> TokenKind {
        let first = match text.chars().next() {
            Some(c) => c,
            None => return TokenKind::Unknown,
        };

        if text.chars().all(|c| c.is_whitespace()) {
            TokenKind::Whitespace
        } else if text
            .chars()
            .all(|c| unicode::is_digit(c) || c == '.' || c == ',')
            && first.is_ascii_digit()
        {
            TokenKind::Number
        } else if text.chars().all(unicode::is_punctuation) {
            TokenKind::Punctuation
        } else if text
            .chars()
            .all(|c| unicode::is_vietnamese_char(c) || c == '-')
        {
            TokenKind::Word
        } else {
            TokenKind::Unknown
        }
    }
}

impl Tokenizer for SyllableTokenizer {
    type Error = VnNlpError;

    fn tokenize<'a>(&self, input: &'a str) -> Result<Vec<Token<'a>>, Self::Error> {
        let mut tokens = Vec::new();
        let mut offset = 0;

        while offset < input.len() {
            let remaining = &input[offset..];

            // Tìm đoạn whitespace
            let ws_len = remaining
                .chars()
                .take_while(|c| c.is_whitespace())
                .map(|c| c.len_utf8())
                .sum::<usize>();

            if ws_len > 0 {
                if self.keep_whitespace {
                    let text = &input[offset..offset + ws_len];
                    tokens.push(Token {
                        text,
                        span: Span::new(offset, offset + ws_len),
                        kind: TokenKind::Whitespace,
                    });
                }
                offset += ws_len;
                continue;
            }

            // Tìm đoạn non-whitespace
            let token_len = remaining
                .chars()
                .take_while(|c| !c.is_whitespace())
                .map(|c| c.len_utf8())
                .sum::<usize>();

            if token_len > 0 {
                let raw_text = &input[offset..offset + token_len];

                // Tách punctuation ra riêng khỏi từ
                let sub_tokens = split_punctuation(raw_text, offset);

                for (sub_text, sub_offset) in sub_tokens {
                    let kind = Self::classify(sub_text);

                    if kind == TokenKind::Punctuation && !self.keep_punctuation {
                        // Skip punctuation
                    } else {
                        tokens.push(Token {
                            text: sub_text,
                            span: Span::new(sub_offset, sub_offset + sub_text.len()),
                            kind,
                        });
                    }
                }

                offset += token_len;
            }
        }

        Ok(tokens)
    }
}

/// Tách dấu câu dính vào từ: "chào!" → ["chào", "!"]
fn split_punctuation(text: &str, base_offset: usize) -> Vec<(&str, usize)> {
    let mut result = Vec::new();
    let mut start = 0;
    let chars: Vec<(usize, char)> = text.char_indices().collect();

    if chars.is_empty() {
        return result;
    }

    let mut i = 0;
    while i < chars.len() {
        let (byte_idx, c) = chars[i];
        let is_punct = unicode::is_punctuation(c);

        if is_punct {
            // Flush từ trước đó
            if start < byte_idx {
                result.push((&text[start..byte_idx], base_offset + start));
            }
            // Thêm punctuation
            let end = byte_idx + c.len_utf8();
            result.push((&text[byte_idx..end], base_offset + byte_idx));
            start = end;
        }

        i += 1;
    }

    // Flush phần còn lại
    if start < text.len() {
        result.push((&text[start..], base_offset + start));
    }

    result
}

/// Builder cho `SyllableTokenizer`.
#[derive(Debug, Clone)]
pub struct SyllableTokenizerBuilder {
    keep_whitespace: bool,
    keep_punctuation: bool,
    lowercase: bool,
}

impl Default for SyllableTokenizerBuilder {
    fn default() -> Self {
        Self {
            keep_whitespace: false,
            keep_punctuation: true,
            lowercase: false,
        }
    }
}

impl SyllableTokenizerBuilder {
    /// Giữ lại token whitespace hay không.
    pub fn keep_whitespace(mut self, keep: bool) -> Self {
        self.keep_whitespace = keep;
        self
    }

    /// Giữ lại token dấu câu hay không.
    pub fn keep_punctuation(mut self, keep: bool) -> Self {
        self.keep_punctuation = keep;
        self
    }

    /// Có lowercase tất cả token hay không.
    pub fn lowercase(mut self, lower: bool) -> Self {
        self.lowercase = lower;
        self
    }

    /// Build `SyllableTokenizer`.
    pub fn build(self) -> SyllableTokenizer {
        SyllableTokenizer {
            keep_whitespace: self.keep_whitespace,
            keep_punctuation: self.keep_punctuation,
            _lowercase: self.lowercase,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_simple_sentence() {
        let tokenizer = SyllableTokenizer::default();
        let tokens = tokenizer.tokenize("Xin chào").unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].text, "Xin");
        assert_eq!(tokens[0].kind, TokenKind::Word);
        assert_eq!(tokens[1].text, "chào");
        assert_eq!(tokens[1].kind, TokenKind::Word);
    }

    #[test]
    fn tokenize_with_punctuation() {
        let tokenizer = SyllableTokenizer::default();
        let tokens = tokenizer.tokenize("Xin chào, Việt Nam!").unwrap();
        let texts: Vec<&str> = tokens.iter().map(|t| t.text).collect();
        assert_eq!(texts, vec!["Xin", "chào", ",", "Việt", "Nam", "!"]);
    }

    #[test]
    fn tokenize_empty_string() {
        let tokenizer = SyllableTokenizer::default();
        let tokens = tokenizer.tokenize("").unwrap();
        assert!(tokens.is_empty());
    }

    #[test]
    fn tokenize_only_whitespace() {
        let tokenizer = SyllableTokenizer::default();
        let tokens = tokenizer.tokenize("   ").unwrap();
        assert!(tokens.is_empty());
    }

    #[test]
    fn tokenize_only_punctuation() {
        let tokenizer = SyllableTokenizer::default();
        let tokens = tokenizer.tokenize("...!?").unwrap();
        assert!(tokens.iter().all(|t| t.kind == TokenKind::Punctuation));
    }

    #[test]
    fn tokenize_mixed_vn_and_ascii() {
        let tokenizer = SyllableTokenizer::default();
        let tokens = tokenizer.tokenize("Hello Việt Nam world").unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].text, "Hello");
        assert_eq!(tokens[1].text, "Việt");
    }

    #[test]
    fn tokenize_with_numbers() {
        let tokenizer = SyllableTokenizer::default();
        let tokens = tokenizer.tokenize("Năm 2024 có 365 ngày").unwrap();
        let numbers: Vec<&Token> = tokens
            .iter()
            .filter(|t| t.kind == TokenKind::Number)
            .collect();
        assert_eq!(numbers.len(), 2);
        assert_eq!(numbers[0].text, "2024");
        assert_eq!(numbers[1].text, "365");
    }

    #[test]
    fn tokenize_span_correctness() {
        let input = "Xin chào";
        let tokenizer = SyllableTokenizer::default();
        let tokens = tokenizer.tokenize(input).unwrap();
        for token in &tokens {
            assert_eq!(token.text, &input[token.span.start..token.span.end]);
        }
    }

    #[test]
    fn tokenize_keep_whitespace() {
        let tokenizer = SyllableTokenizer::builder().keep_whitespace(true).build();
        let tokens = tokenizer.tokenize("a b").unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[1].kind, TokenKind::Whitespace);
    }

    #[test]
    fn tokenize_skip_punctuation() {
        let tokenizer = SyllableTokenizer::builder().keep_punctuation(false).build();
        let tokens = tokenizer.tokenize("Xin chào!").unwrap();
        assert_eq!(tokens.len(), 2);
        assert!(tokens.iter().all(|t| t.kind != TokenKind::Punctuation));
    }

    #[test]
    fn tokenize_multiple_spaces() {
        let tokenizer = SyllableTokenizer::default();
        let tokens = tokenizer.tokenize("Xin    chào").unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].text, "Xin");
        assert_eq!(tokens[1].text, "chào");
    }
}
