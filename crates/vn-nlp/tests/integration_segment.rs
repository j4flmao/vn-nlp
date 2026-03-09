use vn_nlp::segment;

// ============================================================
// Basic segmentation
// ============================================================

#[test]
fn segment_period() {
    let sentences = segment("Hôm nay trời đẹp. Tôi đi chơi.").unwrap();
    assert_eq!(sentences.len(), 2);
}

#[test]
fn segment_exclamation() {
    let sentences = segment("Tuyệt vời! Tôi rất vui!").unwrap();
    assert_eq!(sentences.len(), 2);
}

#[test]
fn segment_question() {
    let sentences = segment("Bạn khỏe không? Tôi khỏe.").unwrap();
    assert_eq!(sentences.len(), 2);
}

#[test]
fn segment_mixed_punctuation() {
    let sentences = segment("Xin chào! Bạn khỏe không? Tôi khỏe.").unwrap();
    assert_eq!(sentences.len(), 3);
}

// ============================================================
// Abbreviations — should NOT split
// ============================================================

#[test]
fn segment_tp_abbreviation() {
    let sentences = segment("TP. HCM có mưa lớn.").unwrap();
    assert_eq!(sentences.len(), 1);
}

#[test]
fn segment_gs_abbreviation() {
    let sentences = segment("GS. Nguyễn Văn A nói.").unwrap();
    assert_eq!(sentences.len(), 1);
}

#[test]
fn segment_ts_abbreviation() {
    let sentences = segment("TS. Trần Văn B trình bày.").unwrap();
    assert_eq!(sentences.len(), 1);
}

#[test]
fn segment_multiple_abbreviations() {
    let sentences = segment("PGS.TS. Lê Văn C dạy ở TP. HCM.").unwrap();
    assert_eq!(sentences.len(), 1);
}

#[test]
fn segment_mr_mrs() {
    let sentences = segment("Mr. Smith gặp Mrs. Jones.").unwrap();
    assert_eq!(sentences.len(), 1);
}

#[test]
fn segment_vv() {
    let sentences = segment("Có nhiều loại: cam, quýt, v.v. rất ngon.").unwrap();
    assert_eq!(sentences.len(), 1);
}

// ============================================================
// Ellipsis
// ============================================================

#[test]
fn segment_ellipsis_as_boundary() {
    let sentences = segment("Tôi nghĩ... rồi tôi đi.").unwrap();
    assert_eq!(sentences.len(), 2);
}

#[test]
fn segment_long_ellipsis() {
    let sentences = segment("Ừm..... để tôi nghĩ.").unwrap();
    assert_eq!(sentences.len(), 2);
}

// ============================================================
// Quoted sentences — should NOT split inside quotes
// ============================================================

#[test]
fn segment_quoted_no_split() {
    let sentences = segment("Anh nói \"Tôi đi. Tôi về.\" rồi đi.").unwrap();
    assert_eq!(sentences.len(), 1);
}

#[test]
fn segment_smart_quotes_no_split() {
    let sentences = segment("Cô ấy bảo \u{201C}Đừng đi!\u{201D} nhưng anh vẫn đi.").unwrap();
    assert_eq!(sentences.len(), 1);
}

#[test]
fn segment_quotes_then_normal() {
    let sentences = segment("\"Xin chào!\" Anh nói. Rồi đi.").unwrap();
    assert_eq!(sentences.len(), 2);
}

// ============================================================
// Edge cases
// ============================================================

#[test]
fn segment_empty() {
    assert!(segment("").unwrap().is_empty());
}

#[test]
fn segment_no_boundary() {
    let sentences = segment("Đây là văn bản không có dấu câu cuối").unwrap();
    assert_eq!(sentences.len(), 1);
}

#[test]
fn segment_only_whitespace() {
    assert!(segment("   ").unwrap().is_empty());
}

#[test]
fn segment_single_word() {
    let sentences = segment("Xin").unwrap();
    assert_eq!(sentences.len(), 1);
}

#[test]
fn segment_single_period() {
    // Just a period with nothing else meaningful
    let sentences = segment(".").unwrap();
    assert_eq!(sentences.len(), 1);
}

#[test]
fn segment_many_sentences() {
    let input = "Câu một. Câu hai. Câu ba. Câu bốn. Câu năm.";
    let sentences = segment(input).unwrap();
    println!("segment_many_sentences => {}", sentences.len());
    for (i, s) in sentences.iter().enumerate() {
        println!("[{}] '{}'", i, s.text);
    }
    assert_eq!(sentences.len(), 5);
}

// ============================================================
// Real-world Vietnamese text
// ============================================================

#[test]
fn segment_real_news() {
    let text = "Theo VnExpress, TP.HCM sẽ triển khai dự án metro số 2. \
                Dự án dự kiến hoàn thành vào năm 2030. \
                Tổng mức đầu tư khoảng 2.1 tỷ USD.";
    let sentences = segment(text).unwrap();
    assert_eq!(sentences.len(), 3);
}

#[test]
fn segment_real_academic() {
    let text = "Theo GS. Nguyễn Văn A, nghiên cứu cho thấy kết quả tích cực. \
                PGS.TS. Trần Văn B cũng đồng ý với nhận định này.";
    let sentences = segment(text).unwrap();
    assert_eq!(sentences.len(), 2);
}

#[test]
fn segment_sentence_text_is_trimmed() {
    let sentences = segment("  Câu một.  Câu hai.  ").unwrap();
    for s in &sentences {
        // text should not have leading whitespace
        assert!(
            !s.text.starts_with(' '),
            "Sentence text has leading space: '{}'",
            s.text
        );
    }
}

// ============================================================
// Span correctness
// ============================================================

#[test]
fn segment_spans_are_valid() {
    let input = "Câu một. Câu hai!";
    let sentences = segment(input).unwrap();
    for s in &sentences {
        assert!(s.span.start <= s.span.end);
        assert!(s.span.end <= input.len());
    }
}
