use vn_nlp::normalize;

// ============================================================
// normalize() — whitespace + NFC
// ============================================================

#[test]
fn normalize_collapses_whitespace() {
    assert_eq!(normalize("  a   b   c  "), "a b c");
}

#[test]
fn normalize_trims_leading_trailing() {
    assert_eq!(normalize("   hello   "), "hello");
}

#[test]
fn normalize_tabs_newlines() {
    assert_eq!(normalize("a\t\tb\n\nc"), "a b c");
}

#[test]
fn normalize_empty() {
    assert_eq!(normalize(""), "");
}

#[test]
fn normalize_single_word() {
    assert_eq!(normalize("hello"), "hello");
}

#[test]
fn normalize_already_clean() {
    assert_eq!(normalize("Xin chào Việt Nam"), "Xin chào Việt Nam");
}

#[test]
fn normalize_preserves_vietnamese_diacritics() {
    let input = "Tiếng Việt rất đẹp và phong phú";
    assert_eq!(normalize(input), input);
}

// ============================================================
// strip_diacritics
// ============================================================

#[test]
fn strip_all_lowercase_vowels() {
    let input = "àáảãạăằắẳẵặâầấẩẫậèéẻẽẹêềếểễệìíỉĩịòóỏõọôồốổỗộơờớởỡợùúủũụưừứửữựỳýỷỹỵ";
    let result = vn_nlp::normalize::strip_diacritics(input);
    // All should be basic ASCII vowels
    assert!(result.chars().all(|c| "aeiouy".contains(c)));
}

#[test]
fn strip_all_uppercase_vowels() {
    let input = "ÀÁẢÃẠĂẰẮẲẴẶÂẦẤẨẪẬÈÉẺẼẸÊỀẾỂỄỆÌÍỈĨỊÒÓỎÕỌÔỒỐỔỖỘƠỜỚỞỠỢÙÚỦŨỤƯỪỨỬỮỰỲÝỶỸỴ";
    let result = vn_nlp::normalize::strip_diacritics(input);
    assert!(result.chars().all(|c| "AEIOUY".contains(c)));
}

#[test]
fn strip_d_bar() {
    assert_eq!(vn_nlp::normalize::strip_diacritics("đ"), "d");
    assert_eq!(vn_nlp::normalize::strip_diacritics("Đ"), "D");
}

#[test]
fn strip_full_sentence() {
    assert_eq!(
        vn_nlp::normalize::strip_diacritics("Xin chào, tôi là người Việt Nam!"),
        "Xin chao, toi la nguoi Viet Nam!"
    );
}

#[test]
fn strip_preserves_ascii() {
    assert_eq!(
        vn_nlp::normalize::strip_diacritics("Hello World 123!"),
        "Hello World 123!"
    );
}

#[test]
fn strip_empty() {
    assert_eq!(vn_nlp::normalize::strip_diacritics(""), "");
}

#[test]
fn strip_real_address() {
    assert_eq!(
        vn_nlp::normalize::strip_diacritics("123 Nguyễn Huệ, Quận 1, TP.HCM"),
        "123 Nguyen Hue, Quan 1, TP.HCM"
    );
}

// ============================================================
// lowercase_vn
// ============================================================

#[test]
fn lowercase_basic() {
    assert_eq!(vn_nlp::normalize::lowercase_vn("XIN CHÀO"), "xin chào");
}

#[test]
fn lowercase_d_bar() {
    assert_eq!(vn_nlp::normalize::lowercase_vn("ĐÀ NẴNG"), "đà nẵng");
}

#[test]
fn lowercase_mixed_case() {
    assert_eq!(vn_nlp::normalize::lowercase_vn("Tiếng Việt"), "tiếng việt");
}

#[test]
fn lowercase_already_lower() {
    assert_eq!(
        vn_nlp::normalize::lowercase_vn("đã lowercase"),
        "đã lowercase"
    );
}

#[test]
fn lowercase_with_numbers() {
    assert_eq!(vn_nlp::normalize::lowercase_vn("QUẬN 1"), "quận 1");
}

// ============================================================
// NFC / NFD
// ============================================================

