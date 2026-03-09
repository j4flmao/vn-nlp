use vn_nlp::segment;

fn main() {
    let text = "Hôm nay trời đẹp. Tôi đi chơi! Bạn có muốn không?";
    println!("Input: {text}\n");

    let sentences = segment(text).unwrap();
    for (i, s) in sentences.iter().enumerate() {
        println!("Sentence {}: \"{}\"", i + 1, s.text.trim());
    }
}
