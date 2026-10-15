# Chọn bằng chứng cho thay đổi phần mềm

Đọc [bằng chứng bootstrap](../../../../docs/verification.md) và [tests hiện có](../../../../crates/knowledge/tests/contracts.rs). Bằng chứng cũ là baseline; báo cáo mới phải gắn commit/diff và môi trường thực sự đã kiểm tra.

## Chọn oracle độc lập

- Bắt đầu từ contract hoặc ví dụ đã được người biên tập xác định, không lấy output của chính hàm đang sửa làm expected result.
- Với parser/schema, dùng fixture tối thiểu có kết quả chấp nhận/từ chối rõ. Với regression, giữ input từng gây lỗi và diagnostic/kết quả kỳ vọng.
- Với snapshot, sửa riêng prose, revision hoặc metadata rồi kiểm tra stale; hash đúng chỉ chứng minh bytes khớp, không chứng minh nội dung đúng.
- Với index eligibility, dùng corpus nhỏ dựng tay, liệt kê claim được xuất/bị loại và lý do theo dependency closure. Bao phủ private xuyên nhiều bậc, rejected dependency và stale input.
- Với phép biến đổi xác định, kiểm tra invariant bổ sung có ý nghĩa: thứ tự đọc file không đổi plan, cùng bytes cho cùng snapshot, sửa nội dung không dùng lại chunk cũ. Không dùng round-trip qua cùng một bug làm chứng cứ duy nhất.
- Với adapter tương lai, dùng expected artifact/trace độc lập; test timeout, output lỗi và sai phiên bản bên cạnh happy path. Không dùng engine để tự xác nhận luật replay của chính adapter engine.

## Chọn mức kiểm tra

| Thay đổi | Kiểm tra phù hợp | Điều chưa được kết luận |
| --- | --- | --- |
| Tài liệu hoặc skill | Diff, metadata, link local và kiểm tra ví dụ/lệnh còn tồn tại | Rust hay adapter đã chạy lại |
| Parser, record, validation | Contract tests và input đối nghịch theo ranh giới thay đổi | Thế cờ hợp lệ hoặc sách được OCR đúng |
| Index-plan | Eligibility, determinism, provenance, stale/private exclusion | Đã có vectors hay retrieval tốt |
| Adapter mới | Fixture offline + live integration riêng nếu có môi trường | Mock/offline pass không là live pass |
| Nội dung bằng chứng | Mở artifact, kiểm tra run/provenance đúng contract | Metadata/hash đơn lẻ không là xác minh nội dung |

Ưu tiên các ranh giới: stale snapshot, tham chiếu sai loại, nguồn private, claim chưa duyệt, unknown fields, tự khai `kernel_checked`, history unknown và ruleset unresolved. Thêm test khi hành vi thay đổi có rủi ro thực; tránh đòi số lượng test hoặc coverage tùy tiện.

## Ghi kết quả trung thực

Mỗi kết luận nêu lệnh, phạm vi và kết quả `PASS`/`FAIL`/`NOT_RUN`; giữ exit status và log cần thiết. Nếu kiểm tra fail, phân biệt lỗi triển khai với môi trường và không đánh dấu hoàn tất phần còn bị chặn. Không biến test pass thành xác nhận chuyên môn, danh tính reviewer hay chứng nhận Lean.

Với live run, ghi phiên bản tool/model, digest đầu vào, tham số/budget, output digest và chỗ lưu artifact; chỉ báo replay/engine/proof đã chạy sau khi kiểm tra artifact thực. Không đưa dữ liệu sách riêng, token hoặc weights vào repo công khai để làm evidence.
