---
name: "xiangqi-engineering"
description: "Triển khai, sửa lỗi và review kỹ thuật Rust/CLI/hợp đồng Xiangqi Tutor; giữ logic thuần, lỗi typed, snapshot/invalidation và kiểm thử đúng ranh giới. Dùng khi đổi code, schema, adapter hoặc chiến lược xác minh phần mềm."
---

# Kỹ thuật Xiangqi Tutor

Dùng cho thay đổi code Rust, hợp đồng dữ liệu, CLI, lỗi hồi quy hoặc triển khai adapter. Đọc [AGENTS](../../../AGENTS.md), [README](../../../README.md), [review](../../../docs/review.md), [kiến trúc](../../../docs/architecture.md), hợp đồng liên quan và [queue](../../../docs/work-plan/README.md) trước khi sửa.

## Xác định phạm vi thực tế

- Đối chiếu code với yêu cầu; ghi hành vi hiện tại, hành vi cần đạt và ranh giới tin cậy bị ảnh hưởng.
- Kiểm tra [record model](../../../crates/knowledge/src/record.rs), [parser/corpus](../../../crates/knowledge/src/lib.rs), [index-plan](../../../crates/knowledge/src/index.rs) và [CLI](../../../apps/xqt/src/main.rs) khi sửa dữ liệu hoặc eligibility.
- Giữ rõ bootstrap đã có schema/reference/snapshot validation và index-plan. OCR, rule replay, Pikafish, Lean, embedding/Qdrant live và TutorRetrieval còn planned; cấu hình hay mock không chứng minh đã tích hợp.
- Chọn lát cắt nhỏ có consumer và kết quả quan sát được. Ghi follow-up vào queue theo quy ước hiện có; không tạo crate/cổng rỗng hoặc hạ tầng vì dự đoán.

## Giữ invariant ở lõi

1. Viết invariant, input thiếu và kết quả lỗi trước khi sửa thuật toán. Tách phép tính thuần khỏi đọc tệp, network, subprocess và lưu kết quả; giữ adapter mỏng. File loading hiện có trong thư viện không có nghĩa lõi được phép gọi engine/network tùy tiện.
2. Dùng enum và `Result` cho trạng thái/lỗi cần phân biệt; giữ mã diagnostic ổn định. Không nuốt lỗi, chuyển dữ liệu thiếu thành false, hoặc dùng panic cho input corpus không tin cậy. Phân biệt input bị từ chối, vi phạm domain và lỗi hạ tầng; engine không khả dụng không có nghĩa nước đi sai. Chỉ đổi sang thông điệp CLI ở biên thích hợp.
3. Kiểm tra mọi đường tạo giá trị: constructor, `serde`, public fields, fixture, CLI và adapter. Struct deserialize được chưa chắc hợp lệ về ngữ nghĩa; đọc invariant qua parser/validator thực tế. Khi thêm validated type, chặn đường vòng tạo giá trị không hợp lệ và kiểm thử cả deserialize.
4. Giữ `deny_unknown_fields`, kiểu tham chiếu, điều kiện nguồn/trang và phạm vi mệnh đề theo [Markdown v1](../../../docs/contracts/markdown-v1.md). Không thêm field hoặc enum tự phong proof; muốn đổi contract phải sửa schema, tài liệu, fixture và consumer cùng nhau.
5. Giữ nguồn và snapshot bất biến. Dependency pin ID + revision + SHA-256 toàn bộ bytes Markdown; đổi prose cũng có thể invalidates. Không tự repin hoặc gắn approved/supported để kiểm tra xanh. Tách DAG phụ thuộc khỏi quan hệ tranh luận có thể tạo chu trình.
6. Đặt mọi đầu vào OCR/sách/model/tool ở phía dữ liệu không tin cậy. Không thực thi Markdown, predicate hay lệnh đọc được. Khi thật sự triển khai IO adapter, định nghĩa timeout, giới hạn tài nguyên, input/output contract, phiên bản và lỗi typed trước khi nối vào lõi.

## Kiểm chứng đúng tầng

Dùng [hướng dẫn kiểm thử](references/verification.md) để chọn oracle độc lập và ranh giới hồi quy. Phân biệt kết quả kiểm tra phần mềm với tri thức cờ: schema hợp lệ, FEN đúng cấu trúc, metadata approved/supported, artifact đã kiểm tra và proof kernel-checked là các kết luận khác nhau. Đọc [hợp đồng mệnh đề](../../../docs/contracts/claims.md) khi thay đổi điều kiện, ngoại lệ hay bằng chứng.

- Sửa lỗi: giữ một ví dụ tái hiện lỗi, xác định kỳ vọng độc lập với hàm đang sửa, rồi chạy hồi quy đúng phạm vi.
- Sửa core/contract: bao phủ happy path và input bị từ chối ở các ranh giới bị ảnh hưởng; không viết test chỉ sao chép thuật toán sản xuất.
- Sửa adapter: tách fixture offline khỏi integration live. Ghi rõ binary/model/service thực tế đã chạy; mock qua không thay thế kết quả live.
- Chỉ sửa tài liệu/skills: kiểm tra diff, metadata và link; không bắt chạy toàn bộ Rust khi không đổi code, fixture hoặc contract.

## Chạy kiểm tra và bàn giao

Dùng Rust **1.98.1** theo [README](../../../README.md) và `rust-toolchain.toml`; không đổi toolchain để lách lỗi môi trường. Khi đổi code, chạy các lệnh hiện hành từ repo root:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo run --locked -p xqt -- validate knowledge/examples
cargo run --locked -p xqt -- index-plan knowledge/examples --public
python3 scripts/check_links.py
```

Nếu thiếu toolchain, component hoặc service, ghi lệnh và lý do `NOT_RUN`; không nhận kết quả của commit cũ làm lần chạy mới. Sau các kiểm tra phù hợp đã qua, bàn giao thay đổi, invariant được bảo vệ, kết quả đã chạy, giới hạn còn lại và follow-up cần thiết. Chỉ nêu đã triển khai phần có code và bằng chứng tương ứng.
