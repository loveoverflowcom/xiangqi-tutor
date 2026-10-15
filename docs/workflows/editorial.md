# Hiệu đính, hỏi chuyên môn và cải thiện lý luận

## Phiếu nghi vấn

Mỗi phiếu gồm ID; nguồn/trang/crop; record+revision; thế/cây biến; điều chưa biết; giả thuyết; bằng chứng hiện có; câu hỏi cụ thể; người trả lời; phương án “chưa biết”; quyết định; các phụ thuộc phải tính lại. Dùng templates/review-question.md cho pilot, chưa là record CLI.

| Tình huống | Câu hỏi có ích |
| --- | --- |
| OCR mơ hồ | Đây là Xe hay Pháo? Vùng ảnh nào xác nhận? |
| Thiếu lượt/biến | Caption nói bên nào đi? Biến quay về node nào? |
| Thuật ngữ | “Tiên” ở đây nghĩa chủ động, đi trước hay lợi một nhịp? |
| Chỉ có lời khẳng định | Đối phương đang đe dọa gì? Nước này lấy đi lựa chọn nào? |
| Mệnh đề quá rộng | Thêm điều kiện nào? Thay một quân/lượt thì có phản ví dụ? |
| Nhiều nước tốt | Nước nào dễ thực hiện cho mức người học và vì sao? |
| Engine bất đồng | OCR/luật sai, thiếu history, khác budget hay quan điểm chuyên môn? |
| Claim duy nhất/luôn | Đã xét những phản ứng nào? Phần nào chưa được kiểm tra? |

## Chu kỳ hiệu đính

1. Xác minh data trước: ảnh → lượt → cây biến → ruleset/history.
2. Giữ lời tác giả, viết nhận định đối chiếu riêng.
3. Giải thích bằng mục tiêu, đe dọa, phản ứng mạnh, cơ chế; có phương án dễ nhầm và phản ví dụ.
4. Phân loại chỉnh sửa: OCR, ký pháp, bộ luật, engine run, ngữ nghĩa claim, giải thích hoặc sư phạm.
5. Tăng revision, chuyển về draft khi thay đổi nội dung; invalidation phụ thuộc. Người thẩm định duyệt rồi mới repin.
6. So lại tập chuẩn, publish index generation mới. Không tự học từ đầu ra chưa duyệt.

## Tận dụng feedback đúng nơi

Sửa quân → bộ gold/huấn luyện board recognition. Sửa chữ → OCR và từ điển. Sửa nhánh → parser/replay tests. Sửa lý luận → claim/explanation/evidence. Sửa độ khó → learner model. Các tập này liên kết bằng provenance nhưng không gộp thành một nhãn confidence trung bình.

Theo dõi độ tin OCR, đủ context, ổn định engine và duyệt chuyên môn riêng. Chưa có dữ liệu hiệu chỉnh thì không gọi số confidence là xác suất đúng. Người dùng “chưa biết” tạo nhiệm vụ nghiên cứu mới, không biến thành lời xác nhận.

## Quyết định cần chủ dự án bổ sung

Trình độ và mục tiêu học; luật thi đấu; ngôn ngữ ưu tiên; ấn bản nguồn; định nghĩa nhãn chiến thuật; sự khác nhau giữa đánh giá khách quan và dễ chơi; ngân sách engine; dữ liệu được phép đưa vào hệ thống công khai; nơi triển khai. Tiếp tục phần độc lập trong queue khi chưa có câu trả lời.
