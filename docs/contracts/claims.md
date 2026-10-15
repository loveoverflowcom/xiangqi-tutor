# Mệnh đề, lý luận và ký hiệu

## Phân loại

Observation là phát biểu về dữ liệu/thế cụ thể; rule là luật được đặc tả; heuristic là khuyến nghị có ngoại lệ; engine_assessment là kết quả đánh giá theo run; hypothesis là nhận định chờ kiểm tra. Loại phát biểu không quyết định trạng thái bằng chứng hoặc formal status.

Ví dụ “sách S khẳng định C” và C là hai mệnh đề khác nhau. Giữ lời tác giả nguyên trạng, thêm nhận định đối chiếu. Không dùng supported như boolean chân lý.

## Ký hiệu trình bày

| Ký hiệu | Nghĩa |
| --- | --- |
| `P ∧ Q → R` | Kéo theo chặt trong mô hình đã định nghĩa |
| `P ∧ Q ⇝ R` | Khuyến nghị có thể bị điều kiện ngoại lệ đánh bại; quy ước dự án, không phải cú pháp Lean |
| `Supports(E,C)` | E hỗ trợ C trong phạm vi đã ghi |
| `Challenges(E,C)` | E nêu phản bác hoặc bất đồng với C |
| `Γ ⊢ C` | Có chứng minh hình thức từ Γ theo bộ kiểm tra đã nêu |

Không tìm thấy phản ví dụ ≠ đã chứng minh. Không truy xuất thấy ngoại lệ ≠ biết ngoại lệ không tồn tại. Danh sách exceptions mặc định là **chưa đầy đủ**.

## Cấu trúc cho máy

Atom `{predicate,args}` chỉ biểu diễn quan hệ có tên. Conditions là danh sách điều kiện cùng cần xét; exceptions là ngoại lệ đã biết; conclusion là điều được phát biểu. v1 **không thực thi** atom hay suy ra kết luận từ việc thiếu exception. Predicate cần đăng ký ngữ nghĩa, kiểu tham số và evaluator trước khi chạy tự động. Null/missing input không được tự chuyển thành false.

Mỗi claim có scope position/pattern/universal. Universal phải viết rõ lượng từ và giới hạn; không nâng từ một tập ví dụ lên toàn bộ thế cờ. Bản giảng nên nêu mục tiêu, đe dọa, phản ứng mạnh của đối thủ, cơ chế và phương án dễ nhầm.

## Lean

Lean là lớp kiểm tra tập con: luật, invariant và chứng nhận chiến thuật hữu hạn. Kernel kiểm chứng phát biểu hình thức so với định nghĩa/tiền đề, không kiểm chứng sự trung thành của bản dịch từ sách.

Pikafish sinh ứng viên; bộ luật kiểm tra diễn biến; một checker nhỏ có thể kiểm tra chứng nhận. Một PV đơn lẻ không phủ các đáp án đối thủ. Không `axiom` hóa kết quả engine, không chấp nhận `sorry` rồi gắn đã chứng minh. Checker tương lai cần ghi theorem digest, model/ruleset digest, toolchain, lệnh, output, dependency axioms và hash artifact; audit cả ngữ nghĩa phát biểu. Hiện không có Lean project hay proof đã chạy.

## Tín hiệu cần hỏi người

Mệnh đề thiếu điều kiện; từ “duy nhất/luôn/bắt buộc” chưa đủ bằng chứng; engine bất đồng; nhiều nước tương đương nhưng khác độ dễ chơi; thuật ngữ khác nghĩa theo sách; thiếu lịch sử/bộ luật. Cho phép trả lời chưa biết và biến nó thành bài nghiên cứu, không yêu cầu người dùng đoán.
