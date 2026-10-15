# Xiangqi Tutor

Gia sư cờ tướng có nguồn: sách → văn bản và hình cờ → thẻ Markdown → kiểm tra → index → giảng bài có bằng chứng.

Dự án khởi tạo ngày 2026-10-15. Nhánh làm việc và nhánh mặc định mong muốn: `develop`.

## Có thể chạy ngay

- Rust **1.98.1**, edition 2024; workspace `xqt-knowledge` + CLI `xqt`.
- Đọc hợp đồng JSON trong thẻ `.md`, giữ nguyên phần lời giảng Markdown.
- Kiểm tra schema, ID, loại tham chiếu, nguồn/trang, snapshot SHA-256 + revision, chu trình phụ thuộc.
- Kiểm tra **cấu trúc** FEN và ký pháp lịch sử; chưa kiểm tra tính hợp lệ của thế hoặc phát lại nước.
- Xuất kế hoạch index xác định, chỉ lấy claim được duyệt biên tập và có nhãn bằng chứng `supported`; loại dữ liệu phụ thuộc chưa đủ điều kiện.
- Chặn nguồn `private` trong kế hoạch `--public`. Xuất metadata không tự xác minh nội dung bằng chứng.

```bash
rustup toolchain install 1.98.1 --profile minimal --component rustfmt,clippy
cargo test --workspace --locked
cargo run --locked -p xqt -- validate knowledge/examples
mkdir -p build
cargo run --locked -p xqt -- index-plan knowledge/examples --public > build/index-plan.json
```

`index-plan` **không gọi embedding, không kết nối Qdrant**. Kết quả luôn ghi `not_embedded`, `editorial_metadata_only`. CLI chưa tích hợp OCR/Pikafish/Lean. [MinerU local](docs/workflows/mineru-local.md) đã chạy độc lập trên PDF tự soạn; OCR tiếng Việt còn lỗi dấu, chưa thử sách thật. Xem [bằng chứng kiểm tra](docs/verification.md).

## Kiến trúc đã chọn

| Thành phần | Quyết định |
| --- | --- |
| Điều phối và kiểm tra | Rust 1.98.1; logic thuần tách IO |
| Tri thức do người biên tập sửa | Thẻ Markdown + ID ổn định + Git; tài sản nguồn lưu riêng |
| Vector DB | Qdrant; chỉ mục dẫn xuất có thể xóa/xây lại, không là nguồn tri thức gốc |
| Tìm kiếm | Exact position + bộ lọc + sparse/dense + xếp hạng lại; adapter còn trong kế hoạch |
| OCR | Adapter PaddleOCR cho chữ/bố cục; adapter riêng cho hình bàn cờ |
| Cờ | Pikafish là dịch vụ/process phân tích; bộ luật và replay là cổng riêng |
| Lý luận | Mệnh đề có điều kiện, ngoại lệ, bằng chứng; Lean cho tập con cần chứng minh |
| Giao tiếp gia sư | Hợp đồng tool cho ChatGPT/MCP ở bước sau; chưa có giao diện mới |

Không phân phối sách, model OCR hay NNUE trong repo công khai. Dữ liệu mẫu là nội dung tự soạn, không phải sách của người dùng.

## Đọc theo thứ tự

1. [Phạm vi và tiêu chí sản phẩm](docs/product.md)
2. [Review quyết định và điểm cần sửa](docs/review.md)
3. [Kiến trúc và ranh giới tin cậy](docs/architecture.md)
4. [Hợp đồng Markdown v1](docs/contracts/markdown-v1.md)
5. [Mệnh đề và ký hiệu](docs/contracts/claims.md)
6. [Làm sạch và OCR](docs/workflows/ingestion.md)
7. [Kiểm chứng cờ và engine](docs/workflows/verification.md)
8. [Index và truy xuất](docs/workflows/indexing.md)
9. [Hiệu đính và bổ sung chuyên môn](docs/workflows/editorial.md)
10. [Đối chiếu app/công cụ tương tự](docs/references.md)
11. [Hàng đợi triển khai](docs/work-plan/README.md)

## Phát triển

Agent làm việc theo [bản đồ 8 skills](.agents/skills/README.md): điều phối, engineering, review,
nhập sách/OCR, hiệu đính, kiểm chứng, index và gia sư. Các skill nằm trong repository;
OCR/Pikafish/Lean/retrieval vẫn cần adapter thật trước khi có bằng chứng live.

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
python3 scripts/check_links.py
python3 scripts/check_skills.py
python3 -m unittest discover -s scripts -p 'test_*.py'
```

Dịch vụ vector cho phát triển, chạy tùy chọn bằng `docker compose up -d qdrant`. Docker không bắt buộc để kiểm tra corpus; xem [config](config/README.md).

Chỉnh một record cần tăng `revision`, duyệt nội dung và cập nhật snapshot của từng phụ thuộc **sau khi xem lại**. CLI không tự repin hoặc tự duyệt. Hướng dẫn [CONTRIBUTING](CONTRIBUTING.md).
