/// Bỏ dấu tiếng Việt → ASCII.
///
/// # Examples
/// ```
/// use vn_nlp_normalize::strip_diacritics;
///
/// assert_eq!(strip_diacritics("Tiếng Việt"), "Tieng Viet");
/// assert_eq!(strip_diacritics("đường"), "duong");
/// ```
pub fn strip_diacritics(input: &str) -> String {
    input.chars().map(map_diacritics).collect()
}

fn map_diacritics(c: char) -> char {
    match c {
        'à' | 'á' | 'ả' | 'ã' | 'ạ' => 'a',
        'ă' | 'ằ' | 'ắ' | 'ẳ' | 'ẵ' | 'ặ' => 'a',
        'â' | 'ầ' | 'ấ' | 'ẩ' | 'ẫ' | 'ậ' => 'a',
        'è' | 'é' | 'ẻ' | 'ẽ' | 'ẹ' => 'e',
        'ê' | 'ề' | 'ế' | 'ể' | 'ễ' | 'ệ' => 'e',
        'ì' | 'í' | 'ỉ' | 'ĩ' | 'ị' => 'i',
        'ò' | 'ó' | 'ỏ' | 'õ' | 'ọ' => 'o',
        'ô' | 'ồ' | 'ố' | 'ổ' | 'ỗ' | 'ộ' => 'o',
        'ơ' | 'ờ' | 'ớ' | 'ở' | 'ỡ' | 'ợ' => 'o',
        'ù' | 'ú' | 'ủ' | 'ũ' | 'ụ' => 'u',
        'ư' | 'ừ' | 'ứ' | 'ử' | 'ữ' | 'ự' => 'u',
        'ỳ' | 'ý' | 'ỷ' | 'ỹ' | 'ỵ' => 'y',
        'đ' => 'd',
        'À' | 'Á' | 'Ả' | 'Ã' | 'Ạ' => 'A',
        'Ă' | 'Ằ' | 'Ắ' | 'Ẳ' | 'Ẵ' | 'Ặ' => 'A',
        'Â' | 'Ầ' | 'Ấ' | 'Ẩ' | 'Ẫ' | 'Ậ' => 'A',
        'È' | 'É' | 'Ẻ' | 'Ẽ' | 'Ẹ' => 'E',
        'Ê' | 'Ề' | 'Ế' | 'Ể' | 'Ễ' | 'Ệ' => 'E',
        'Ì' | 'Í' | 'Ỉ' | 'Ĩ' | 'Ị' => 'I',
        'Ò' | 'Ó' | 'Ỏ' | 'Õ' | 'Ọ' => 'O',
        'Ô' | 'Ồ' | 'Ố' | 'Ổ' | 'Ỗ' | 'Ộ' => 'O',
        'Ơ' | 'Ờ' | 'Ớ' | 'Ở' | 'Ỡ' | 'Ợ' => 'O',
        'Ù' | 'Ú' | 'Ủ' | 'Ũ' | 'Ụ' => 'U',
        'Ư' | 'Ừ' | 'Ứ' | 'Ử' | 'Ữ' | 'Ự' => 'U',
        'Ỳ' | 'Ý' | 'Ỷ' | 'Ỹ' | 'Ỵ' => 'Y',
        'Đ' => 'D',
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_basic() {
        assert_eq!(strip_diacritics("Tiếng Việt"), "Tieng Viet");
    }

    #[test]
    fn strip_d_bar() {
        assert_eq!(strip_diacritics("đường"), "duong");
        assert_eq!(strip_diacritics("Đà Nẵng"), "Da Nang");
    }

    #[test]
    fn strip_all_tones() {
        assert_eq!(strip_diacritics("àáảãạ"), "aaaaa");
        assert_eq!(strip_diacritics("ằắẳẵặ"), "aaaaa");
        assert_eq!(strip_diacritics("ầấẩẫậ"), "aaaaa");
    }

    #[test]
    fn strip_uppercase() {
        assert_eq!(strip_diacritics("ẮẰẲẴẶ"), "AAAAA");
    }

    #[test]
    fn strip_no_change() {
        assert_eq!(strip_diacritics("Hello World"), "Hello World");
    }

    #[test]
    fn strip_empty() {
        assert_eq!(strip_diacritics(""), "");
    }

    #[test]
    fn strip_full_sentence() {
        assert_eq!(
            strip_diacritics("Xin chào, tôi là người Việt Nam!"),
            "Xin chao, toi la nguoi Viet Nam!"
        );
    }
}
