# Index tri thức và truy xuất

## Nguồn và đơn vị index

Nguồn canonical là record Markdown. Qdrant chỉ giữ payload/chunks dẫn xuất cùng corpus generation; không cập nhật lý luận chỉ trong vector DB. Một claim atomic + reasoning/counterexamples/questions là một đơn vị bootstrap, chưa chia token tự động. Khi thẻ quá dài, tạo child chunk có parent_id và đủ context; không cắt nước đi khỏi thế xuất phát.

Ba mặt dữ liệu: source/page/crop provenance; position/history/variation/engine; concept/claim/explanation/evidence. Có thể materialize graph quan hệ trong SQL/JSON sau; chưa cần thêm graph DB.

## Đã có

`xqt index-plan <dir> [--public]` xuất JSON hoàn chỉnh ra stdout sau khi validate toàn corpus. Mỗi claim có ID chunk theo record+revision+bytes hash, statement, prose, loại, language, sources, dependency snapshots và position contexts (ID/revision/hash, FEN, ruleset, history). Không làm mất trạng thái history unknown hay ruleset unresolved khi xuất.

Chỉ xuất approved+supported và dependencies đủ điều kiện xuyên suốt DAG. Challenged/conflicted/unassessed không vào luồng giảng mặc định. Draft và rejected không bị xóa khỏi corpus. --public chặn mọi nguồn private trong dependency closure. Metadata review/evidence được khai báo, chưa là live attestation. `not_embedded` và `editorial_metadata_only` ghi rõ giới hạn.

Hash thay đổi làm snapshot fail ngay cả khi chưa tăng revision. Không tự sửa snapshot. Claim supersedes record cũ chưa tự thực thi retirement; cần chuyển record cũ rejected và review bản thay thế.

## Cần triển khai ở queue 030

1. Đăng ký embedding profile: model/revision, dimensions, metric, tokenizer, normalization, language, prompt và chunker version. Không điền dimensions bịa trước khi chọn model.
2. Đọc index-plan đã duyệt; sinh dense và sparse bằng tokenizer phù hợp Trung/Việt. Thử alias giản/phồn/Việt cho khái niệm và khai cuộc; không sửa mất văn bản gốc.
3. Qdrant staging collection có schema/payload indexes, corpus generation và embedding profile. Chuyển chunk SHA thành UUID hợp lệ theo ánh xạ có phiên bản (Qdrant không nhận arbitrary SHA string làm point ID). Kiểm tra collision và khả năng truy ngược.
4. Upsert có idempotency; kiểm tra số record, dimensions, các snapshot và truy xuất gold. Không tạo fake vectors để báo hoàn thành.
5. Đổi alias sang generation mới sau kiểm tra, giữ bản cũ rollback. Retirement/delete phải loại points vắng mặt; upsert đơn thuần không xóa vectors cũ. Bản cũ không được phục vụ lẫn với claim mới trong cùng response.
6. Sửa nguồn/FEN/atom/evidence invalidates các phụ thuộc và cache liên quan; tái duyệt trước publish. Truy xuất đối chiếu generation/hash với manifest đang hoạt động.

## Retrieval đề xuất

Có thế cờ: exact canonical position/ruleset/history context trước; tiếp theo lọc force/phase/opening/theme và tìm concept tương tự. Không dùng text embedding của FEN để nhận thế “gần giống”. Cùng board có history/rules khác phải giữ ngữ cảnh riêng. Nếu dùng đối xứng/lật bàn, lưu phép biến đổi, map nước/POV và chỉ gộp khi luật cho phép.

Câu hỏi tự nhiên: keyword/sparse + dense multilingual, hợp thứ hạng, rerank có context. Giữ metadata vị trí, phiên bản, loại mệnh đề và tình trạng thẩm định. Trả lời nêu nguồn, phạm vi, điều chưa biết; có thể trả không đủ bằng chứng. Vector similarity không là confidence chân lý.

Mặc định không đưa claim conflicted vào lời khẳng định; chế độ nghiên cứu có thể hiển thị cả hai lập luận với nhãn rõ. Bảo vệ chống prompt injection từ tài liệu: không coi trích sách như system/tool instruction.

## Đánh giá

Tách lỗi OCR, cây biến, truy xuất và lập luận. Đo Recall@k theo câu hỏi gold, tìm đúng position, đúng nguồn/nhánh, citation fidelity, nước hợp lệ và chất lượng giải thích do người duyệt. Giữ hard negatives chỉ khác một quân/lượt. Mọi lần đổi model/chunker/reranker cần so baseline trên tập giữ riêng, không dùng câu hỏi tự sinh rồi tự chấm làm chứng cứ duy nhất.
