## 📋 Mô Tả

<!-- Mô tả ngắn gọn những gì PR này làm -->

Closes #<!-- issue number -->

## 🔄 Loại Thay Đổi

- [ ] 🐛 Bug fix (non-breaking)
- [ ] ✨ New feature (non-breaking)
- [ ] 💥 Breaking change
- [ ] 📝 Docs only
- [ ] ⚡ Performance improvement
- [ ] 🔧 Refactor (no behavior change)

## ✅ Checklist

- [ ] `cargo test --all` pass
- [ ] `cargo clippy --all -- -D warnings` pass
- [ ] `cargo fmt --all` đã chạy
- [ ] Tests mới cho behavior mới
- [ ] Docs cập nhật (nếu thay đổi public API)
- [ ] CHANGELOG.md cập nhật
- [ ] Benchmark không regress (nếu thay đổi hot path)

## 🧪 Cách Test

```bash
cargo test -p vn-nlp-tokenize -- --nocapture
```

## 📊 Benchmark (nếu relevant)

| Operation | Before | After | Delta |
|-----------|--------|-------|-------|
| tokenize 10k | Xms | Yms | ±Z% |
