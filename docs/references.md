# Tham khảo app, thư viện và nguồn — kiểm tra 2026-10-15

Đây là nghiên cứu từ tài liệu chính thức/repository, không là audit UI thực tế. Không sao chép mã/app hay sách trong bước khởi tạo. “Áp dụng” bên dưới là quyết định thiết kế của dự án, không là tính năng đã triển khai.

## App tương tự

| Nguồn | Điều quan sát từ tài liệu | Áp dụng cho Xiangqi Tutor |
| --- | --- | --- |
| [Lichess Study](https://lichess.org/@/lichess/blog/study-chess-the-lichess-way/V0KrLSkA), [lila](https://github.com/lichess-org/lila) | Study/analysis có chapter, diễn biến và bình luận; lila liệt kê shared analysis board và tactic trainer | Giữ cây biến và lời bình tại node; học theo chủ đề, không chỉ nhận nước tốt nhất |
| [ElephantChess](https://github.com/benckx/elephantchess) | Repo mô tả chơi cờ tướng, bot, puzzles và phân tích ván | Tách play, review, puzzle; dùng ván đã chơi để tạo bài học khi adapter sẵn sàng |
| [Anki manual](https://docs.ankiweb.net/manual/deck-options) | FSRS và lịch ôn với các mức phản hồi, quản lý lượng bài | Lỗi người học và lịch ôn là dữ liệu riêng, không trộn với độ đúng tri thức; để bước sau |

Chưa xác minh được trang lessons/features của Xiangqi.com trong lượt tra cứu này; không dựa vào trang không đọc được để khẳng định tính năng.

## OCR và kỳ phổ

- [PaddleOCR](https://github.com/PaddlePaddle/PaddleOCR), [tài liệu](https://www.paddleocr.ai/main/en/): OCR/bố cục đa ngôn ngữ; version model chọn sau benchmark trên sách, không pin mù một tên “mới nhất”.
- [TheOne1006/chinese-chess-recognition](https://github.com/TheOne1006/chinese-chess-recognition): board alignment + grid classification; repo liên kết model ONNX. Đo riêng domain scan sách.
- [RakuXQ](https://github.com/rakulgt/rakuxq): local OpenCV/ONNX, audit correction, accepted/review_required. Không dùng heuristic lượt đi từ hướng ảnh cho sách. Weights tải riêng; không tự phân phối lại.
- [DeepXiangQi](https://github.com/DuyLeTran/DeepXiangQi): YOLO/pose, dựng lại bàn cờ và FEN; README nói weights không kèm, mục đích educational/research. Chưa chọn làm dependency sản phẩm.
- [cchess](https://github.com/walker8088/cchess): Python kỳ phổ, ký pháp và engine bridge; ứng viên adapter, không thay việc kiểm thử luật thi đấu.

## Engine và proof

- [Pikafish](https://github.com/official-pikafish/Pikafish), [runtime](https://www.pikafish.com/wiki/en/reference/engine-runtime.html), [UCI options](https://www.pikafish.com/wiki/en/reference/uci-options.html): phân biệt score POV, cp/mate/WDL, MultiPV và ngân sách.
- [Networks](https://github.com/official-pikafish/Networks): kiểm tra riêng điều kiện sử dụng weights trước khi triển khai/phân phối; giấy phép source không tự bao gồm mọi artifact.
- [Lean proof validation](https://lean-lang.org/doc/reference/latest/ValidatingProofs/): kiểm tra proof và fidelity của statement là hai việc; audit axioms/dependencies.
- [SWI-Prolog WFS](https://www.swi-prolog.org/pldoc/man?section=WFS): tham khảo trạng thái undefined khi xây suy luận; chưa là dependency.
- [W3C PROV-O](https://www.w3.org/TR/prov-o/): tham khảo nguồn, derivation, revision; chưa bắt buộc RDF/OWL hay graph DB.

## Hạ tầng

- [Rust 1.98.1](https://blog.rust-lang.org/2026/09/03/Rust-1.98.1/): pin theo yêu cầu; bản vá sửa miscompilation vtable của 1.98.0.
- [Qdrant v1.19.2](https://github.com/qdrant/qdrant/releases/tag/v1.19.2): image phát triển đã chọn, không khẳng định Docker đã chạy.
- [Qdrant hybrid queries](https://qdrant.tech/documentation/search/hybrid-queries/): dense/sparse, hợp thứ hạng và truy vấn nhiều bước. Tự đánh giá trên corpus Việt/Trung.

## Phạm vi áp dụng

Mượn mô hình luồng học và cấu trúc dữ liệu, không sao chép kiến trúc quy mô Lichess. Chưa có benchmark nội bộ cho app/model. Chọn công cụ dựa trên dữ liệu thật; ghi rõ license/source/weights/version trong mỗi adapter trước tích hợp.
