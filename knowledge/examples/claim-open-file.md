# Nguyên tắc phát triển Xe cần thẩm định

```xqt
{
  "schema_version": 1,
  "id": "claim-open-file",
  "revision": 1,
  "title": "Nguyên tắc phát triển Xe cần thẩm định",
  "language": "vi",
  "review": {
    "state": "draft",
    "by": null,
    "note": ""
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
    "claim_type": "heuristic",
    "statement": "Trong thực chiến, cân nhắc phát triển Xe ra lộ mở phù hợp.",
    "conclusion": {
      "predicate": "consider_rook_activation",
      "args": [
        "side_to_move"
      ]
    },
    "evidence_status": "unassessed",
    "evidence": [],
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

Đây là giả thuyết giảng dạy giản lược do dự án tự soạn, chưa được kỳ thủ duyệt. Điều kiện và ngoại lệ chưa đầy đủ; không thực thi như luật.

## [counterexamples]

Cần tìm thế mà đưa Xe ra lộ mở thua chiến thuật hoặc bỏ lỡ ứng cứu Tướng. Chưa có thế phản ví dụ được kiểm tra.

## [questions]

Thế nào là lộ mở phù hợp? Điều kiện an toàn nào cần thêm? Phù hợp trình độ nào?
