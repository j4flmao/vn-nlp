use vn_nlp::tokenize::TokenStream;

fn main() {
    let text = "Văn bản tiếng Việt rất hay và đẹp!";
    println!("Input: {text}\n");
    println!("Streaming tokens:");

    let stream = TokenStream::new(text);
    for result in stream {
        let token = result.unwrap();
        println!("  {:?} ({:?})", token.text, token.kind);
    }
}
