#![forbid(unsafe_code)]
#![doc = "Core types, traits, and errors for the vn-nlp ecosystem."]

pub mod error;
pub mod traits;
pub mod types;

pub use error::VnNlpError;
pub use traits::{Normalizer, Segmenter, Tokenizer};
pub use types::{Sentence, Span, Token, TokenKind};
