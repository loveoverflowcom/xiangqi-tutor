# Lượt đi được khai báo trong bản ghi mẫu

```xqt
{
  "schema_version": 1,
  "id": "claim-explicit-turn",
  "revision": 1,
  "title": "Lượt đi được khai báo trong bản ghi mẫu",
  "language": "vi",
  "review": {
    "state": "approved",
    "by": "bootstrap-author",
    "note": "Duyệt cấu trúc ví dụ tự soạn; không phải thẩm định kỳ thủ hay engine."
  },
  "dependencies": [
    {
      "id": "source-demo",
      "revision": 1,
      "content_sha256": "7efff4aeb52e9304ba9aee0515b6690d9d0c88230230f1008e1e628a01a3636f"
    },
    {
      "id": "position-start",
      "revision": 1,
      "content_sha256": "696e029efb6449193cfc6fba1de2843aad5162fe6ca9313e2619158c57aec401"
    }
  ],
  "payload": {
    "kind": "claim",
    "claim_type": "observation",
    "statement": "Bản ghi position-start khai báo lượt Đỏ bằng trường w.",
    "conclusion": {
      "predicate": "declared_side_to_move",
      "args": [
        "position-start",
        "red"
      ]
    },
    "evidence_status": "supported",
    "evidence": [
      {
        "kind": "source",
        "reference": "example:source-demo.txt",
        "artifact_sha256": "59ffee3bfb3ce202becdba34c9a0e24d238244c19877b9107c09416d7b5df80a",
        "note": "Đối chiếu ví dụ tự soạn và bản ghi; không có chứng minh Lean hay phân tích engine."
      }
    ],
    "scope": "position",
    "position_ids": [
      "position-start"
    ],
    "sources": [
      {
        "source_id": "source-demo",
        "pdf_page": 1,
        "printed_page": null,
        "region": null
      }
    ],
    "conditions": [],
    "exceptions": [],
    "formal_status": "not_formalized",
    "relations": []
  }
}
```

## [reasoning]

Đọc trường lượt đi đã khai báo trong metadata, không suy ra từ hướng ảnh. Đây là ví dụ dữ liệu, không là định lý cờ.

## [counterexamples]

Một ảnh có cùng bố trí quân có thể đi kèm lượt Đen; không suy rộng từ hình ảnh.

## [questions]

Bộ luật thi đấu nào sẽ dùng cho các bài học thật?
