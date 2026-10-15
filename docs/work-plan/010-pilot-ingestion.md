# Pilot nhập một chương có provenance và duyệt OCR

## Why

Cần biết tỷ lệ sai và chi phí sửa trên sách thật trước khi chọn model.

## Scope

Importer giữ hash raw, mapping trang/crop, text layer/OCR, candidate boards, câu hỏi lượt đi; sinh thẻ draft và báo cáo lỗi theo trang.

## Non-goals

Chưa nhập toàn thư viện; không tự động phê duyệt hoặc giả lập board recognition.

## Dependencies

Chủ dự án chọn 2–3 nguồn đại diện, quyền truy cập, gold split; đọc workflow ingestion.

## Suggested sequence

Bắt đầu text layer; benchmark OCR chữ và board recognizer; giữ top candidates; người duyệt ghép cây biến.

## Review boundary

Một chương truy về đúng trang/hình, retry không tạo trùng, unknown không bị điền mặc định; kiểm tra asset hash thực.

## Risks / unknowns

Domain shift scan đen trắng; thiếu model weights; ký pháp và branch alignment.

## Follow-ups

Bổ sung type/schema theo dữ liệu pilot, migration có fixture.
