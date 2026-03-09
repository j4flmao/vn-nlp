#![forbid(unsafe_code)]
#![doc = "Vietnamese tokenization algorithms."]

#[cfg(feature = "dictionary")]
pub mod dictionary;
pub mod stream;
pub mod syllable;
mod unicode;

#[cfg(feature = "dictionary")]
pub use dictionary::DictionaryTokenizer;
pub use stream::TokenStream;
pub use syllable::SyllableTokenizer;

use vn_nlp_core::{Token, VnNlpError};

/// Tách từ tiếng Việt theo âm tiết (convenience function).
///
/// # Arguments
/// * `input` - Văn bản tiếng Việt UTF-8
///
/// # Returns
/// Vector các `Token` với span position trong input gốc.
///
/// # Errors
/// Returns `VnNlpError` nếu xảy ra lỗi xử lý.
///
/// # Examples
/// ```
/// use vn_nlp_tokenize::tokenize;
///
/// let tokens = tokenize("Xin chào Việt Nam").unwrap();
/// assert_eq!(tokens[0].text, "Xin");
/// assert_eq!(tokens[1].text, "chào");
/// ```
pub fn tokenize(input: &str) -> Result<Vec<Token<'_>>, VnNlpError> {
    let tokenizer = SyllableTokenizer::default();
    use vn_nlp_core::Tokenizer;
    tokenizer.tokenize(input)
}
