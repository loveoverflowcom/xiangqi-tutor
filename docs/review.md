# Review thiết kế trước khi khởi tạo — 2026-10-15

Repository trống ở thời điểm kiểm tra; không có code hoặc quy ước cũ cần giữ. Các quyết định bên dưới dựa trên yêu cầu trong hội thoại và nguồn ở references.md.

| Điểm cần làm rõ | Quyết định |
| --- | --- |
| Lean có quá cứng cho sách? | Thẻ mệnh đề mềm ở lớp biên tập; Lean chỉ kiểm tra tập con có ngữ nghĩa rõ. |
| JSON/YAML là đủ để suy luận? | JSON block có kiểu cho máy, Markdown cho người. Atom hiện chỉ mô tả; chưa là logic engine. |
| Supported có đồng nghĩa đúng? | Không. Đây là tình trạng bằng chứng do biên tập khai báo, độc lập với chứng minh. |
| Thế OCR hợp lệ có phải đọc đúng? | Không. Luật không chứng minh sự tương ứng với ảnh gốc. |
| FEN đủ cho mọi kết luận? | Không. Giữ history/ruleset; thiếu lịch sử phải nêu phạm vi chưa xác minh. |
| Một PV có chứng minh thắng cưỡng bức? | Không. Chứng nhận chiến lược cần bao phủ mọi đáp án đối thủ trong phạm vi đặc tả. |
| Có nên ép engine thành nguồn chân lý? | Không. Lưu run provenance, budget, score kind và POV; kết luận chỉ trong phạm vi kiểm tra. |
| Mọi liên kết có nên là DAG? | Không. DAG dành cho phụ thuộc dẫn xuất có snapshot; quan hệ supports/challenges có thể tạo chu trình. |
| Chỉnh thẳng vector để sửa tri thức? | Không. Sửa Markdown, duyệt, invalidation, sinh lại chỉ mục. |
| Hash đủ chứng minh nguồn đúng? | Không. Hash chỉ giúp nhận diện bytes; importer còn phải xác minh asset thực và mapping trang. |
| Tự động gán nhãn rồi tự đánh giá cùng dữ liệu? | Không. Giữ bộ chuẩn độc lập, split theo sách/ấn bản và người duyệt. |

## Chưa chốt

- Bộ luật và phiên bản cho long-check/chase, hòa và giới hạn nước; cần nguồn WXF/luật Việt hoặc Trung và adapter chứng minh tương thích.
- Embedding/reranker và tokenizer Việt–Trung. Chọn bằng bộ câu hỏi thật, không theo benchmark tổng quát.
- Triển khai local hay VPS, GPU, dung lượng, quyền sử dụng từng nguồn.
- Mức cờ và mục tiêu học của chủ dự án; định nghĩa độ khó và nhãn chiến thuật.
- OCR model nào tốt trên hình sách đen trắng; các repo ảnh thực chưa được kiểm tra trên tài liệu của người dùng.

Các điểm này không chặn validator/index-plan. Chúng chặn việc quảng bá đã có gia sư, kết luận luật đầy đủ hoặc proof đã kiểm tra.
