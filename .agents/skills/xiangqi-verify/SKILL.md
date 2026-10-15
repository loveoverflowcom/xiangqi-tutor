---
name: "xiangqi-verify"
description: "Kiểm chứng dữ liệu và nhận định cờ tướng theo các cổng độc lập về ảnh nguồn, schema, replay, Pikafish và chứng nhận hình thức. Dùng khi cần đánh giá thế, nước đi, nhãn bằng chứng, phản bác hoặc proof; báo rõ phạm vi thực sự chạy và phần NOT_RUN."
---

# Kiểm chứng và ghi bằng chứng

## Phạm vi và tài liệu

Đọc [AGENTS](../../../AGENTS.md), [kiến trúc](../../../docs/architecture.md) và [workflow verification](../../../docs/workflows/verification.md).
Đối chiếu [claim contract](../../../docs/contracts/claims.md) và [Markdown v1](../../../docs/contracts/markdown-v1.md) khi đề xuất đổi nhãn.
Đọc [bằng chứng bootstrap](../../../docs/verification.md) để hiểu baseline; không dùng PASS cũ thay log của tác vụ hiện tại.
Với triển khai adapter, dùng [queue 020](../../../docs/work-plan/020-rules-and-pikafish.md); với Lean, đọc [backlog chứng nhận](../../../docs/work-plan/backlog/formal-certificates.md).

## Đầu vào và đầu ra

- Nhận record/revision/hash, nguồn/trang/hình, FEN, lượt, ruleset/history và phát biểu cần kiểm tra.
- Nhận artifact/run thực tế nếu có; kiểm tra provenance và input snapshot trước khi dùng lại.
- Trả báo cáo từng cổng, kết luận trong phạm vi chứng cứ, phản ví dụ/bất đồng và bước khắc phục.
- Ghi command/tool version, input snapshot, artifact/hash có thật và PASS/FAIL/NOT_RUN cho mỗi kiểm tra.
- Không tạo EngineRun, proof attestation hoặc nhãn kernel_checked giả trong schema hiện tại.

## Chọn cổng độc lập

| Cổng | Trạng thái bootstrap | Kết luận được phép |
| --- | --- | --- |
| Tương ứng ảnh | Duyệt thủ công khi đọc được nguồn | Quân/hướng/lượt khớp vùng ảnh đã xem |
| Schema/tham chiếu/snapshot/FEN | CLI đã triển khai | Dữ liệu đạt kiểm tra hình thức hiện có |
| Luật và replay | Adapter chưa triển khai | NOT_RUN nếu chưa có kiểm tra thực tế |
| Pikafish | Adapter chưa triển khai | NOT_RUN nếu chưa có engine run thực tế |
| Formal | Chưa có Lean project/checker | NOT_RUN nếu chưa có chứng nhận kiểm tra được |

Không dùng cổng trước thay cổng sau. FEN hợp cấu trúc chưa là thế hợp lệ; score/PV chưa là proof.
Nếu tác vụ cung cấp công cụ ngoài, xác minh khả năng/lệnh/version thực tế rồi báo riêng phạm vi đó; không gọi là tích hợp sẵn trong repo.

## Quy trình

1. **Pin đầu vào.** Đọc corpus hiện tại và snapshot dependency; kiểm tra bằng chứng có thuộc đúng bản hay không.
   Nếu nguồn/FEN/claim đã đổi, giữ run cũ làm lịch sử và đánh dấu không dùng cho bản mới trước khi tái kiểm.
2. **Chốt câu hỏi.** Phân biệt nhận dạng, nước hợp lệ, đánh giá ưu thế, nước duy nhất hay thắng cưỡng bức.
   Ghi scope, điều kiện, ngoại lệ và ruleset/history cần thiết. Không nâng claim cụ thể thành nguyên tắc phổ quát.
3. **Đối chiếu nguồn.** Xem ảnh và caption nếu có; xác định quân, hướng, lượt và node nhánh.
   Thiếu ảnh hoặc ambiguity thì ghi chưa xác minh, lập phiếu theo [mẫu](../../../templates/review-question.md) và chuyển xiangqi-ingest.
