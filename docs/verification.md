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

## Chưa chạy trong bootstrap

- Sách thật của người dùng: NOT_RUN, chưa có file được nhập trong tác vụ.
- OCR chữ/hình và đánh giá accuracy: NOT_RUN.
- Bộ luật và replay history: NOT_RUN; validator chỉ kiểm tra hình thức.
- Pikafish/NNUE: NOT_RUN, chưa có adapter hoặc binary trong repo.
- Lean: NOT_RUN, chưa có theorem project.
- Embedding/Qdrant live/retrieval benchmark: NOT_RUN.
- Docker Compose: NOT_RUN; môi trường này không có Docker. Cấu hình pin image và loopback, không thay thế integration test.

Workflow GitHub Actions thực hiện lại kiểm tra Rust, corpus, index-plan và link trên push develop/PR. Tình trạng từng run ở trang Actions của repository; không dùng tài liệu này thay log CI theo commit.

## Bộ skills repository — 2026-10-15

Phạm vi: 8 skills, metadata agent, routing từ AGENTS/README và cổng Python trong CI.
Không đổi code Rust, contract hoặc corpus. Kiểm tra local trên Linux với Python 3.12.14:

| Kiểm tra | Kết quả |
| --- | --- |
| Skill Creator quick_validate, từng thư mục | PASS — 8 skills |
| `python3 scripts/check_skills.py` | PASS — 8 skills và agent interfaces |
| `python3 -m unittest discover -s scripts -p 'test_*.py'` | PASS — 11 test methods, có metadata bị sửa lỗi và CLI exit status |
| `python3 scripts/check_links.py` | PASS — 114 liên kết local |
| `git diff --check` | PASS |
| Rust/CLI local trong tác vụ skills | NOT_RUN — không đổi Rust/contract/corpus; môi trường không có cargo/rustup |

Cổng `skills` trong CI chạy checker, Python tests, link và diff; cổng `contracts` giữ các kiểm tra Rust/CLI hiện có.
Các kết quả CI phải đọc theo SHA của PR, không suy từ PASS bootstrap ở trên.

Thử workflow bằng ba agent ít context trên yêu cầu giả định, chỉ đọc tài liệu và không dùng hệ thống live:

| Tình huống | Hành vi quan sát |
| --- | --- |
| OCR mơ hồ Xe/Pháo, ô bị crop, thiếu lượt, muốn public sách cá nhân | Giữ staging/unknown/private; không tạo FEN, hash hoặc quyền nguồn giả |
| Báo cáo gián tiếp depth 22/+500/một PV; muốn gọi nước duy nhất buộc thắng và Lean chứng minh | Hạn chế kết luận, không giả run/proof; giao bài kiểm tra lập luận thay vì bịa thế/nước |
| Sửa reasoning nhưng giữ revision; B stale; A2 supersedes A | Yêu cầu revision/review và xem từng consumer; không repin hàng loạt, không coi supersedes là retirement tự động |

Các ca này là bằng chứng hành vi tư vấn giới hạn, không là kiểm chứng mọi prompt, legal replay,
OCR accuracy hoặc integration Pikafish/Lean/Qdrant. Sau ca revision, đã làm rõ trong skills rằng
draft/rejected vẫn qua validation và dependency tới claim rejected làm bản thay thế mất eligibility.

## Công cụ MinerU độc lập — 2026-10-15

Trên macOS 26.5.2 arm64, RAM 16 GB, MinerU 4.0.12/Python 3.12.15 đã chạy inference local.
[Quy trình, mẫu tự soạn, snapshot và kết quả OCR](workflows/mineru-local.md) ghi các lệnh đã dùng,
lỗi chứng chỉ uv, cách chuyển tải VLM từ Xet sang HTTP và giới hạn của phép thử.

| Kiểm tra thực tế | Kết quả |
| --- | --- |
| `mineru version --json`, `uv pip check` | PASS — 4.0.12/3.12.15, 122 packages tương thích |
| `mineru-kit models verify --tier standard` | PASS — Torch small models và GGUF VLM đều ok |
| PDF có text layer, Flash/txt | PASS — giữ 6/6 dòng nguyên văn |
| PDF chỉ chứa ảnh, Basic và Standard | PASS thực thi; mỗi chế độ khớp 4/6 dòng nguyên văn, 5/6 nếu bỏ whitespace |
| Chất lượng tiếng Việt trên mẫu | KNOWN_ERRORS — cả hai chế độ OCR còn sai dấu/ký tự |
| `mineru-kit webui --help` | PASS — kiểm tra lệnh; khởi chạy/chuyển đổi qua WebUI NOT_RUN |

PDF sách thật, hình cờ và benchmark accuracy: NOT_RUN. Adapter OCR của `xqt` vẫn planned;
kết quả công cụ độc lập không chứng nhận importer, FEN, thẻ tri thức hoặc index.
