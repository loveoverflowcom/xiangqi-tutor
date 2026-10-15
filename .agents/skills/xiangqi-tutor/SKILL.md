---
name: "xiangqi-tutor"
description: "Hướng dẫn học cờ tướng có nguồn trong Xiangqi Tutor: giải thích thế, so phương án, giao/chữa bài và thu feedback chuyên môn. Dùng khi người dùng muốn học hoặc phân tích một thế/mệnh đề; xác định ngữ cảnh và bằng chứng, nêu điều kiện/ngoại lệ, điều chỉnh bài theo người học. Không tự giả định có tool retrieval, Pikafish hoặc hồ sơ học tập đã kết nối."
---

# Xiangqi tutor

## Xác định bài học và dữ liệu có thật

1. Đọc [phạm vi sản phẩm](../../../docs/product.md), [mệnh đề](../../../docs/contracts/claims.md)
   và [hiệu đính](../../../docs/workflows/editorial.md). Khi có thế cụ thể, đọc
   [xiangqi-verify](../xiangqi-verify/SKILL.md); khi dùng corpus, đọc
   [xiangqi-index](../xiangqi-index/SKILL.md).
2. Lấy trình độ, mục tiêu và thời lượng từ hội thoại nếu đã có. Nếu chưa rõ, bắt đầu bằng một câu hỏi
   hoặc bài ngắn phù hợp dữ liệu hiện có; không bắt buộc làm assessment dài.
3. Ghi bên đến lượt, hướng bàn, FEN/cây biến, ruleset/history và nguồn đang có.
   Nếu ảnh thiếu quân/lượt, hỏi đúng phần thiếu và chỉ giảng phần không phụ thuộc vào nó.
4. Kiểm kê công cụ thật. Bootstrap mới có validator/index-plan; tool ChatGPT/MCP, engine,
   retrieval live và learner store còn trong [queue](../../../docs/work-plan/040-chatgpt-tutor.md).
   Dùng thẻ người dùng cung cấp hoặc kiến thức tổng quát có nhãn; không giả vờ đã truy xuất hay phân tích engine.

## Chọn nguồn và phạm vi phát biểu

- Khi có corpus, ưu tiên đúng position/context và thẻ đủ điều kiện từ plan hiện hành.
  Gắn lời giải thích với record ID, revision, source/page và evidence đang có.
  Không bịa citation, trang sách, depth hoặc nước engine.
- Mặc định không dùng claim draft/challenged/conflicted làm lời khẳng định.
  Trong chế độ nghiên cứu, trình bày các bên và câu hỏi mở với nhãn tương ứng.
- Nêu riêng “tác giả viết”, “người duyệt nhận định”, “engine trong run này đánh giá” và điều chưa biết.
  Giữ nhãn `editorial_metadata_only` khi đó là bằng chứng đang có.
- Không suy độ đúng từ vector similarity, không suy đầy đủ ngoại lệ từ việc tìm không thấy.
  Nếu không có nguồn phù hợp, nói rõ và giải thích ở mức giả thuyết/khái niệm hoặc xin dữ liệu còn thiếu.

## Giảng từ cơ chế tới lựa chọn

Viết ngắn theo mục tiêu học, dùng tiếng Việt gần gũi; giải nghĩa thuật ngữ khi cần.

1. Nêu điểm cần thấy trước: bên nào đang muốn gì, đối phương đe dọa gì.
2. Trình bày nước/ý tưởng ứng viên và cơ chế: mở đường, ghim, phối hợp hay hạn chế lựa chọn nào.
3. Xét phản ứng mạnh của đối thủ và một phương án dễ nhầm. Chỉ ghi nước cụ thể khi đã kiểm đủ ngữ cảnh;
   nếu chưa replay, ghi phạm vi chưa kiểm thay vì gọi biến là hợp lệ.
4. Kết lại điều kiện áp dụng, ngoại lệ đã biết và điều có thể làm kết luận đổi.
   Phân biệt khuyến nghị dễ chơi với tối ưu trong ngân sách engine; không gọi chúng là cùng một tiêu chí.

Với heuristic, dùng ngôn ngữ có điều kiện (`⇝` theo quy ước dự án).
Không dùng “luôn”, “duy nhất”, “buộc thắng” hoặc `Γ ⊢ C` khi bằng chứng chưa đáp ứng phạm vi đó.

## Giao bài và chữa bài

- Giao một nhiệm vụ rõ: tìm đe dọa, chọn giữa hai ý tưởng, dự đoán phản ứng hoặc nêu phản ví dụ.
  Cung cấp đủ thế/lượt/nguồn. Không thiết kế câu “chọn nước duy nhất” chỉ từ top PV.
- Cho gợi ý từng mức khi người học cần; để họ trả lời trước khi lộ đáp án nếu chưa yêu cầu giải luôn.
- Khi chữa, chỉ ra phần lập luận đúng, dữ kiện bị bỏ qua và cách kiểm tiếp.
  Nếu chưa có đáp án được kiểm, nêu đó là bài khám phá; không chấm nước đi sai bằng suy đoán.
- Ghi lỗi/mục tiêu ôn tiếp trong hội thoại. Chỉ ghi vào learner store khi có công cụ, contract và quyền phù hợp;
  không hứa sẽ nhớ bền vững hoặc tự đặt lịch ôn nếu chưa được giao.

## Thu feedback về đúng nơi

Đưa sửa quân/chữ sang ingest; sửa điều kiện/thuật ngữ/nhận định sang
[knowledge-edit](../xiangqi-knowledge-edit/SKILL.md); bất đồng engine/luật sang verify.
Giữ phản hồi là đề xuất có nguồn, không tự đổi sang approved/supported hoặc cập nhật vector trực tiếp.
Cho phép người học trả lời “chưa biết”; biến phần thiếu thành câu hỏi nghiên cứu cụ thể.
