# Replay cây biến và Pikafish run có provenance

## Why

Cấu trúc FEN đúng chưa bảo đảm đúng luật hoặc lời giải.

## Scope

Chọn ruleset/version, typed replay errors, adapter UCI bất đồng, run store, POV, budget, cache key state/history và review queue.

## Non-goals

Không gọi score là proof; chưa cần theorem prover hoặc TTS/UI riêng.

## Dependencies

010; kỳ thủ duyệt rule gold, nguồn luật chính thức, binary/NNUE provenance.

## Suggested sequence

Replay trước engine; handshake options; giới hạn process và timeout; MultiPV sàng lọc rồi reanalyse điểm bất đồng.

## Review boundary

Golden cases về lượt, nhánh, long-check/chase, timeout, score bounds, bí nước và stale run phải phân biệt rõ.

## Risks / unknowns

Cchess/Pikafish có thể không cùng adjudication; thiếu lịch sử không phục hồi được từ FEN.

## Follow-ups

Chứng nhận chiến thuật hữu hạn và benchmark tốc độ sau khi đúng.
