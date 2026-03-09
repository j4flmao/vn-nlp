use thiserror::Error;

/// Lỗi chung cho vn-nlp.
#[derive(Debug, Error)]
pub enum VnNlpError {
    /// Input UTF-8 không hợp lệ.
    #[error("Invalid UTF-8 input at byte offset {offset}")]
    InvalidUtf8 {
        /// Byte offset vị trí lỗi.
        offset: usize,
    },

    /// Không tìm thấy file dictionary.
    #[error("Dictionary not found: {path}")]
    DictionaryNotFound {
        /// Đường dẫn dictionary.
        path: String,
    },

    /// Input quá lớn.
    #[error("Input too large: {size} bytes (max: {max})")]
    InputTooLarge {
        /// Kích thước input.
        size: usize,
        /// Kích thước tối đa cho phép.
        max: usize,
    },

    /// Lỗi I/O.
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
