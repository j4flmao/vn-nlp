/// Chuẩn hóa số viết tắt tiếng Việt.
///
/// # Examples
/// ```
/// use vn_nlp_normalize::number::normalize_number;
///
/// assert_eq!(normalize_number("1k"), "1000");
/// assert_eq!(normalize_number("10tr"), "10000000");
/// assert_eq!(normalize_number("2.5tr"), "2500000");
/// ```
pub fn normalize_number(input: &str) -> String {
    let input_lower = input.to_lowercase();

    // Try to extract numeric prefix and suffix
    let (num_str, suffix) = split_number_suffix(&input_lower);

    if num_str.is_empty() {
        return input.to_string();
    }

    let multiplier = match suffix {
        "k" => Some(1_000u64),
        "tr" | "triệu" => Some(1_000_000),
        "tỷ" | "ty" => Some(1_000_000_000),
        "m" => Some(1_000_000),
        _ => None,
    };

    match multiplier {
        Some(mult) => {
            if let Ok(num) = num_str.parse::<f64>() {
                let result = (num * mult as f64) as u64;
                result.to_string()
            } else {
                input.to_string()
            }
        }
        None => input.to_string(),
    }
}

fn split_number_suffix(input: &str) -> (&str, &str) {
    // Find where digits/dots end and suffix begins
    let num_end = input
        .char_indices()
        .find(|(_, c)| !c.is_ascii_digit() && *c != '.' && *c != ',')
        .map(|(i, _)| i)
        .unwrap_or(input.len());

    (&input[..num_end], &input[num_end..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_k() {
        assert_eq!(normalize_number("1k"), "1000");
        assert_eq!(normalize_number("5k"), "5000");
        assert_eq!(normalize_number("1.5k"), "1500");
    }

    #[test]
    fn normalize_tr() {
        assert_eq!(normalize_number("10tr"), "10000000");
        assert_eq!(normalize_number("2.5tr"), "2500000");
    }

    #[test]
    fn normalize_ty() {
        assert_eq!(normalize_number("1tỷ"), "1000000000");
    }

    #[test]
    fn no_suffix() {
        assert_eq!(normalize_number("123"), "123");
        assert_eq!(normalize_number("hello"), "hello");
    }

    #[test]
    fn empty_input() {
        assert_eq!(normalize_number(""), "");
    }
}
