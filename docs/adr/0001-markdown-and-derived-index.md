# ADR 0001 — Markdown làm nguồn biên tập, Qdrant làm chỉ mục dẫn xuất

Status: Accepted for bootstrap, 2026-10-15.

## Context

Chủ dự án muốn trực tiếp điều chỉnh mệnh đề và lý luận trong `.md`, đồng thời truy xuất bằng vector DB. Chỉ lưu embedding hoặc chỉ viết Lean sẽ làm khó hiệu đính và giữ nguồn.

## Decision

Dùng Markdown với một fenced `xqt` JSON block được typed-deserialize. Giữ phần prose bằng Markdown chuẩn. Chọn Qdrant làm vector DB, pin image phát triển v1.19.2; embedding chưa chọn. Dữ liệu engine, formal proof và index phải ghi provenance độc lập.

## Alternatives

YAML front matter thuận tiện nhưng thêm parser và biến thể typing; có thể thêm adapter nhập sau, không duy trì hai định dạng canonical. PostgreSQL+pgvector là phương án hợp lý nếu cần gộp dữ liệu giao dịch sau này; hiện chưa có app giao dịch để biện minh thêm DB. Lean-only không phù hợp để biên tập mệnh đề chưa hoàn chỉnh.

## Consequences

Git diff và CLI offline dùng được ngay. ID + snapshots giúp phát hiện stale. Cần adapter riêng cho embedding/Qdrant; không tuyên bố export JSON đã index. Đổi định dạng canonical phải có migration schema và bộ fixture round-trip.
