#![forbid(unsafe_code)]
#![doc = "Vietnamese text normalization — diacritics stripping, Unicode NFC/NFD."]

pub mod diacritics;
pub mod number;
pub mod unicode_norm;

pub use diacritics::strip_diacritics;
pub use unicode_norm::{to_nfc, to_nfd};

/// Lowercase tiếng Việt — giữ đúng dấu.
///
/// # Examples
/// ```
/// use vn_nlp_normalize::lowercase_vn;
///
/// assert_eq!(lowercase_vn("XIN CHÀO VIỆT NAM"), "xin chào việt nam");
/// assert_eq!(lowercase_vn("Đà Nẵng"), "đà nẵng");
/// ```
pub fn lowercase_vn(input: &str) -> String {
    input.chars().flat_map(|c| c.to_lowercase()).collect()
}

/// Chuẩn hóa văn bản tiếng Việt: NFC + collapse whitespace.
///
/// # Examples
/// ```
/// use vn_nlp_normalize::normalize;
///
/// let result = normalize("  Xin   chào   Việt   Nam  ");
/// assert_eq!(result, "Xin chào Việt Nam");
/// ```
pub fn normalize(input: &str) -> String {
    let nfc = to_nfc(input);
    collapse_whitespace(&nfc)
}

/// Mở rộng abbreviation phổ biến tiếng Việt.
///
/// # Examples
/// ```
/// use vn_nlp_normalize::expand_abbreviation;
///
/// assert_eq!(expand_abbreviation("TP.HCM"), Some("Thành phố Hồ Chí Minh"));
/// assert_eq!(expand_abbreviation("HN"), Some("Hà Nội"));
/// assert_eq!(expand_abbreviation("hello"), None);
/// ```
pub fn expand_abbreviation(input: &str) -> Option<&'static str> {
    match input {
        "TP.HCM" | "TPHCM" => Some("Thành phố Hồ Chí Minh"),
        "HN" => Some("Hà Nội"),
        "ĐN" => Some("Đà Nẵng"),
        "HP" => Some("Hải Phòng"),
        "CT" => Some("Cần Thơ"),
        "SG" => Some("Sài Gòn"),
        "VN" => Some("Việt Nam"),
        "TP." => Some("Thành phố"),
        "GS." => Some("Giáo sư"),
        "PGS." => Some("Phó Giáo sư"),
        "TS." => Some("Tiến sĩ"),
        "ThS." => Some("Thạc sĩ"),
        "CN." => Some("Cử nhân"),
        "BS." => Some("Bác sĩ"),
        "KS." => Some("Kỹ sư"),
        "TT." => Some("Thủ tướng"),
        "v.v." => Some("vân vân"),
        "v.d." => Some("ví dụ"),
        _ => None,
    }
}

/// Thu gọn nhiều whitespace liên tiếp thành 1 space, trim đầu cuối.
fn collapse_whitespace(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut prev_ws = true; // Trim leading

    for c in input.chars() {
        if c.is_whitespace() {
            if !prev_ws {
                result.push(' ');
                prev_ws = true;
            }
        } else {
            result.push(c);
            prev_ws = false;
        }
    }

    // Trim trailing space
    if result.ends_with(' ') {
        result.pop();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_collapses_whitespace() {
        assert_eq!(normalize("  a   b   c  "), "a b c");
    }

    #[test]
    fn normalize_empty() {
        assert_eq!(normalize(""), "");
    }

    #[test]
    fn normalize_single_word() {
        assert_eq!(normalize("  hello  "), "hello");
    }

    #[test]
    fn lowercase_vn_basic() {
        assert_eq!(lowercase_vn("XIN CHÀO"), "xin chào");
    }

    #[test]
    fn lowercase_vn_with_diacritics() {
        assert_eq!(lowercase_vn("ĐÀ NẴNG"), "đà nẵng");
        assert_eq!(lowercase_vn("TIẾNG VIỆT"), "tiếng việt");
    }

    #[test]
    fn lowercase_vn_mixed() {
        assert_eq!(lowercase_vn("Hello Việt Nam"), "hello việt nam");
    }

    #[test]
    fn lowercase_vn_already_lower() {
        assert_eq!(lowercase_vn("xin chào"), "xin chào");
    }

    #[test]
    fn lowercase_vn_empty() {
        assert_eq!(lowercase_vn(""), "");
    }

    #[test]
    fn expand_abbreviation_known() {
        assert_eq!(expand_abbreviation("TP.HCM"), Some("Thành phố Hồ Chí Minh"));
        assert_eq!(expand_abbreviation("HN"), Some("Hà Nội"));
        assert_eq!(expand_abbreviation("GS."), Some("Giáo sư"));
    }

    #[test]
    fn expand_abbreviation_unknown() {
        assert_eq!(expand_abbreviation("XYZ"), None);
        assert_eq!(expand_abbreviation("hello"), None);
    }
}
