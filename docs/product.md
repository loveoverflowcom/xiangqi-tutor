# Sản phẩm và giới hạn bootstrap

## Người dùng và mục tiêu

Chủ dự án có nhiều sách Việt/Trung/Anh, muốn xây gia sư qua ChatGPT kết hợp Pikafish và vector DB. Người dùng vừa học, vừa hiệu đính kiến thức chuyên môn; hệ thống phải ghi lại câu hỏi chưa rõ và sử dụng bản sửa ở lần giảng sau.

Một bài học hữu ích gồm thế cờ đúng, mục tiêu, nước ứng viên, phản ứng tốt của đối thủ, lời giải thích cơ chế, phương án tự nhiên dễ sai, ngoại lệ và nguồn truy ngược.

## Luồng cần xây

- Nhập một chương sách; OCR và tách hình; duyệt các vùng nghi vấn.
- Khôi phục cây biến; không ép tất cả nước thành một chuỗi.
- Đối chiếu nhận định với Pikafish, ưu tiên bất đồng cần thẩm định.
- Biên tập thẻ Markdown có phạm vi, ngoại lệ, bằng chứng và liên kết.
- Tìm đúng bài học theo câu hỏi hoặc thế cờ; trả lời có nguồn và mức xác minh.
- Giao bài, ghi lỗi của người học, ôn lại khi đã có lõi dữ liệu đáng tin.

## Hoàn thành bootstrap

Có tài liệu, hợp đồng, mẫu gốc, validator Rust và kế hoạch index có thể chạy offline. Không coi bootstrap là đã triển khai OCR, luật thi đấu đầy đủ, live engine, Lean, embedding, MCP hay hệ thống học thích nghi.

## Tiêu chí pilot

Chọn 2–3 sách, khoảng 100 trang đại diện. Đây là đề xuất kích thước pilot, không là ngưỡng bảo đảm chất lượng. Giữ riêng sách/chương đánh giá để tránh rò rỉ giữa bộ điều chỉnh và kiểm thử.

Đo: đúng toàn bộ bàn cờ+lượt; đúng cả nhánh biến; gắn đúng lời giảng với thế; lỗi đổi góc nhìn score; recall bài học; câu trả lời có nguồn; thời gian hiệu đính mỗi trang. Đặt ngưỡng sau khi có baseline do chủ dự án duyệt, không dùng accuracy tự quảng cáo của model thay số đo trên sách.
