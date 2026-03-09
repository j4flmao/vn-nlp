# Normalization (Chuẩn Hóa Tiếng Việt)

Tính năng chuẩn hóa làm sạch chuỗi được build sẵn trong module `vn_nlp::normalize`. Bạn dùng nó khi nào? Khi muốn bỏ dấu câu, đưa chữ về chữ thường cho quá trình xử lý tìm kiếm (Search Engine), hoặc xóa đi các icon rác.

## Cú pháp cơ bản

```rust
use vn_nlp::normalize;

fn main() {
    // String chứa các ký tự có dấu phức tạp
    let input_text = "LờiChào Tốt Lành: Tiếng Việt Rất Tốt!";

    // Xóa loại bỏ thanh điệu tiếng Việt (Diacritics remove)
    let clean = normalize::strip_diacritics(input_text);

    // In ra kết quả
    println!("Clean text: {}", clean);
}
```

Kết quả in ra:
```
Clean text: LoiChao Tot Lanh: Tieng Viet Rat Tot!
```

## Các tính năng mở rộng khác của `normalize`:

- **NFC / NFD Convert:** Ở Việt Nam hay có tình huống chữ 'ê' + dấu sắc riêng, và chữ 'ế' gộp. Bạn có thể Convert từ NFD về NFC hoặc ngược lại.
- **Lowercasing / Uppercasing:** Chuyển qua lại chữ in hoa, in thường.

Đây là bước cực kỳ quan trọng cho các quá trình NLP sau lưng (như Embedding Model, Classification), để model học được dữ liệu sạch.
