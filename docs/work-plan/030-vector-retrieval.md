# Qdrant generation và truy xuất có đánh giá

## Why

Index-plan chưa có vectors hay live retrieval.

## Scope

Benchmark multilingual embedding/sparse tokenizer/reranker; adapter Qdrant; mapping UUID; generation/alias rollback và retirement; exact-position path.

## Non-goals

Không thêm graph DB hoặc train model lớn khi chưa có benchmark chứng minh cần.

## Dependencies

Gold corpus/queries và claim eligible từ 010/020; đọc indexing workflow.

## Suggested sequence

Chọn profile; index staging; kiểm tra retrieval+provenance; đổi alias; test sửa/xóa record và phục hồi.

## Review boundary

Không trộn generation, không rò private vào public, superseded record không còn phục vụ, recall/citation đạt ngưỡng pilot do chủ dự án chọn.

## Risks / unknowns

Độ dài thẻ; CJK/Vietnamese tokenizer; model/version đổi dimensions; nguồn đồng nghĩa nhưng khác phạm vi.

## Follow-ups

Scale/caching chỉ làm khi có đo.
