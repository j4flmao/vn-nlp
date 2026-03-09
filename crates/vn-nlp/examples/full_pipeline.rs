use vn_nlp::{normalize, segment, tokenize};

fn main() {
    let raw = "  Hôm nay, TP.HCM  có mưa lớn.   Tôi ở nhà!  ";
    println!("Raw input: \"{raw}\"");

    // Step 1: Normalize
    let clean = normalize(raw);
    println!("Normalized: \"{clean}\"");

    // Step 2: Segment
    let sentences = segment(&clean).unwrap();
    println!("\n{} câu:", sentences.len());
    for (i, s) in sentences.iter().enumerate() {
        println!("  [{i}] \"{}\"", s.text.trim());

        // Step 3: Tokenize each sentence
        let tokens = tokenize(s.text.trim()).unwrap();
        for t in &tokens {
            print!("    {:?}", t.text);
        }
        println!();
    }
}
