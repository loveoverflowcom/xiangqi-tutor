# Gia sư trong ChatGPT và vòng hiệu đính

## Why

Người dùng cần hỏi bài, luyện tập và sửa lý luận ngay trong hội thoại.

## Scope

Tool contract tìm bài/thế, phân tích, trả nguồn+mức xác minh, ghi phiếu hiệu đính; learner state và review bài.

## Non-goals

Không tự tạo front-end riêng; không ghi đè corpus bằng output chưa duyệt.

## Dependencies

030, engine adapter 020, quyết định local/VPS/auth và trình độ người học.

## Suggested sequence

Read-only explain trước; bài tập/gợi ý tăng dần; lưu lỗi; bản sửa đi qua draft/review; lịch ôn dựa dữ liệu người học.

## Review boundary

Một phiên có thể giải thích, từ chối khi thiếu nguồn, tạo bài cùng mục tiêu, nhận sửa và truy xuất revision mới.

## Risks / unknowns

Không biến lỗi engine/OCR thành điểm yếu người học; không lộ đáp án khi đang giao bài.

## Follow-ups

FSRS và adaptive curriculum sau dữ liệu thật.
