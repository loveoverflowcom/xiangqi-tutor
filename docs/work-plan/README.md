# Hàng đợi triển khai

Các số 010/020/030/040 thể hiện thứ tự khuyến nghị hiện tại, có thể đánh lại số, không là ID cố định. Mỗi file là một thay đổi có ranh giới review rõ. Không mở issue tự động từ queue này.

Bootstrap đã thực hiện: docs, mẫu Markdown, validator/reference snapshots, index-plan offline, Rust 1.98.1 và CI. Kết quả kiểm tra ở [verification](../verification.md).

| Thứ tự | Việc | Phụ thuộc |
| --- | --- | --- |
| [010](010-pilot-ingestion.md) | Nhập pilot có provenance và duyệt OCR | Sách mẫu, mapping trang, gold corpus |
| [020](020-rules-and-pikafish.md) | Luật, cây biến và phân tích Pikafish có provenance | 010 và chọn ruleset |
| [030](030-vector-retrieval.md) | Embedding + Qdrant generation + retrieval benchmark | Gold claims đủ điều kiện từ 010/020 |
| [040](040-chatgpt-tutor.md) | Tool gia sư qua ChatGPT, bài tập và hiệu đính | 030 và hợp đồng engine |

[Formal checker](backlog/formal-certificates.md) làm khi đã có rules model ổn định và một tập chiến thuật có giá trị. Không bắt Lean hóa toàn bộ sách trước khi dùng pilot.
