# Contributing to vn-nlp

Xem chi tiết tại [docs/04-contributing.md](docs/04-contributing.md).

## Quick Start

```bash
git clone https://github.com/j4flmao/vn-nlp.git
cd vn-nlp
cargo build --all
cargo test --all
cargo clippy --all -- -D warnings
cargo fmt --all -- --check
```

## Commit Convention

```
<type>(<scope>): <subject>
```

Types: `feat`, `fix`, `docs`, `test`, `bench`, `refactor`, `chore`, `perf`

## Pull Request

- Branch từ `dev`, không phải `main`
- `cargo test --all` pass
- `cargo clippy --all -- -D warnings` pass
- `cargo fmt --all` đã chạy
