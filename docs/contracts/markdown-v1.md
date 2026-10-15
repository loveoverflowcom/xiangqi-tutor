# Hợp đồng Markdown v1

Nguồn thực thi schema là `crates/knowledge/src/record.rs`; contract version là số nguyên `1`. Unknown fields/enum values bị từ chối. Một file = một record = đúng một fenced block có info string `xqt`, nội dung JSON chuẩn. Không dùng YAML ngầm hoặc nhận lệnh từ prose.

## Envelope bắt buộc

`schema_version`, `id` (ASCII chữ/số/`-`/`_`, 1–100 bytes), `revision` (>0), `title`, `language` (`vi`, `zh-Hans`, `zh-Hant`, `en`), `review`, `dependencies`, `payload`.

Review có state draft/approved/rejected, by (string hoặc null), note. Approved cần người duyệt và lý do nhưng chỉ là xác nhận biên tập; không chứng thực danh tính hay độ đúng cờ.

Dependencies là snapshot `{id, revision, content_sha256}`. Hash SHA-256 hex chữ thường của **toàn bộ bytes Markdown**, gồm newline. Không tự repin. Một revision không được tái sử dụng khi xuất bản bản sửa; validator vẫn phát hiện bytes đổi dù người sửa quên tăng revision.

## Payload

| kind | Trường |
| --- | --- |
| source | locator, asset_sha256, rights: private/redistributable/original |
| position | fen, ruleset, history, origin |
| claim | claim_type, statement, scope, position_ids, sources, conditions, exceptions, conclusion, evidence_status, formal_status, evidence, relations |

Source hash là bytes tài sản gốc, khác hash thẻ nguồn. Bootstrap kiểm tra hình dạng hash, **chưa xác minh tài sản ngoài repo**. Locator `example:` chỉ dùng cho mẫu; hợp đồng locator nguồn thật sẽ được importer chốt.

SourceSpan gồm source_id, pdf_page 1-based, printed_page (string/null), region (null hoặc normalized `[x,y,width,height]`, gốc trên-trái). PDF page là tọa độ logic nhập; importer ảnh lẻ phải ánh xạ số trang, không giả vờ đã có PDF.

Position FEN canonical: 10 ranks × 9 giao điểm, alphabet `rnbakcp` và chữ hoa cho Đỏ, `w`=Đỏ/`b`=Đen, hai placeholder `- -`, halfmove u32, fullmove >0. Quy ước tọa độ nước `a0a1`: file a–i, rank 0–9. Adapter nguồn h/e hoặc ký pháp Trung phải chuyển có kiểm tra; không thay chữ bằng regex tùy tiện. Validator v1 yêu cầu mỗi bên một Tướng, chỉ kiểm tra cấu trúc, không giới hạn vị trí cung/đường qua sông.

History `{state:"unknown"}` hoặc `{state:"known",initial_fen,moves}`. Known hiện chỉ là khai báo: chưa replay kiểm chứng history khớp final FEN. Một ảnh không đủ xác định lượt đi thì giữ ở staging ngoài canonical position; không điền mặc định w. Ruleset chưa chọn ghi `unresolved` và không tuyên bố kết luận luật đầy đủ.

Claim phải có source span; scope position cần position_ids. Tất cả source/position tham chiếu phải có dependency snapshot đúng loại. Các H2 bắt buộc là `## [reasoning]`, `## [counterexamples]`, `## [questions]`. Statement canonical chỉ nằm trong JSON để tránh hai bản phát biểu khác nhau trong cùng thẻ. Phần prose không là kết quả thực thi logic.

Evidence gồm kind source/human/engine, reference, artifact_sha256, note. Các artifact ngoài corpus chưa được mở/kiểm tra. `supported` không tự động được engine hay Lean xác nhận. Dùng evidence run contract ở workflows/verification.md khi làm adapter.

Relations gồm supports/challenges/refines/supersedes và target_id. Đích phải là claim tồn tại. Quan hệ này có thể có chu trình và không cần dependency; chỉ pin dependency khi thực sự dùng nội dung đích làm đầu vào suy luận. Supersedes **chưa tự rút record cũ khỏi index**: cần chuyển review record cũ về rejected và ghi lý do; workflow tái xuất index xử lý xóa generation cũ.

## Trạng thái và xuất index

Evidence unassessed/supported/challenged/conflicted; formal not_formalized/statement_only/proof_pending. Schema không nhận kernel_checked. Chỉ claim approved+supported cùng toàn bộ dependency đủ điều kiện được xuất; mode public còn chặn nguồn private xuyên suốt chuỗi phụ thuộc. Nguồn được nhãn original/redistributable là khai báo của chủ corpus, không là kết luận pháp lý.

Xem record hoàn chỉnh trong `knowledge/examples/`; bản chưa điền ở `templates/` không nằm trong corpus CLI.
