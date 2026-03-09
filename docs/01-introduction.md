# Giới Thiệu Chung (Introduction)

`vn-nlp` là một thư viện mã nguồn mở được viết hoàn toàn bằng **Rust** (Pure Rust), chuyên dùng để xử lý ngôn ngữ tự nhiên tiếng Việt (Vietnamese NLP).

Mục tiêu của dự án là mang lại các luồng xử lý:
- Tốc độ siêu nhanh do zero-cost abstractions của Rust.
- Tối ưu bộ nhớ với zero-copy mechanisms.
- An toàn vì không sử dụng `unsafe_code` (`#![forbid(unsafe_code)]`).

## Cài đặt thư viện

Bạn có thể thêm thư viện vào dự án Rust của mình rất đơn giản thông qua cấu hình `Cargo.toml`:

```toml
[dependencies]
vn-nlp = "0.1"
```

## Các Modules Chính (Feature Flags)

Mặc định, `vn-nlp` sẽ tự động kích hoạt các modules cơ bản cho bạn, bao gồm:
1. `tokenize`: Chia tách từ vựng.
2. `normalize`: Chuẩn hóa văn bản.
3. `segment`: Tách câu.

Nếu bạn chỉ muốn sử dụng một tính năng nhất định để tích kiệm dung lượng file compile, hãy tắt `default-features` đi:

```toml
[dependencies]
vn-nlp = { version = "0.1", default-features = false, features = ["tokenize"] }
```
