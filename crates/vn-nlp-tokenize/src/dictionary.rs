use std::collections::HashSet;
use std::fs;
use std::path::Path;

use vn_nlp_core::{Span, Token, TokenKind, Tokenizer, VnNlpError};

/// Dictionary-based tokenizer sử dụng longest-match.
///
/// Tách từ ghép tiếng Việt bằng cách tìm từ dài nhất khớp trong từ điển.
///
/// # Examples
/// ```no_run
/// use vn_nlp_tokenize::DictionaryTokenizer;
/// use vn_nlp_core::Tokenizer;
///
/// let tokenizer = DictionaryTokenizer::from_file("data/dictionaries/vn-words.txt").unwrap();
/// let tokens = tokenizer.tokenize("Thành phố Hồ Chí Minh").unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct DictionaryTokenizer {
    words: HashSet<String>,
    max_word_len: usize,
}

impl DictionaryTokenizer {
    /// Tạo tokenizer từ danh sách từ.
    pub fn new(words: Vec<String>) -> Self {
        let max_word_len = words
            .iter()
            .map(|w| w.split_whitespace().count())
            .max()
            .unwrap_or(1);
        let words: HashSet<String> = words.into_iter().map(|w| w.to_lowercase()).collect();
        Self {
            words,
            max_word_len,
        }
    }

    /// Load dictionary từ file (mỗi từ/cụm từ trên một dòng).
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, VnNlpError> {
        let path = path.as_ref();
        let content = fs::read_to_string(path).map_err(|_| VnNlpError::DictionaryNotFound {
            path: path.display().to_string(),
        })?;
        let words: Vec<String> = content
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .collect();
        Ok(Self::new(words))
    }

    /// Kiểm tra cụm từ có trong dictionary không.
    fn lookup(&self, word: &str) -> bool {
        self.words.contains(&word.to_lowercase())
    }
}

impl Tokenizer for DictionaryTokenizer {
    type Error = VnNlpError;

    fn tokenize<'a>(&self, input: &'a str) -> Result<Vec<Token<'a>>, Self::Error> {
        let mut tokens = Vec::new();

        // Split thành syllables trước
        let syllables: Vec<(usize, &str)> = extract_syllables(input);

        if syllables.is_empty() {
            return Ok(tokens);
        }

        let mut i = 0;
        while i < syllables.len() {
            let mut best_match_end = i + 1;

            // Thử longest match: từ max_word_len syllables xuống 1
            let max_len = self.max_word_len.min(syllables.len() - i);
            for len in (1..=max_len).rev() {
                let candidate: String = syllables[i..i + len]
                    .iter()
                    .map(|(_, s)| *s)
                    .collect::<Vec<_>>()
                    .join(" ");

                if self.lookup(&candidate) {
                    best_match_end = i + len;
                    break;
                }
            }

            // Tạo token từ matched syllables
            let start_byte = syllables[i].0;
            let last = &syllables[best_match_end - 1];
            let end_byte = last.0 + last.1.len();
            let text = &input[start_byte..end_byte];

            tokens.push(Token {
                text,
                span: Span::new(start_byte, end_byte),
                kind: classify_text(text),
            });

            i = best_match_end;
        }

        Ok(tokens)
    }
}

/// Trích xuất các âm tiết (syllable) với byte offset.
fn extract_syllables(input: &str) -> Vec<(usize, &str)> {
    let mut syllables = Vec::new();
    let mut offset = 0;

    while offset < input.len() {
        let remaining = &input[offset..];

        // Skip whitespace
        let ws_len: usize = remaining
            .chars()
            .take_while(|c| c.is_whitespace())
            .map(|c| c.len_utf8())
            .sum();
        offset += ws_len;

        if offset >= input.len() {
            break;
        }

        let remaining = &input[offset..];
        let word_len: usize = remaining
            .chars()
            .take_while(|c| !c.is_whitespace())
            .map(|c| c.len_utf8())
            .sum();

        if word_len > 0 {
            syllables.push((offset, &input[offset..offset + word_len]));
            offset += word_len;
        }
    }

    syllables
}

/// Phân loại text đơn giản.
fn classify_text(text: &str) -> TokenKind {
    let first = match text.chars().next() {
        Some(c) => c,
        None => return TokenKind::Unknown,
    };

    if first.is_ascii_digit()
        && text
            .chars()
            .all(|c| c.is_ascii_digit() || c == '.' || c == ',')
    {
        TokenKind::Number
    } else if text
        .chars()
        .all(|c| c.is_alphabetic() || c.is_whitespace() || c == '-')
    {
        TokenKind::Word
    } else {
        TokenKind::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_tokenizer() -> DictionaryTokenizer {
        DictionaryTokenizer::new(vec![
            "xin chào".to_string(),
            "việt nam".to_string(),
            "thành phố".to_string(),
            "hồ chí minh".to_string(),
            "thành phố hồ chí minh".to_string(),
            "đà nẵng".to_string(),
            "hà nội".to_string(),
        ])
    }

    #[test]
    fn dictionary_longest_match() {
        let tokenizer = make_tokenizer();
        let tokens = tokenizer.tokenize("thành phố hồ chí minh").unwrap();
        // Should match "thành phố hồ chí minh" as one token (longest match)
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].text, "thành phố hồ chí minh");
    }

    #[test]
    fn dictionary_multiple_words() {
        let tokenizer = make_tokenizer();
        let tokens = tokenizer.tokenize("xin chào việt nam").unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].text, "xin chào");
        assert_eq!(tokens[1].text, "việt nam");
    }

    #[test]
    fn dictionary_fallback_single_syllable() {
        let tokenizer = make_tokenizer();
        let tokens = tokenizer.tokenize("tôi yêu việt nam").unwrap();
        // "tôi" and "yêu" not in dict → single syllable tokens
        // "việt nam" in dict → compound token
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].text, "tôi");
        assert_eq!(tokens[1].text, "yêu");
        assert_eq!(tokens[2].text, "việt nam");
    }

    #[test]
    fn dictionary_empty_input() {
        let tokenizer = make_tokenizer();
        let tokens = tokenizer.tokenize("").unwrap();
        assert!(tokens.is_empty());
    }

    #[test]
    fn dictionary_span_correctness() {
        let input = "xin chào việt nam";
        let tokenizer = make_tokenizer();
        let tokens = tokenizer.tokenize(input).unwrap();
        for token in &tokens {
            assert_eq!(token.text, &input[token.span.start..token.span.end]);
        }
    }

    #[test]
    fn dictionary_from_file() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../data/dictionaries/vn-words.txt"
        );
        if Path::new(path).exists() {
            let tokenizer = DictionaryTokenizer::from_file(path).unwrap();
            let tokens = tokenizer.tokenize("việt nam").unwrap();
            assert!(!tokens.is_empty());
        }
    }
}
