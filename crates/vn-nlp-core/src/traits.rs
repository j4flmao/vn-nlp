use crate::types::{Sentence, Token};

/// Trait cho các thuật toán tokenization.
pub trait Tokenizer {
    /// Loại lỗi trả về.
    type Error;

    /// Tách input thành danh sách token.
    fn tokenize<'a>(&self, input: &'a str) -> Result<Vec<Token<'a>>, Self::Error>;
}

/// Trait cho các thuật toán normalization.
pub trait Normalizer {
    /// Chuẩn hóa input và trả về string mới.
    fn normalize(&self, input: &str) -> String;

    /// Chuẩn hóa in-place.
    fn normalize_in_place(&self, input: &mut String);
}

/// Trait cho các thuật toán sentence segmentation.
pub trait Segmenter {
    /// Loại lỗi trả về.
    type Error;

    /// Chia input thành danh sách câu.
    fn segment<'a>(&self, input: &'a str) -> Result<Vec<Sentence<'a>>, Self::Error>;
}
