# Tạo thẻ position

Sao chép [mẫu hoàn chỉnh](../knowledge/examples/position-start.md) vào corpus riêng rồi chỉnh các trường và prose. Không nhập thư mục templates vào CLI.

1. Cấp ID ổn định và title; revision bắt đầu 1.
2. Review bắt đầu draft, by null, note giải thích phần còn thiếu.
3. Pin các dependency bằng ID/revision/SHA-256 bytes thực sau khi đọc nội dung.
4. Giữ nguồn/trang/crop, lượt/ruleset/history chính xác; thiếu thì giữ staging.
5. Chỉnh mệnh đề, điều kiện, ngoại lệ và lý luận độc lập với nguyên văn.
6. Chạy validate; duyệt chuyên môn trước khi approved/supported.

Schema thực thi nằm ở [record.rs](../crates/knowledge/src/record.rs). Không thêm nhãn kernel_checked; xem [claim contract](../docs/contracts/claims.md).
