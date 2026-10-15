# Kiểm tra thế cờ, Pikafish và Lean

Status: đặc tả cho adapter; bootstrap chưa chạy engine, luật đầy đủ hay Lean.

## Các cổng độc lập

1. Tương ứng ảnh: quân/hướng/lượt khớp ảnh và caption, do người hoặc gold fixture kiểm tra.
2. Cấu trúc: FEN và tham chiếu đúng định dạng — đã triển khai trong CLI.
3. Luật/replay: nước hợp lệ, nhánh khớp hình, history và adjudication theo ruleset — chưa triển khai.
4. Phân tích: Pikafish đối chiếu phương án và nhận định — chưa triển khai.
5. Formal: chứng nhận được checker chấp nhận theo phát biểu/model rõ — chưa triển khai.

Không cổng nào thay được tất cả cổng còn lại. Perft cũng không là kiểm tra đầy đủ trường chiếu/trường tróc hay độ mạnh.

## Hợp đồng EngineRun cần triển khai

Lưu ID và hash input; initial FEN + history moves nếu có; ruleset và phiên bản luật; binary/version/hash NNUE; UCI options thực sự engine quảng bá; Threads/Hash; MultiPV; ngân sách nodes/time; depth/seldepth/nodes/time đạt; raw output; timeout/error; score kind cp/mate, bounds nếu có, POV gốc, score quy về Đỏ; WDL nếu engine hỗ trợ; từng PV dạng tọa độ.

Cache key bao gồm toàn bộ input state/history/ruleset + engine/NNUE + options + budget. Không dùng chỉ FEN hoặc depth. Nhiều thread/hash state có thể ảnh hưởng lặp lại kết quả; ghi điều kiện chạy, không hứa deterministic search.

Score UCI gốc theo bên đến lượt tại root; so trước/sau phải chuẩn hóa POV. CP là thang đánh giá, không số quân chắc chắn hoặc xác suất thắng người học. WDL là mô hình theo điều kiện hiệu chỉnh. Mate có thể bao gồm kết quả quyết định do luật/bí nước; không tự dịch tất cả thành “chiếu hết”. Bounds phải được giữ.

## Phân bổ tính toán đề xuất

Sàng lọc ngân sách nhỏ với MultiPV 3–5; tăng ngân sách ở chỗ bất đồng, thứ hạng thay đổi, chênh lệch nhỏ hoặc claim mạnh. Đánh giá riêng nước sách và phản bác, không chỉ top PV. Ghi các mức ngân sách và độ ổn định; không đặt ngưỡng cp toàn cục thành “sai lầm” cho mọi tàn cuộc.

Không nâng hòa từ eval gần 0, hoặc “duy nhất” từ chỉ một PV. Nhiều biến gần tương đương cần xem mục tiêu sư phạm. Xác nhận chiến thuật của một thế không chứng minh nguyên tắc phổ quát.

## Claim và nhãn

- Source says C: bằng chứng rằng sách có lời C, không tự là C đúng.
- Engine supports C: giới hạn theo run, ruleset và điều kiện nhận dạng.
- Human reviewed: nêu người/động cơ/phạm vi đánh giá, không tự là proof.
- Challenged/conflicted: giữ hồ sơ và phản ví dụ, không đưa vào bài giảng khẳng định mặc định.

## Lean và chứng nhận

Tập con đầu tiên nên là checker diễn biến hoặc chứng nhận mate hữu hạn. Một PV chỉ cho một đường, còn chiến lược buộc thắng phải bao phủ mọi trả lời hợp lệ của đối thủ trong mô hình. Audit ngữ nghĩa theorem, điều kiện trước và định nghĩa luật; không chỉ xem compile xanh.

`kernel_checked` cần attestation riêng, hash theorem/model/toolchain/artifact và kiểm tra dependency axioms, không là metadata tác giả tự gắn. Không dùng sorry/custom axiom cho kết quả engine. Adapter phải nêu phần nào kernel kiểm, phần nào tin vào native/external computation. Không cần formalize tất cả lời giảng.

## Golden cases cần có

Đổi lượt cùng bố trí; góc nhìn score qua một ply; xe/pháo nhận nhầm nhưng FEN vẫn hợp lệ; ký pháp tiền/hậu mơ hồ; biến bắt đầu sai node; thiếu history; long-check/chase; bí nước; engine timeout/bound; hai nước tương đương; một PV bỏ sót phòng thủ; sửa đầu vào làm stale kết quả. Các fixture cờ chuyên môn phải được chủ dự án duyệt trước khi gọi là gold.
