# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.2] — 2026-03-09

## [0.1.1] — 2026-03-09

### Added
- Cargo workspace with 5 crates (`vn-nlp`, `vn-nlp-core`, `vn-nlp-tokenize`, `vn-nlp-normalize`, `vn-nlp-segment`)
- Core types: `Token`, `Span`, `TokenKind`, `Sentence`
- Core traits: `Tokenizer`, `Normalizer`, `Segmenter`
- `VnNlpError` error enum with `thiserror`
- Syllable-based tokenizer (zero-copy, O(n))
- `TokenStream` lazy iterator API
- Token classification: Word, Punctuation, Number, Whitespace
- CI/CD pipeline (GitHub Actions)
