---
name: "xiangqi-work-item"
description: "Điều phối một công việc Xiangqi Tutor từ issue hoặc docs/work-plan tới thay đổi có thể review. Dùng khi được yêu cầu bắt đầu, tiếp tục, triển khai hoặc hoàn tất feature, bug, pilot hay work item; đọc phạm vi, chọn skill chủ sở hữu, giữ ledger bằng chứng và chuẩn bị PR về develop."
---

# Xiangqi work item

## Chốt đầu vào và ranh giới

1. Đọc [AGENTS](../../../AGENTS.md), [CONTRIBUTING](../../../CONTRIBUTING.md),
   [review thiết kế](../../../docs/review.md), [queue](../../../docs/work-plan/README.md)
   và item/issue được yêu cầu. Kiểm tra code và test đang tồn tại trước khi tin tên adapter trong kế hoạch.
2. Ghi base SHA, `git status --short`, đầu ra người dùng cần và tiêu chí chấp nhận.
   Giữ nguyên thay đổi ngoài phạm vi; dùng checkout/worktree riêng khi cần. Không tự stash/reset.
3. Chọn một kết quả có thể review, ví dụ “từ caption xác định lượt đi và sinh draft có nguồn”.
   Tách phát hiện mới thành follow-up khi không cần để kết quả này đúng.
4. Xác định phụ thuộc thực tế. Nếu thiếu sách, ruleset hoặc live service, tiếp tục phần độc lập;
   ghi phần chặn, điều cần bổ sung và phần chưa chạy. Không tự tạo dữ kiện để vượt cổng.
5. Với thay đổi repository, tạo `feat/...` hoặc `fix/...` từ `develop` mới đọc.
   Với yêu cầu chỉ review/tư vấn, giữ thao tác đọc. Không đổi default branch hoặc merge ngoài yêu cầu.

## Chọn chủ sở hữu

Đọc [bản đồ skills](../README.md) rồi nạp skill phù hợp; không nạp toàn bộ references.

| Phạm vi | Skill cần đọc |
| --- | --- |
| Rust, parser, CLI, adapter và mọi logic mới | [xiangqi-engineering](../xiangqi-engineering/SKILL.md) |
| Sách, OCR chữ/hình, mapping trang, cây biến | [xiangqi-ingest](../xiangqi-ingest/SKILL.md) |
| Sửa mệnh đề, điều kiện, ngoại lệ, evidence, snapshot | [xiangqi-knowledge-edit](../xiangqi-knowledge-edit/SKILL.md) |
| Luật, replay, Pikafish, Lean và phạm vi kết luận | [xiangqi-verify](../xiangqi-verify/SKILL.md) |
| Eligibility, embedding, Qdrant, retrieval | [xiangqi-index](../xiangqi-index/SKILL.md) |
| Lời giảng, bài tập, feedback của người học | [xiangqi-tutor](../xiangqi-tutor/SKILL.md) |
| Đánh giá diff hoàn tất | [xiangqi-code-review](../xiangqi-code-review/SKILL.md) |

Skill này chỉ sở hữu thứ tự giao việc; để contract, workflow và code sở hữu quy tắc sản phẩm.
Nếu chúng bất đồng, ghi rõ hai nguồn và giải quyết trong phạm vi tác vụ; không sửa tài liệu để che lỗi code.

## Thực hiện và ghi bằng chứng

Trước khi sửa, lập ledger ngắn trong ghi chú công việc hoặc PR:

| Tiêu chí / invariant | Nguồn | Lỗi cần ngăn | Oracle độc lập rẻ nhất | Kết quả / phạm vi |
| --- | --- | --- | --- | --- |
| Ví dụ: đổi prose làm snapshot cũ stale | Markdown v1 | phục vụ lý luận cũ | fixture sửa một byte | chưa chạy |

1. Thực hiện logic thuần và kiểm tra ranh giới trước IO theo engineering; tái dùng schema/template hiện có.
2. Chạy cổng phù hợp với thay đổi, ghi lệnh, đầu vào và output thật.
   Phân biệt `PASS`, `FAIL`, `NOT_RUN` và `UNAVAILABLE`; phân biệt fake với dịch vụ thật.
3. Với cổng lỗi, phân loại regression, lỗi có sẵn hay môi trường trước khi sửa.
   Không bỏ kiểm tra hoặc đổi nhãn tri thức để làm kết quả xanh.
4. Cập nhật tài liệu chủ sở hữu khi hành vi/trạng thái thay đổi. Giữ queue gọn theo quy ước hiện có;
   không dựng thêm status board hay tự mở issue/comment từ follow-up.
5. Review diff bằng code-review; kiểm tra nguồn sách, secret và artifact riêng tư trước khi stage từng path.
   Đưa thay đổi đã tạo vào Git theo yêu cầu và quyền trong hội thoại; PR nhắm `develop`.
   Không hỏi lại quyền đã có, không tự gửi thông điệp cho người khác hoặc merge khi chưa được yêu cầu.

## Bàn giao

Nêu kết quả người dùng nhận được, phạm vi, nhánh/base, đường dẫn hoặc PR, các tiêu chí đã đáp ứng,
kiểm tra thực sự chạy, live/fake, giới hạn và follow-up. Khi một cổng chưa chạy, ghi lý do,
đầu vào/công cụ còn thiếu, cách chạy lại và phần kết luận chưa được phép khẳng định.

Không coi “có skill mô tả OCR/Pikafish” là “đã tích hợp OCR/Pikafish”.
