# Chứng nhận chiến thuật với Lean

## Why

Một số kết luận cưỡng bức cần bảo đảm cao hơn score/PV. Chưa có rule model và tập bài thật nên giữ ở backlog.

## Scope

Một checker nhỏ cho chứng nhận có lượng từ rõ và phạm vi hữu hạn; proof artifact và attestation riêng. Audit theorem fidelity và dependency axioms.

## Non-goals

Không biến toàn bộ lời giảng thành theorem; không sửa Lean để tin vô điều kiện output Pikafish.

## Dependencies

020 hoàn thành, corpus cờ chuyên môn được duyệt, định nghĩa terminal/lịch sử/luật rõ, chọn bộ chứng nhận có ích.

## Suggested sequence

Chọn một họ thế; định nghĩa chứng nhận; kiểm tra soundness; sinh ứng viên bằng công cụ ngoài; kiểm tra với Lean; nối evidence store.

## Review boundary

Checker từ chối PV thiếu nhánh, invalid move, wrong model hash, sorry/custom engine axiom và proof stale.

## Risks / unknowns

Kích thước cây; bộ luật; khác biệt giữa thắng hữu hạn và ưu thế đánh giá; native computation trust boundary.

## Follow-ups

Mở rộng khi pilot có lợi ích đo được.
