# Nhập sách, làm sạch và OCR

Status: quy trình thiết kế; chưa triển khai importer/OCR. Bắt đầu bằng pilot nhỏ ở product.md.

Công cụ độc lập [MinerU local](mineru-local.md) đã được cài và smoke test trên PDF tự soạn
ngày 2026-10-15, có hướng dẫn khắc phục lỗi tải và kết quả thực tế. Đây là bước chuẩn bị;
chưa có adapter nối đầu ra MinerU vào thẻ `xqt`, chưa thử sách thật và còn lỗi dấu tiếng Việt.

## 1. Intake

Đăng ký sách/ấn bản/ngôn ngữ/quyền truy cập. Giữ raw file bất biến, SHA-256, tổng trang, ngày nhập, số trang PDF và số trang in riêng. Phát hiện trùng bytes và gần trùng trang; bản scan khác nhau vẫn có provenance riêng. Không bỏ lời bình khác nhau chỉ vì cùng thế cờ.

Tài sản cá nhân ở private storage. Không tải sách lên dịch vụ ngoài nếu cấu hình nguồn chưa cho phép. Lưu job manifest, tool/model/config version và trạng thái từng trang để có thể tiếp tục sau lỗi. Các đường dẫn cụ thể của adapter chưa là API đã tồn tại.

## 2. Text-first và phân tích bố cục

PDF có text: trích xuất layer và kiểm tra thứ tự đọc trước. Trang scan: render bản làm việc (thử 300 DPI rồi đo), deskew/crop có lưu transform; không ghi đè raw. Chỉ làm sạch ảnh vừa đủ, giữ ảnh trước/sau để phát hiện mất dấu hoặc chữ.

PaddleOCR là ứng viên chữ/bố cục. Tách title, prose, nước đi, chú thích, số trang, hình cờ; nối đoạn qua trang nhưng giữ mapping mọi đoạn về page+region. OCR phải lưu text thô, confidence và bbox riêng; confidence không là xác suất đoạn đúng.

## 3. Hình cờ

Crop hình → tìm hướng và lưới 10×9 → phân loại 90 giao điểm → candidate positions. Thử TheOne1006 hoặc RakuXQ; đo lại trên hình sách đen trắng, chữ quân lạ, scan lệch/mờ. Model ảnh thực không được mặc định tốt trên sách. Ảnh có vùng khuất/cắt phải ghi unknown, không biến vùng không thấy thành ô trống.

Lượt đi lấy từ caption hoặc cây nước; không suy từ vị trí Tướng trên ảnh. Nếu không có, đưa staging và hỏi người. Giữ ảnh gốc, crop, transform, top candidates, confidence từng quân và bản dựng lại để duyệt cạnh nhau.

Không cho engine chọn thế “đẹp” nhất rồi coi là OCR đúng. Rule checks chỉ gợi ý điểm nghi vấn. Những lỗi sửa của người là nhãn cho board recognizer, không tự trở thành training text của gia sư.

## 4. Chuẩn hóa văn bản và ký pháp

Tách raw/corrected/editorial. Chuẩn hóa Unicode và thuật ngữ theo từ điển có phiên bản. Giữ giản/phồn thể gốc, thêm alias; không thay chữ toàn bộ rồi làm mất trích dẫn. Nhận diện số cột, tiền/hậu, tiến/thoái/bình theo bên nhìn và thế thực tế. Parser phải báo ambiguity thay vì chọn nước engine thích.

## 5. Khôi phục cây biến

Gắn mỗi hình, đoạn và nước với node cụ thể. Mỗi nhánh ghi parent/start position, move, lời tác giả, nguồn. Các dấu “nếu…”, “biến khác”, đánh số quay lại không là chuỗi chính. Replay từng nhánh bằng bộ luật đã chọn; đối chiếu diagram kế tiếp. Lỗi phải trả về vùng sách cần sửa.

## 6. Sinh Markdown và thẩm định

Sinh source/position/claim draft theo hợp đồng. Statement giữ nhận định của tác giả; lời giảng hiện đại ở phần editorial, không tự sửa nguyên văn. Claim có điều kiện, phạm vi, ngoại lệ và câu hỏi. Chỉ sau review mới ghi approved; supported cần có evidence link nhưng chưa phải theorem.

## 7. Publish

Validator → cổng replay/engine (khi đã triển khai) → người duyệt → kế hoạch index → embedding staging → kiểm tra → đổi generation đang phục vụ. Không tự động publish đầu ra OCR hay LLM.

## Sự cố và khôi phục

Job retry idempotent theo asset hash + page + adapter/model/config. Timeout không ghi success. Sai OCR làm mất hiệu lực FEN, biến, phân tích và claim phụ thuộc. Lưu lỗi ở cấp trang để tiếp tục phần khác; corpus publish cuối cùng không được chứa tham chiếu stale. Cơ chế job/retry còn trong queue 010.
