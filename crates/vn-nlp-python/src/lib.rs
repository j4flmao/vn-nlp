use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use vn_nlp_core::Tokenizer;

/// Token result cho Python.
#[pyclass]
#[derive(Clone)]
struct Token {
    #[pyo3(get)]
    text: String,
    #[pyo3(get)]
    kind: String,
    #[pyo3(get)]
    start: usize,
    #[pyo3(get)]
    end: usize,
}

#[pymethods]
impl Token {
    fn __repr__(&self) -> String {
        format!(
            "Token(text='{}', kind='{}', start={}, end={})",
            self.text, self.kind, self.start, self.end
        )
    }

    fn __str__(&self) -> String {
        self.text.clone()
    }
}

/// Sentence result cho Python.
#[pyclass]
#[derive(Clone)]
struct Sentence {
    #[pyo3(get)]
    text: String,
    #[pyo3(get)]
    start: usize,
    #[pyo3(get)]
    end: usize,
}

#[pymethods]
impl Sentence {
    fn __repr__(&self) -> String {
        format!(
            "Sentence(text='{}', start={}, end={})",
            self.text, self.start, self.end
        )
    }

    fn __str__(&self) -> String {
        self.text.clone()
    }
}

/// Tách từ tiếng Việt theo âm tiết.
///
/// Args:
///     text: Văn bản tiếng Việt UTF-8.
///
/// Returns:
///     List[Token]: Danh sách token.
///
/// Example:
///     >>> import vn_nlp
///     >>> tokens = vn_nlp.tokenize("Xin chào Việt Nam!")
///     >>> [t.text for t in tokens]
///     ['Xin', 'chào', 'Việt', 'Nam', '!']
#[pyfunction]
fn tokenize(text: &str) -> PyResult<Vec<Token>> {
    let tokenizer = vn_nlp_tokenize::SyllableTokenizer::default();
    let tokens = tokenizer
        .tokenize(text)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;

    Ok(tokens
        .iter()
        .map(|t| Token {
            text: t.text.to_string(),
            kind: format!("{:?}", t.kind),
            start: t.span.start,
            end: t.span.end,
        })
        .collect())
}

/// Chuẩn hóa văn bản tiếng Việt (NFC + collapse whitespace).
///
/// Example:
///     >>> vn_nlp.normalize("  Xin   chào  ")
///     'Xin chào'
#[pyfunction]
fn normalize(text: &str) -> String {
    vn_nlp_normalize::normalize(text)
}

/// Bỏ dấu tiếng Việt → ASCII.
///
/// Example:
///     >>> vn_nlp.strip_diacritics("Tiếng Việt")
///     'Tieng Viet'
#[pyfunction]
fn strip_diacritics(text: &str) -> String {
    vn_nlp_normalize::strip_diacritics(text)
}

/// Lowercase tiếng Việt (giữ đúng dấu).
///
/// Example:
///     >>> vn_nlp.lowercase("ĐÀ NẴNG")
///     'đà nẵng'
#[pyfunction]
fn lowercase(text: &str) -> String {
    vn_nlp_normalize::lowercase_vn(text)
}

/// NFC normalization.
#[pyfunction]
fn to_nfc(text: &str) -> String {
    vn_nlp_normalize::to_nfc(text)
}

/// NFD normalization.
#[pyfunction]
fn to_nfd(text: &str) -> String {
    vn_nlp_normalize::to_nfd(text)
}

/// Chia văn bản thành câu.
///
/// Example:
///     >>> vn_nlp.segment("Hôm nay trời đẹp. Tôi đi chơi!")
///     [Sentence(text='Hôm nay trời đẹp.', ...), Sentence(text='Tôi đi chơi!', ...)]
#[pyfunction]
fn segment(text: &str) -> PyResult<Vec<Sentence>> {
    use vn_nlp_core::Segmenter;
    let segmenter = vn_nlp_segment::RuleSegmenter;
    let sentences = segmenter
        .segment(text)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;

    Ok(sentences
        .iter()
        .map(|s| Sentence {
            text: s.text.trim().to_string(),
            start: s.span.start,
            end: s.span.end,
        })
        .collect())
}

/// Mở rộng abbreviation phổ biến.
///
/// Example:
///     >>> vn_nlp.expand_abbreviation("TP.HCM")
///     'Thành phố Hồ Chí Minh'
#[pyfunction]
fn expand_abbreviation(text: &str) -> Option<String> {
    vn_nlp_normalize::expand_abbreviation(text).map(|s| s.to_string())
}

/// Normalize số viết tắt.
///
/// Example:
///     >>> vn_nlp.normalize_number("10tr")
///     '10000000'
#[pyfunction]
fn normalize_number(text: &str) -> String {
    vn_nlp_normalize::number::normalize_number(text)
}

/// Vietnamese NLP module — tokenization, normalization, segmentation.
#[pymodule]
fn vn_nlp(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(tokenize, m)?)?;
    m.add_function(wrap_pyfunction!(normalize, m)?)?;
    m.add_function(wrap_pyfunction!(strip_diacritics, m)?)?;
    m.add_function(wrap_pyfunction!(lowercase, m)?)?;
    m.add_function(wrap_pyfunction!(to_nfc, m)?)?;
    m.add_function(wrap_pyfunction!(to_nfd, m)?)?;
    m.add_function(wrap_pyfunction!(segment, m)?)?;
    m.add_function(wrap_pyfunction!(expand_abbreviation, m)?)?;
    m.add_function(wrap_pyfunction!(normalize_number, m)?)?;
    m.add_class::<Token>()?;
    m.add_class::<Sentence>()?;
    Ok(())
}
