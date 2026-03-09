# Segmentation (Phân Tách Câu Tiếng Việt)

Tính năng chia tách câu được build sẵn trong module `vn_nlp::segment`. Tính năng này chịu trách nhiệm cắt một đoạn văn dài (Paragraph) thành các mảng câu nhỏ, dựa trên ngữ nghĩa của dấu chấm câu `.`, dấu hỏi chấm `?` hoặc dấu chấm than `!`.

## Cú pháp cơ bản

```rust
use vn_nlp::segment;

fn main() {
    let input_text = "Hôm nay trời đẹp. Tôi đi chơi! Bạn có đi cùng không?";
    
    // Gọi thuật toán segmentation (tách câu)
    let sentences = segment(input_text).unwrap();
    
    // In ra độ dài mảng (có tổng 3 câu)
    println!("Tổng số câu: {}", sentences.len());

    // In từng câu
    for c in sentences {
        println!("Câu: '{}'", c);
    }
}
```

Kết quả mong đợi:
```
Tổng số câu: 3
Câu: 'Hôm nay trời đẹp.'
Câu: 'Tôi đi chơi!'
Câu: 'Bạn có đi cùng không?'
```

## Giải Thích Lỗi
Cũng giống như các hàm khác, hàm `segment` trả về `Result<Vec<&str>, Error>`. 
Trong những trường hợp ngoại lệ như đoạn văn không có ký tự ngắt câu rõ ràng, hàm sẽ văng lỗi `Error` mà bạn có thể dùng `unwrap` để Panic, hoặc `match` để xử lý theo chủ ý của bản thân.
