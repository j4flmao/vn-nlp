#![forbid(unsafe_code)]
#![doc = "Vietnamese NLP library — tokenization, normalization, segmentation."]
#![doc = ""]
#![doc = "# Quick Start"]
#![doc = ""]
#![doc = "```rust"]
#![doc = "use vn_nlp::tokenize;"]
#![doc = ""]
#![doc = "let tokens = tokenize(\"Xin chào Việt Nam\").unwrap();"]
#![doc = "assert_eq!(tokens[0].text, \"Xin\");"]
#![doc = "```"]

// Re-export core types
pub use vn_nlp_core::*;

// Re-export tokenize module
#[cfg(feature = "tokenize")]
pub mod tokenize {
    //! Tokenization algorithms cho tiếng Việt.
    pub use vn_nlp_tokenize::*;
}

// Re-export normalize module
#[cfg(feature = "normalize")]
pub mod normalize {
    //! Text normalization — diacritics, Unicode NFC/NFD.
    pub use vn_nlp_normalize::*;
}

// Re-export segment module
#[cfg(feature = "segment")]
pub mod segment {
    //! Sentence segmentation.
    pub use vn_nlp_segment::*;
}

// Convenience re-exports at top level
#[cfg(feature = "tokenize")]
pub use vn_nlp_tokenize::tokenize;

#[cfg(feature = "normalize")]
pub use vn_nlp_normalize::normalize;

#[cfg(feature = "segment")]
pub use vn_nlp_segment::segment;
