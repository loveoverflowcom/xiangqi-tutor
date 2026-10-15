---
name: "xiangqi-knowledge-edit"
description: "Hiệu đính thẻ Markdown Xiangqi Tutor, điều kiện và ngoại lệ của mệnh đề, phản ví dụ, lý luận hoặc phản hồi chuyên môn. Dùng khi sửa OCR đã nhập, sửa thế cờ hoặc đổi nhận định; quản lý revision, dependency stale và handoff kiểm chứng trước khi tái index."
---

# Hiệu đính tri thức có phiên bản

## Phạm vi và tài liệu

Đọc [AGENTS](../../../AGENTS.md), [CONTRIBUTING](../../../CONTRIBUTING.md) và [quy trình editorial](../../../docs/workflows/editorial.md).
Khi sửa trường dữ liệu, đối chiếu [Markdown v1](../../../docs/contracts/markdown-v1.md) với [schema Rust](../../../crates/knowledge/src/record.rs).
Khi đổi lập luận, đọc [hợp đồng claim](../../../docs/contracts/claims.md); dùng [phiếu nghi vấn](../../../templates/review-question.md) cho phần chưa rõ.
Giữ hợp đồng tại tài liệu/code canonical; không sao chép schema riêng trong skill.

Validator và kế hoạch index đã có. Replay, Pikafish, Lean, embedding và cơ chế invalidation live chưa triển khai.
Chỉ ghi kiểm tra đã thực hiện; giữ NOT_RUN cho phần thiếu và không diễn giải metadata thành attestation.

## Đầu vào và đầu ra

- Nhận ID/đường dẫn thẻ, revision/hash hiện tại, nguồn/trang liên quan và yêu cầu chỉnh sửa.
- Nhận bằng chứng mới hoặc câu trả lời chuyên môn nếu có; phân biệt dữ liệu được cung cấp với suy luận của người sửa.
- Trả diff thẻ, lý do thay đổi, revision mới và danh sách dependency/cache/index cần xem lại.
- Trả phiếu chưa giải quyết, kết quả validator và phạm vi kiểm chứng tiếp theo.
- Không sửa tri thức chỉ trong Qdrant; sửa Markdown canonical rồi sinh lại dẫn xuất.

## Quy trình

1. **Đọc đúng bản.** Mở thẻ hiện tại, dependency trực tiếp và nguồn cần thiết; ghi snapshot trước sửa.
   Xem lịch sử khi cần hiểu ý nghĩa thay đổi; không dùng Git author làm chứng cứ kỳ thủ đã duyệt.
2. **Phân loại lỗi.** Xác định OCR chữ/quân, ký pháp/cây biến, ruleset/history, engine, ngữ nghĩa hay sư phạm.
   Sai dữ liệu đầu vào phải sửa trước khi đánh giá lý luận. Thiếu ảnh chuyển xiangqi-ingest.
3. **Giữ lời nguồn.** Không ghi đè raw hoặc biến lời tác giả thành lời giải của hệ thống.
   Phân biệt “sách nói C” với “C đúng”; đặt nhận định đối chiếu ở phần biên tập.
4. **Làm rõ mệnh đề.** Sửa statement canonical trong JSON; không tạo bản statement cạnh tranh trong prose.
   Chốt scope, điều kiện, ngoại lệ đã biết và kết luận theo contract. Atom vẫn chỉ mô tả, chưa được thực thi.
   Với “luôn”, “duy nhất”, “bắt buộc”, nêu lượng từ/phạm vi và phần phản ứng chưa xét.
5. **Sửa lời giảng.** Giữ các H2 bắt buộc theo hợp đồng; giải thích mục tiêu, đe dọa và phản ứng mạnh của đối thủ.
   Thêm phương án dễ nhầm hoặc phản ví dụ có nguồn. Không dùng việc chưa tìm thấy ngoại lệ như chứng minh.
6. **Cập nhật trạng thái trung thực.** Tăng revision cho bản sửa đã được dùng; nội dung thay đổi cần về draft để xem lại.
   Ghi người thực sự duyệt và lý do khi có review; không tạo danh tính hoặc lượt duyệt chưa xảy ra.
   Tách review, evidence_status và formal_status; supported không đồng nghĩa proof hay engine đã chạy.
