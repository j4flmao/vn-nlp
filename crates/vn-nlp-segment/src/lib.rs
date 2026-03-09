#![forbid(unsafe_code)]
#![doc = "Vietnamese sentence segmentation."]

pub mod rules;

pub use rules::RuleSegmenter;

use vn_nlp_core::{Sentence, VnNlpError};

/// Chia văn bản thành danh sách câu (convenience function).
///
/// # Examples
/// ```
/// use vn_nlp_segment::segment;
///
/// let sentences = segment("Hôm nay trời đẹp. Tôi đi chơi!").unwrap();
/// assert_eq!(sentences.len(), 2);
/// ```
pub fn segment(input: &str) -> Result<Vec<Sentence<'_>>, VnNlpError> {
    let segmenter = RuleSegmenter;
    use vn_nlp_core::Segmenter;
    segmenter.segment(input)
}
