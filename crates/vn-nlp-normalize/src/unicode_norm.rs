use unicode_normalization::UnicodeNormalization;

/// Chuẩn hóa NFC (Canonical Decomposition → Canonical Composition).
///
/// # Examples
/// ```
/// use vn_nlp_normalize::to_nfc;
///
/// let text = "hoà"; // có thể là NFD
/// let nfc = to_nfc(text);
/// // Kết quả ở dạng NFC (precomposed)
/// ```
pub fn to_nfc(input: &str) -> String {
    input.nfc().collect()
}

/// Chuẩn hóa NFD (Canonical Decomposition).
///
/// # Examples
/// ```
/// use vn_nlp_normalize::to_nfd;
///
/// let text = "hoà";
/// let nfd = to_nfd(text);
/// // Kết quả ở dạng NFD (decomposed)
/// ```
pub fn to_nfd(input: &str) -> String {
    input.nfd().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nfc_roundtrip() {
        let original = "hoà bình";
        let nfd = to_nfd(original);
        let nfc = to_nfc(&nfd);
        let nfc2 = to_nfc(&nfc);
        assert_eq!(nfc, nfc2);
    }

    #[test]
    fn nfd_roundtrip() {
        let original = "Việt Nam";
        let nfd = to_nfd(original);
        let nfd2 = to_nfd(&nfd);
        assert_eq!(nfd, nfd2);
    }

    #[test]
    fn nfc_nfd_equivalence() {
        let text = "Tiếng Việt";
        let nfc = to_nfc(text);
        let nfd = to_nfd(text);
        // NFC và NFD phải equal khi so sánh qua NFC
        assert_eq!(to_nfc(&nfc), to_nfc(&nfd));
    }

    #[test]
    fn empty_string() {
        assert_eq!(to_nfc(""), "");
        assert_eq!(to_nfd(""), "");
    }

    #[test]
    fn ascii_unchanged() {
        assert_eq!(to_nfc("Hello"), "Hello");
        assert_eq!(to_nfd("Hello"), "Hello");
    }
}
