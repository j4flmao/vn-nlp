# 🦀 vn-nlp

[![CI](https://github.com/j4flmao/vn-nlp/actions/workflows/ci.yml/badge.svg)](https://github.com/j4flmao/vn-nlp/actions)
[![crates.io](https://img.shields.io/crates/v/vn-nlp.svg)](https://crates.io/crates/vn-nlp)
[![docs.rs](https://docs.rs/vn-nlp/badge.svg)](https://docs.rs/vn-nlp)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

> **Vietnamese NLP library in pure Rust** — tokenization, normalization, segmentation.  
> Zero-copy, `no_std` compatible (with `alloc`), zero-cost abstractions.

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
vn-nlp = "0.1"
```

### Tokenize

```rust
use vn_nlp::tokenize;

let tokens = tokenize("Xin chào Việt Nam!").unwrap();
assert_eq!(tokens[0].text, "Xin");
assert_eq!(tokens[1].text, "chào");
```

### Normalize

```rust
use vn_nlp::normalize;

let clean = normalize::strip_diacritics("Tiếng Việt");
assert_eq!(clean, "Tieng Viet");
```

### Sentence Segmentation

```rust
use vn_nlp::segment;

let sentences = segment("Hôm nay trời đẹp. Tôi đi chơi.").unwrap();
assert_eq!(sentences.len(), 2);
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `tokenize` | ✅ | Word tokenization |
| `normalize` | ✅ | Unicode normalization & diacritics |
| `segment` | ✅ | Sentence segmentation |
| `dictionary` | ❌ | Dictionary-based word segmentation |

```toml
# Chỉ dùng tokenizer
vn-nlp = { version = "0.1", default-features = false, features = ["tokenize"] }
```

## Documentation

- [API Docs (docs.rs)](https://docs.rs/vn-nlp)
- [Architecture](https://github.com/j4flmao/vn-nlp/blob/main/docs/02-architecture.md)
- [Contributing](https://github.com/j4flmao/vn-nlp/blob/main/CONTRIBUTING.md)

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.