#[test]
fn nfc_nfd_roundtrip() {
    let text = "Việt Nam";
    let nfc = vn_nlp::normalize::to_nfc(text);
    let nfd = vn_nlp::normalize::to_nfd(text);
    // NFC of NFC == NFC, NFC of NFD == NFC
    assert_eq!(
        vn_nlp::normalize::to_nfc(&nfc),
        vn_nlp::normalize::to_nfc(&nfd)
    );
}

#[test]
fn nfc_idempotent() {
    let text = "ồ ứ ế ắ";
    let once = vn_nlp::normalize::to_nfc(text);
    let twice = vn_nlp::normalize::to_nfc(&once);
    assert_eq!(once, twice);
}

#[test]
fn nfd_idempotent() {
    let text = "ồ ứ ế ắ";
    let once = vn_nlp::normalize::to_nfd(text);
    let twice = vn_nlp::normalize::to_nfd(&once);
    assert_eq!(once, twice);
}

#[test]
fn nfc_preserves_ascii() {
    assert_eq!(vn_nlp::normalize::to_nfc("Hello"), "Hello");
}

// ============================================================
// Number normalization
// ============================================================

#[test]
fn number_k() {
    assert_eq!(vn_nlp::normalize::number::normalize_number("1k"), "1000");
    assert_eq!(vn_nlp::normalize::number::normalize_number("5k"), "5000");
    assert_eq!(vn_nlp::normalize::number::normalize_number("1.5k"), "1500");
}

#[test]
fn number_tr() {
    assert_eq!(
        vn_nlp::normalize::number::normalize_number("10tr"),
        "10000000"
    );
    assert_eq!(
        vn_nlp::normalize::number::normalize_number("2.5tr"),
        "2500000"
    );
}

#[test]
fn number_ty() {
    assert_eq!(
        vn_nlp::normalize::number::normalize_number("1tỷ"),
        "1000000000"
    );
}

#[test]
fn number_no_suffix() {
    assert_eq!(vn_nlp::normalize::number::normalize_number("123"), "123");
}

#[test]
fn number_not_a_number() {
    assert_eq!(
        vn_nlp::normalize::number::normalize_number("hello"),
        "hello"
    );
}

#[test]
fn number_empty() {
    assert_eq!(vn_nlp::normalize::number::normalize_number(""), "");
}

// ============================================================
// Abbreviation expansion
// ============================================================

#[test]
fn expand_known() {
    assert_eq!(
        vn_nlp::normalize::expand_abbreviation("TP.HCM"),
        Some("Thành phố Hồ Chí Minh")
    );
    assert_eq!(vn_nlp::normalize::expand_abbreviation("HN"), Some("Hà Nội"));
    assert_eq!(
        vn_nlp::normalize::expand_abbreviation("VN"),
        Some("Việt Nam")
    );
    assert_eq!(
        vn_nlp::normalize::expand_abbreviation("GS."),
        Some("Giáo sư")
    );
    assert_eq!(
        vn_nlp::normalize::expand_abbreviation("TS."),
        Some("Tiến sĩ")
    );
    assert_eq!(
        vn_nlp::normalize::expand_abbreviation("BS."),
        Some("Bác sĩ")
    );
}

#[test]
fn expand_unknown() {
    assert_eq!(vn_nlp::normalize::expand_abbreviation("XYZ"), None);
    assert_eq!(vn_nlp::normalize::expand_abbreviation(""), None);
}

// ============================================================
// Full pipeline
// ============================================================

#[test]
fn full_pipeline_normalize_lowercase_strip() {
    let raw = "  XIN   CHÀO   VIỆT   NAM  ";
    let normalized = normalize(raw);
    assert_eq!(normalized, "XIN CHÀO VIỆT NAM");
    let lowered = vn_nlp::normalize::lowercase_vn(&normalized);
    assert_eq!(lowered, "xin chào việt nam");
    let stripped = vn_nlp::normalize::strip_diacritics(&lowered);
    assert_eq!(stripped, "xin chao viet nam");
}

#[test]
fn pipeline_nfd_to_nfc_then_strip() {
    let nfd = vn_nlp::normalize::to_nfd("Tiếng Việt");
    let nfc = vn_nlp::normalize::to_nfc(&nfd);
    let stripped = vn_nlp::normalize::strip_diacritics(&nfc);
    assert_eq!(stripped, "Tieng Viet");
}
