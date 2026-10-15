# Skills cho Xiangqi Tutor

Skills này dành cho agent làm việc trong repository. Đọc đúng chủ sở hữu theo bảng;
không nạp toàn bộ bộ skills cho một tác vụ nhỏ. `AGENTS.md` là điểm vào chung,
contract/code là nguồn quy tắc; skill hướng dẫn cách thực hiện và thu bằng chứng.

| Yêu cầu | Skill | Đầu ra chính |
| --- | --- | --- |
| “Làm issue”, “tiếp tục pilot”, triển khai từ queue | [xiangqi-work-item](xiangqi-work-item/SKILL.md) | Một kết quả có ranh giới, ledger, kiểm tra, bàn giao/PR |
| Viết/sửa Rust, CLI, parser, adapter | [xiangqi-engineering](xiangqi-engineering/SKILL.md) | Logic thuần, kiểu/lỗi rõ, kiểm tra theo rủi ro |
| Review diff, PR, thay đổi corpus | [xiangqi-code-review](xiangqi-code-review/SKILL.md) | Findings có trigger và bằng chứng; giới hạn đã đọc |
| Nhập sách, làm sạch OCR, khôi phục hình/cây biến | [xiangqi-ingest](xiangqi-ingest/SKILL.md) | Provenance, candidates/draft và danh sách nghi vấn |
| Sửa/làm rõ mệnh đề hoặc bổ sung chuyên môn | [xiangqi-knowledge-edit](xiangqi-knowledge-edit/SKILL.md) | Hiệu đính có revision, điều kiện/ngoại lệ và invalidation |
| Kiểm thế, luật, Pikafish, phản ví dụ, Lean | [xiangqi-verify](xiangqi-verify/SKILL.md) | Kết quả riêng từng cổng, provenance và phạm vi kết luận |
| Kiểm tra index, embedding, Qdrant, tìm kiếm | [xiangqi-index](xiangqi-index/SKILL.md) | Index-plan hoặc generation/retrieval evidence đúng capability |
| Giảng cờ, so phương án, giao/chữa bài | [xiangqi-tutor](xiangqi-tutor/SKILL.md) | Bài học có nguồn, điều kiện, bài tập và feedback |

Đọc engineering khi tác vụ domain có sửa code. Đọc verify khi một kết luận cần luật/engine/proof;
knowledge-edit sở hữu việc sửa thẻ, index sở hữu dữ liệu dẫn xuất. Work-item điều phối các chủ sở hữu,
không tạo vòng gọi bắt buộc từ các skill về work-item.

## Thực trạng và cách dùng

Đã có Markdown parser, kiểm schema/reference/snapshot/FEN cấu trúc và index-plan offline.
OCR, legal replay, Pikafish, Lean, embedding/Qdrant live và ChatGPT tool runtime chưa được triển khai ở baseline.
Skill mô tả cách dùng capability khi có thật; không cài thêm runtime đó.
Xem trạng thái hiện tại trong [README](../../README.md) và [verification](../../docs/verification.md).

Các agent hỗ trợ `.agents/skills` có thể khám phá `SKILL.md`; `agents/openai.yaml` cung cấp tên và prompt.
Với agent khác, yêu cầu đọc file trực tiếp, ví dụ:

> Đọc `.agents/skills/xiangqi-work-item/SKILL.md`, triển khai một lát cắt của pilot 010 từ develop.

> Đọc `.agents/skills/xiangqi-knowledge-edit/SKILL.md`, sửa điều kiện claim này và liệt kê phụ thuộc cần kiểm lại.

> Đọc `.agents/skills/xiangqi-verify/SKILL.md`, đánh giá bằng chứng hiện có; chỉ chạy các công cụ thực sự có.

Đây là bộ skills của repository, chưa phải plugin ChatGPT đã cài trong tài khoản.

## Nguồn học hỏi từ Cantos

Đối chiếu Cantos `develop` tại [commit 1854b349](https://github.com/loveoverflowcom/cantos/commit/1854b3497a149654d3ac246e882f09fc81cd960a).
Các liên kết sau cố định phiên bản để tránh diễn giải theo tài liệu đã đổi:

| Nguồn | Phần áp dụng |
| --- | --- |
| [cantos-work-item](https://github.com/loveoverflowcom/cantos/blob/1854b3497a149654d3ac246e882f09fc81cd960a/.agents/skills/cantos-work-item/SKILL.md) | Một kết quả review, routing theo owner, ledger tiêu chí → oracle → evidence |
| [cantos-engineering](https://github.com/loveoverflowcom/cantos/blob/1854b3497a149654d3ac246e882f09fc81cd960a/.agents/skills/cantos-engineering/SKILL.md) | Pure core trước IO, types/lỗi rõ, audit đường tạo, kiểm tra nhiều lớp |
| [cantos-code-review](https://github.com/loveoverflowcom/cantos/blob/1854b3497a149654d3ac246e882f09fc81cd960a/.agents/skills/cantos-code-review/SKILL.md) | Review theo ranh giới và consumer, oracle độc lập, bằng chứng gắn HEAD |
| [cantos-behavior](https://github.com/loveoverflowcom/cantos/blob/1854b3497a149654d3ac246e882f09fc81cd960a/.agents/skills/cantos-behavior/SKILL.md) | Tách đặc tả/model chạy được khỏi conformance của implementation |
| [cantos-production-pipeline](https://github.com/loveoverflowcom/cantos/blob/1854b3497a149654d3ac246e882f09fc81cd960a/.agents/skills/cantos-production-pipeline/SKILL.md) | Candidate khác accepted artifact; input fingerprint và stale evidence |
| [approvals-and-gates](https://github.com/loveoverflowcom/cantos/blob/1854b3497a149654d3ac246e882f09fc81cd960a/.agents/skills/cantos-publication/references/approvals-and-gates.md), [staged-publication](https://github.com/loveoverflowcom/cantos/blob/1854b3497a149654d3ac246e882f09fc81cd960a/.agents/skills/cantos-publication/references/staged-publication.md) | Review gắn đúng candidate; chuẩn bị/kiểm trước khi đổi active generation |

Điều chỉnh cho Xiangqi: raw/corrected/editorial; nguồn riêng tư; JSON `xqt` hiện có;
dependency DAG khác relations tranh luận; OCR confidence khác legality, engine score và formal proof.
Không mang stack Cantos, xtask, Quint, audio jobs hoặc chính sách xin lại quyền vào dự án này.

## Duy trì và kiểm tra

- Sửa quy tắc ở tài liệu chủ sở hữu trước; skill chỉ dẫn tới tài liệu đó. Sửa metadata cùng skill khi trigger đổi.
- Giữ frontmatter hai trường `name` và `description`; cả hai dùng chuỗi JSON double-quoted hợp lệ trong YAML.
  `agents/openai.yaml` dùng `interface` với ba chuỗi double-quoted `display_name`, `short_description`, `default_prompt`.
  Đây là tập cú pháp có chủ ý để checker chạy bằng Python standard library.
- Chạy `python3 scripts/check_skills.py`, `python3 -m unittest discover -s scripts -p 'test_*.py'`
  và `python3 scripts/check_links.py`. Cổng này kiểm cấu trúc, không chứng minh agent luôn làm đúng.
- Khi sửa workflow đáng kể, thử tình huống mới bằng agent ít context; dùng artifact/prompt gốc,
  không đưa sẵn kết luận mong muốn. Kiểm tra routing và giới hạn công cụ từ output thật.
