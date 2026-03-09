use criterion::{black_box, criterion_group, criterion_main, Criterion};
use vn_nlp_core::Tokenizer;
use vn_nlp_tokenize::{SyllableTokenizer, TokenStream};

fn generate_text(n: usize) -> String {
    let words = ["Xin", "chào", "Việt", "Nam", "tôi", "yêu", "đất", "nước"];
    (0..n)
        .map(|i| words[i % words.len()])
        .collect::<Vec<_>>()
        .join(" ")
}

fn bench_tokenize_10k(c: &mut Criterion) {
    let text = generate_text(10_000);
    c.bench_function("syllable_tokenize_10k", |b| {
        let tokenizer = SyllableTokenizer::default();
        b.iter(|| tokenizer.tokenize(black_box(&text)))
    });
}

fn bench_tokenize_100k(c: &mut Criterion) {
    let text = generate_text(100_000);
    c.bench_function("syllable_tokenize_100k", |b| {
        let tokenizer = SyllableTokenizer::default();
        b.iter(|| tokenizer.tokenize(black_box(&text)))
    });
}

fn bench_stream_vs_collect(c: &mut Criterion) {
    let text = generate_text(10_000);

    let mut group = c.benchmark_group("stream_vs_collect");
    group.bench_function("syllable_collect", |b| {
        let tokenizer = SyllableTokenizer::default();
        b.iter(|| tokenizer.tokenize(black_box(&text)))
    });
    group.bench_function("stream_collect", |b| {
        b.iter(|| {
            let stream = TokenStream::new(black_box(&text));
            let _: Vec<_> = stream.collect::<Result<Vec<_>, _>>().unwrap();
        })
    });
    group.finish();
}

fn bench_vs_manual_split(c: &mut Criterion) {
    let text = generate_text(10_000);

    let mut group = c.benchmark_group("tokenizer_vs_manual");
    group.bench_function("syllable_tokenizer", |b| {
        let tokenizer = SyllableTokenizer::default();
        b.iter(|| tokenizer.tokenize(black_box(&text)))
    });
    group.bench_function("manual_split_whitespace", |b| {
        b.iter(|| {
            let _: Vec<&str> = black_box(&text).split_whitespace().collect();
        })
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_tokenize_10k,
    bench_tokenize_100k,
    bench_stream_vs_collect,
    bench_vs_manual_split
);
criterion_main!(benches);
