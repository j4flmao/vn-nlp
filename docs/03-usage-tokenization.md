# Tokenization (Chia Từ Tiếng Việt)

Tính năng chia tách từ được build sẵn trong module `vn_nlp::tokenize`.

Thuật toán ở đây giúp phân loại văn bản đầu vào ra thành các Token (từ). Mỗi token chứa thông tin: Text, Vị trí Start/End, và các thẻ POS, vv...

## Cú pháp cơ bản

```rust
use vn_nlp::tokenize;

fn main() {
    let input_text = "Xin chào Việt Nam!";
    
    // Gọi thuật toán tokenization
    let tokens = tokenize(input_text).unwrap();
    
    // In kết quả
    for t in tokens {
        println!("Token: '{}'", t.text);
    }
}
```

Kết quả mong đợi:
```
Token: 'Xin'
Token: 'chào'
Token: 'Việt'
Token: 'Nam'
Token: '!'
```

## Giải Thích
Tham số đầu vào của hàm `tokenize` là một tham chiếu chuỗi Text `&str`. Nó trả ra kiểu dữ liệu `Result<Vec<Token>, Error>`. 

Vì hàm `tokenize` chạy theo phong cách Zero-Cost của Rust nên các Text bên trong `Token` không bị phân bổ lại vào Heap Memory mà chỉ là tham chiếu lifetime đến `input_text` cũ. Nên cực đỉnh cho những đoạn text dài!
