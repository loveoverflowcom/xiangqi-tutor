# Đóng góp

Nhánh chính cho phát triển là `develop`. Thay đổi sau bootstrap dùng nhánh `feat/...` hoặc `fix/...` và PR về `develop`.

## Quy trình sửa tri thức

1. Xác định ID và nguồn trang/hình. Giữ nguyên nguyên văn trong bản nguồn riêng.
2. Tạo/sửa bản ghi; tăng revision nếu đã được dùng. Chuyển duyệt về draft khi nội dung thay đổi.
3. Ghi lý do sửa, người sửa, phần nào của kết luận bị ảnh hưởng. Không dùng Git author làm chứng cứ kỳ thủ.
4. Chạy `xqt validate`. Nếu stale, xem lại toàn bộ phụ thuộc trước khi cập nhật revision/hash.
5. Duyệt ảnh/quân/lượt đi; replay và engine ở bước tích hợp tương ứng. Các bước chưa chạy ghi NOT_RUN.
6. Chạy index-plan; xem danh sách excluded. Không đổi nhãn supported chỉ để được xuất.
7. PR nêu hành vi trước/sau, nguồn, kiểm tra thực hiện và phần chưa xác minh.

## Nguồn dữ liệu

Sách người dùng lưu dưới `private/` hoặc kho riêng ngoài repo. `.gitignore` chỉ là hỗ trợ, không thay việc rà soát diff. Corpus mẫu công khai chỉ có nội dung do dự án tự soạn.

## Lệnh kiểm tra

Dùng các lệnh ở README. Commit Cargo.lock. Không chạy pipeline network trong unit test; dùng fixture offline. Adapter live phải có integration test riêng với trạng thái rõ ràng.
