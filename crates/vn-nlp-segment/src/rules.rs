use vn_nlp_core::{Segmenter, Sentence, Span, VnNlpError};

/// Danh sách abbreviation không nên split.
const ABBREVIATIONS: &[&str] = &[
    "TP.", "GS.", "PGS.", "TS.", "ThS.", "CN.", "BS.", "KS.", "TT.", "Mr.", "Mrs.", "Ms.", "Dr.",
    "Prof.", "v.v.", "v.d.", "tr.", "t.",
];

/// Rule-based sentence segmenter cho tiếng Việt.
///
/// Chia câu theo dấu `.` `!` `?` nhưng bỏ qua abbreviation.
#[derive(Debug, Clone, Default)]
pub struct RuleSegmenter;

impl RuleSegmenter {
    /// Kiểm tra vị trí có phải abbreviation không.
    fn is_abbreviation(&self, text: &str, dot_byte_pos: usize) -> bool {
        // Kiểm tra xem dot hiện tại có nằm trong một abbreviation đã biết không.
        for abbr in ABBREVIATIONS {
            for (start, _) in text.match_indices(abbr) {
                let end = start + abbr.len();
                if dot_byte_pos >= start && dot_byte_pos < end {
                    // Các abbreviation "t." và "tr." chỉ hợp lệ khi đứng sau chữ số,
                    // để tránh match nhầm bên trong từ (vd: "một.").
                    if *abbr == "t." || *abbr == "tr." {
                        let prev_char = if start == 0 {
                            None
                        } else {
                            text[..start].chars().last()
                        };
                        if !prev_char.is_some_and(|c| c.is_ascii_digit()) {
                            continue;
                        }
                    }

                    return true;
                }
            }
        }

        // Kiểm tra single uppercase letter + dot (e.g., "A.", "B.")
        let before_dot = &text[..dot_byte_pos];
        if let Some(last_char) = before_dot.chars().last() {
            if last_char.is_uppercase() {
                let char_before = before_dot.chars().rev().nth(1);
                if char_before.is_none_or(|c| c.is_whitespace() || c == '.') {
                    return true;
                }
            }
        }

        false
    }

    /// Kiểm tra ký tự có phải sentence boundary không.
    fn is_sentence_end(c: char) -> bool {
        matches!(c, '.' | '!' | '?')
    }

    /// Kiểm tra vị trí có nằm trong quotes không.
    fn is_inside_quotes(text: &str, byte_pos: usize) -> bool {
        let before = &text[..byte_pos];
        let double_quotes = before.chars().filter(|c| *c == '"').count();
        let left_smart = before.chars().filter(|c| *c == '\u{201C}').count(); // "
        let right_smart = before.chars().filter(|c| *c == '\u{201D}').count(); // "

        // Odd number of regular quotes means we're inside
        if double_quotes % 2 != 0 {
            return true;
        }
        // More left smart quotes than right means we're inside
        if left_smart > right_smart {
            return true;
        }
        false
    }
}

impl Segmenter for RuleSegmenter {
    type Error = VnNlpError;

    fn segment<'a>(&self, input: &'a str) -> Result<Vec<Sentence<'a>>, Self::Error> {
        let mut sentences = Vec::new();
        let mut start = 0;

        let chars: Vec<(usize, char)> = input.char_indices().collect();
        let mut i = 0;

        while i < chars.len() {
            let (byte_pos, c) = chars[i];

            if Self::is_sentence_end(c) {
                // Bỏ qua nếu nằm trong quotes
                if Self::is_inside_quotes(input, byte_pos) {
                    i += 1;
                    continue;
                }

                // Xử lý ellipsis (...)
                if c == '.' {
                    let dot_count = chars[i..].iter().take_while(|(_, ch)| *ch == '.').count();

                    if dot_count >= 3 {
                        // Ellipsis — coi như sentence boundary
                        let end = chars[i + dot_count - 1].0 + 1;
                        let text = input[start..end].trim();
                        if !text.is_empty() {
                            sentences.push(Sentence {
                                text: input[start..end].trim_start(),
                                span: Span::new(start, end),
                                tokens: Vec::new(),
                            });
                        }
                        start = end;
                        i += dot_count;
                        continue;
                    }

                    // Bỏ qua dấu chấm bên trong số thập phân (vd: 2.1, 3.14)
                    let prev_char = if i > 0 { Some(chars[i - 1].1) } else { None };
                    let next_char = chars.get(i + 1).map(|(_, ch)| *ch);
                    if prev_char.is_some_and(|ch| ch.is_ascii_digit())
                        && next_char.is_some_and(|ch| ch.is_ascii_digit())
                    {
                        i += 1;
                        continue;
                    }

                    // Kiểm tra abbreviation
                    if self.is_abbreviation(input, byte_pos) {
                        i += 1;
                        continue;
                    }
                }

                // Sentence boundary
                let end = byte_pos + c.len_utf8();
                let text = input[start..end].trim();
                if !text.is_empty() {
                    sentences.push(Sentence {
                        text: input[start..end].trim_start(),
                        span: Span::new(start, end),
                        tokens: Vec::new(),
                    });
                }
                start = end;
            }

            i += 1;
        }

        // Phần còn lại
        let remaining = input[start..].trim();
        if !remaining.is_empty() {
            sentences.push(Sentence {
                text: remaining,
                span: Span::new(start, input.len()),
                tokens: Vec::new(),
            });
        }

        Ok(sentences)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seg(input: &str) -> Vec<String> {
        let segmenter = RuleSegmenter;
        segmenter
            .segment(input)
            .unwrap()
            .iter()
            .map(|s| s.text.trim().to_string())
            .collect()
    }

    #[test]
    fn basic_sentences() {
        let result = seg("Hôm nay trời đẹp. Tôi đi chơi!");
        assert_eq!(result, vec!["Hôm nay trời đẹp.", "Tôi đi chơi!"]);
    }

    #[test]
    fn question_mark() {
        let result = seg("Bạn có khỏe không? Tôi khỏe.");
        assert_eq!(result, vec!["Bạn có khỏe không?", "Tôi khỏe."]);
    }

    #[test]
    fn abbreviation_tp() {
        let result = seg("TP. HCM có mưa lớn.");
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn abbreviation_gs() {
        let result = seg("GS. Nguyễn Văn A nói.");
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn ellipsis() {
        let result = seg("Tôi nghĩ... rồi tôi đi.");
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn empty_input() {
        let result = seg("");
        assert!(result.is_empty());
    }

    #[test]
    fn no_sentence_boundary() {
        let result = seg("Xin chào Việt Nam");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "Xin chào Việt Nam");
    }

    #[test]
    fn multiple_exclamation() {
        let result = seg("A! B! C!");
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn quoted_sentence_no_split() {
        let result = seg("Anh nói \"Tôi đi. Tôi về.\" rồi đi.");
        // Should be 1 sentence because the dots inside quotes don't split
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn smart_quotes_no_split() {
        let result = seg("Cô ấy bảo \u{201C}Đừng đi!\u{201D} nhưng anh vẫn đi.");
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn quotes_then_normal() {
        let result = seg("\"Xin chào!\" Anh nói. Rồi đi.");
        assert_eq!(result.len(), 2);
    }
}