4. **Chạy CLI.** Validate corpus đủ dependency; đọc diagnostics trước khi làm phân tích tiếp.
   Chạy từ repo root với `corpus_dir` là đường dẫn thật của tác vụ.

   ```bash
   cargo run --locked -p xqt -- validate "$corpus_dir"
   ```

   CLI chưa xác minh bytes asset ngoài corpus, nội dung evidence artifact, legal replay hoặc history khớp final FEN.
5. **Kiểm tra luật khi có công cụ.** Chọn ruleset/version có căn cứ và replay từng nhánh từ đúng vị trí xuất phát.
   Ghi lỗi node/nước và đối chiếu diagram tiếp theo; không chọn ruleset để ép nước sách thành hợp lệ.
   Thiếu history phải hạn chế kết luận liên quan trường chiếu/trường tróc/hòa; không tự phục dựng lịch sử duy nhất từ FEN.
6. **Đối chiếu engine khi có run thật.** Ghi provenance theo hợp đồng EngineRun trong workflow canonical.
   Giữ raw output, binary/NNUE/options/budget, input history và kết quả timeout/error; chỉ dùng options engine thực sự quảng bá.
   Cache theo toàn bộ state/history/ruleset và cấu hình/run budget; không dùng mỗi FEN hoặc depth.
7. **Diễn giải đúng mức.** Giữ score kind, bounds và POV gốc; chuẩn hóa về Đỏ khi so trước/sau.
   CP không là số quân chắc chắn hay xác suất thắng; mate không tự đồng nghĩa chiếu hết trong mọi quy tắc.
   Xem nước sách và phản bác, nhiều PV/ngân sách khi cần; không suy “duy nhất” từ một PV hoặc hòa từ eval gần 0.
8. **Kiểm chứng hình thức khi có checker.** Audit phát biểu/model/ruleset và tiền đề trước khi đọc trạng thái build.
   Chiến lược cưỡng bức phải bao phủ mọi đáp án hợp lệ trong phạm vi; một PV không đủ.
   Không axiom hóa kết quả engine hoặc dùng sorry; nêu ranh giới native/external computation và dependency axioms.
   Giữ attestation riêng theo thiết kế tương lai; schema v1 không cho tác giả tự ghi kernel_checked.
9. **Đối chiếu claim.** Tách “nguồn nói C”, “run hỗ trợ C” và “người duyệt C”; ghi điều kiện và bất đồng.
   Không tự nâng supported/approved/proof theo kiểm tra cấu trúc. Chuyển sửa thẻ sang xiangqi-knowledge-edit.

## Khi kiểm tra không đủ

- Thiếu binary, weights, adapter hoặc checker: ghi NOT_RUN và phần phụ thuộc còn thiếu; không dùng output giả.
- Timeout hay engine lỗi: giữ lỗi và dữ liệu chạy thật; không báo PASS hoặc diễn giải output dở dang thành kết luận cuối.
- Hai nước tương đương: nêu phạm vi đánh giá và khác biệt sư phạm; không ép nhãn duy nhất.
- Kết quả bất đồng: kiểm lại data/rules/history/budget; giữ hồ sơ phản bác, không tự xóa nguồn cũ.
- Fixture chưa được kỳ thủ duyệt: gọi là candidate fixture, không gọi là gold đã xác nhận.
- Nhiều follow-up triển khai: chuyển xiangqi-work-item để cập nhật queue đúng dependency, không mở rộng hạ tầng tùy ý.

## Handoff

Trả bảng cổng với input revision/hash, phương pháp, kết quả và giới hạn; gắn artifact thực tế nếu có.
Nêu kết luận nào có bằng chứng, kết luận nào còn thiếu, record cần hiệu đính và kiểm tra cần chạy lại.
Giữ claim conflicted/challenged ngoài luồng giảng khẳng định mặc định theo [workflow indexing](../../../docs/workflows/indexing.md).
Không thông báo đã chạy OCR/Pikafish/Lean/Qdrant chỉ vì CLI hoặc CI cấu trúc thành công.
