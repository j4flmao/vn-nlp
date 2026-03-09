use vn_nlp::normalize;

fn main() {
    let text = "  Xin   chào   Việt   Nam  ";
    println!("Input:      \"{text}\"");
    println!("Normalized: \"{}\"", normalize(text));

    let vn = "Tiếng Việt rất đẹp";
    println!("\nInput:      \"{vn}\"");
    println!(
        "No accents: \"{}\"",
        vn_nlp::normalize::strip_diacritics(vn)
    );
}
