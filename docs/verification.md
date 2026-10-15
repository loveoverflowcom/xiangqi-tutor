# Bằng chứng kiểm tra bootstrap

Kiểm tra ngày 2026-10-15 trên Linux x86_64, Rust **1.98.1 (48a229cea 2026-09-01)**. Toolchain được cài đúng phiên bản cho lượt làm việc này; repo pin trong rust-toolchain.toml và Cargo.toml, dependency resolution trong Cargo.lock.

| Kiểm tra | Kết quả |
| --- | --- |
| cargo check --workspace | PASS |
| cargo fmt --all -- --check | PASS |
| cargo clippy --workspace --all-targets --locked -- -D warnings | PASS |
| cargo test --workspace --locked | PASS — 15 contract tests |
| xqt validate knowledge/examples | PASS — 4 records; cấu trúc/schema/snapshot, không là legal replay |
| xqt index-plan knowledge/examples --public | PASS — 1 chunk, 1 claim bị loại; history unknown và ruleset unresolved được giữ |
| Embedding status trong plan | not_embedded |
| Verification trong chunk | editorial_metadata_only |
| Local Markdown link targets | PASS — scripts/check_links.py |

Các tests bao phủ stale khi chỉ đổi prose, dependency thiếu, ID trùng, metadata lạ, tự gắn kernel_checked, sai lượt/dimension FEN, metadata thiếu/nhân đôi, conflicted exclusion, nguồn private xuyên chuỗi phụ thuộc, rejected dependency, loại tham chiếu sai, region sai, thiếu reasoning và relation tranh luận có chu trình.

## Chưa chạy

- Sách thật của người dùng: NOT_RUN, chưa có file được nhập trong tác vụ.
- OCR chữ/hình và đánh giá accuracy: NOT_RUN.
- Bộ luật và replay history: NOT_RUN; validator chỉ kiểm tra hình thức.
- Pikafish/NNUE: NOT_RUN, chưa có adapter hoặc binary trong repo.
- Lean: NOT_RUN, chưa có theorem project.
- Embedding/Qdrant live/retrieval benchmark: NOT_RUN.
- Docker Compose: NOT_RUN; môi trường này không có Docker. Cấu hình pin image và loopback, không thay thế integration test.

Workflow GitHub Actions thực hiện lại kiểm tra Rust, corpus, index-plan và link trên push develop/PR. Tình trạng từng run ở trang Actions của repository; không dùng tài liệu này thay log CI theo commit.
