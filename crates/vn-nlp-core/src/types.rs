/// Một token sau khi tách.
///
/// Giữ reference `&'a str` tới input gốc (zero-copy).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'a> {
    /// Slice của input gốc (zero-copy).
    pub text: &'a str,
    /// Vị trí trong string gốc (byte offset).
    pub span: Span,
    /// Loại token: Word, Punctuation, Number, Whitespace.
    pub kind: TokenKind,
}

/// Vị trí byte offset trong string gốc.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// Byte offset bắt đầu (inclusive).
    pub start: usize,
    /// Byte offset kết thúc (exclusive).
    pub end: usize,
}

impl Span {
    /// Tạo span mới.
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Độ dài span tính theo bytes.
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Span có rỗng không.
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

/// Phân loại token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    /// Từ tiếng Việt hoặc ASCII.
    Word,
    /// Dấu câu (., !, ?, ;, :, ...).
    Punctuation,
    /// Số (0-9).
    Number,
    /// Khoảng trắng (space, tab, newline).
    Whitespace,
    /// Không xác định.
    Unknown,
}

/// Một câu sau khi segment.
#[derive(Debug, Clone)]
pub struct Sentence<'a> {
    /// Nội dung câu.
    pub text: &'a str,
    /// Vị trí trong string gốc.
    pub span: Span,
    /// Các token trong câu.
    pub tokens: Vec<Token<'a>>,
}
