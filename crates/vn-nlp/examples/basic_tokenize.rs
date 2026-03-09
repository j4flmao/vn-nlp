use vn_nlp::tokenize;

fn main() {
    let text = "Xin chào, tôi yêu Việt Nam!";
    println!("Input: {text}");

    let tokens = tokenize(text).unwrap();
    for token in &tokens {
        println!(
            "  {:?} → {:?} [{},{})",
            token.text, token.kind, token.span.start, token.span.end
        );
    }
}