7. **Xem tác động.** Lần theo dependency DAG để tìm position/claim và kết quả dẫn xuất đang pin bản cũ.
   Hash bao gồm toàn bộ bytes Markdown, kể cả prose/newline; thay metadata cũng có thể làm stale.
   Giữ lỗi stale hiện ra cho đến khi từng consumer được xem lại; không repin tự động chỉ để xanh.
8. **Repin có căn cứ.** Xem lại đầu vào và kết luận từ dependency gốc lên consumer.
   Sau khi chấp nhận nội dung mới, tính hash bytes cuối cùng, cập nhật revision/hash đúng snapshot và kiểm tra tiếp chuỗi phụ thuộc.
   Không tái dùng revision cũ cho bản sửa đã xuất bản hoặc viện dẫn hash thẻ thay hash asset.
9. **Giữ tranh luận.** Dùng relations cho supports/challenges/refines/supersedes theo nghĩa đã định.
   Nếu dùng nội dung claim khác làm tiền đề thì pin dependency riêng; quan hệ tranh luận không tự là chứng minh.
   Khi thay thế record cũ, chuyển review bản cũ về rejected và ghi lý do; supersedes một mình chưa loại khỏi index.
   Việc rejected cũng đổi bytes/hash và làm consumer pin bản cũ mất eligibility. Nếu bản thay thế dùng bản cũ làm tiền đề,
   phải xem lại lineage và căn cứ thật; không xóa dependency cần thiết chỉ để bản thay thế được xuất.
10. **Kiểm tra và bàn giao.** Chạy validator trên corpus đủ dependency; liệt kê record chưa thể cập nhật.
    Chuyển kết luận cờ sang xiangqi-verify và kế hoạch tái xuất sang xiangqi-index.

## Kiểm tra thực tế

Chạy từ repo root với `corpus_dir` trỏ đến corpus đã sửa.

```bash
cargo run --locked -p xqt -- validate "$corpus_dir"
```

Lưu command, snapshot input, diagnostics và kết quả thật. CLI chỉ kiểm schema/tham chiếu/snapshot và cấu trúc FEN.
Nếu sửa schema/code, dùng xiangqi-engineering để kiểm thử ranh giới tin cậy và cập nhật contract cùng thay đổi.
Không dùng smoke test corpus ví dụ để kết luận thẻ người dùng đã hợp lệ.
Validator đọc toàn corpus trước index-plan: draft/rejected vẫn phải có snapshot hợp lệ;
đổi review không tự chữa stale của record hoặc consumer.

## Xử lý bất đồng và thiếu dữ liệu

- Người trả lời chưa biết: giữ câu hỏi mở; không chuyển thành xác nhận hoặc bắt người dùng đoán.
- Engine bất đồng: kiểm tra ảnh/lượt/nhánh/ruleset/history trước; giữ hai lập luận và run có thật.
- Bằng chứng không đủ: giữ unassessed/challenged/conflicted theo dữ liệu; không đổi supported để được index.
- Artifact evidence không đọc được: nói chưa xác minh nội dung; hash đúng định dạng không đủ xác nhận.
- Thiếu consumer phụ thuộc trong corpus: ghi phạm vi chưa kiểm và không tuyên bố đã tái kiểm chứng toàn bộ.
- Nguồn sai hoặc thiếu quyền công khai: sửa phạm vi dữ liệu, không xóa provenance để vượt bộ lọc.

## Handoff

Ghi record/revision/hash trước và sau, lý do sửa, bằng chứng nguồn và người/công cụ đã thực sự tham gia.
Nêu consumer đã xem lại, consumer còn stale, kiểm tra PASS/FAIL/NOT_RUN và câu hỏi cụ thể tiếp theo.
Đánh dấu run/index generation cũ bị ảnh hưởng; chưa có hạ tầng live thì mô tả việc cần làm thay vì báo đã invalidated.
Chỉ bàn giao index khi metadata review/evidence đáp ứng hợp đồng; không cần một luồng xác nhận ngoài phạm vi review đã có.
