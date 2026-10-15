# Cấu hình phát triển

`docker compose up -d qdrant` khởi động vector DB tùy chọn. Image pin v1.19.2, REST và gRPC chỉ bind loopback. Volume có tên giữ dữ liệu khi restart. Không dùng cấu hình dev này như triển khai Internet.

Chưa có adapter embedding hoặc Qdrant trong Rust. Dịch vụ không tự nạp index-plan. Không tạo collection với kích thước vector đoán trước; lựa chọn embedding profile và mapping point ID nằm trong queue 030.

Corpus thật ở private storage. `.env.example` là mẫu giá trị không bí mật. Không commit token, weights hay dữ liệu sách. Triển khai cloud/auth/quota/backup cần quyết định riêng khi có người dùng thật.
