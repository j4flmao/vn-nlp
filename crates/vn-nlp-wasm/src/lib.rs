use serde::Serialize;
use vn_nlp_core::Tokenizer;
use wasm_bindgen::prelude::*;

/// Token result trả về cho JavaScript.
#[derive(Serialize)]
pub struct JsToken {
    pub text: String,
    pub kind: String,
    pub start: usize,
    pub end: usize,
}

/// Sentence result trả về cho JavaScript.
#[derive(Serialize)]
pub struct JsSentence {
    pub text: String,
    pub start: usize,
    pub end: usize,
}

/// Tách từ tiếng Việt — trả về JSON array of tokens.
///
/// ```js
/// const tokens = tokenize("Xin chào Việt Nam!");
/// // [{ text: "Xin", kind: "Word", start: 0, end: 3 }, ...]
/// ```
#[wasm_bindgen]
pub fn tokenize(input: &str) -> Result<JsValue, JsError> {
    let tokenizer = vn_nlp_tokenize::SyllableTokenizer::default();
    let tokens = tokenizer
        .tokenize(input)
        .map_err(|e| JsError::new(&e.to_string()))?;

    let js_tokens: Vec<JsToken> = tokens
        .iter()
        .map(|t| JsToken {
            text: t.text.to_string(),
            kind: format!("{:?}", t.kind),
            start: t.span.start,
            end: t.span.end,
        })
        .collect();

    serde_wasm_bindgen::to_value(&js_tokens).map_err(|e| JsError::new(&e.to_string()))
}

/// Chuẩn hóa văn bản tiếng Việt (NFC + collapse whitespace).
#[wasm_bindgen]
pub fn normalize(input: &str) -> String {
    vn_nlp_normalize::normalize(input)
}

/// Bỏ dấu tiếng Việt → ASCII.
#[wasm_bindgen]
pub fn strip_diacritics(input: &str) -> String {
    vn_nlp_normalize::strip_diacritics(input)
}

/// Lowercase tiếng Việt (giữ đúng dấu).
#[wasm_bindgen]
pub fn lowercase_vn(input: &str) -> String {
    vn_nlp_normalize::lowercase_vn(input)
}

/// NFC normalization.
#[wasm_bindgen]
pub fn to_nfc(input: &str) -> String {
    vn_nlp_normalize::to_nfc(input)
}

/// NFD normalization.
#[wasm_bindgen]
pub fn to_nfd(input: &str) -> String {
    vn_nlp_normalize::to_nfd(input)
}

/// Chia văn bản thành câu — trả về JSON array.
#[wasm_bindgen]
pub fn segment(input: &str) -> Result<JsValue, JsError> {
    use vn_nlp_core::Segmenter;
    let segmenter = vn_nlp_segment::RuleSegmenter;
    let sentences = segmenter
        .segment(input)
        .map_err(|e| JsError::new(&e.to_string()))?;

    let js_sentences: Vec<JsSentence> = sentences
        .iter()
        .map(|s| JsSentence {
            text: s.text.trim().to_string(),
            start: s.span.start,
            end: s.span.end,
        })
        .collect();

    serde_wasm_bindgen::to_value(&js_sentences).map_err(|e| JsError::new(&e.to_string()))
}

/// Mở rộng abbreviation phổ biến.
#[wasm_bindgen]
pub fn expand_abbreviation(input: &str) -> Option<String> {
    vn_nlp_normalize::expand_abbreviation(input).map(|s| s.to_string())
}
