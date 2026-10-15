# Thế ban đầu minh họa hợp đồng

```xqt
{
  "schema_version": 1,
  "id": "position-start",
  "revision": 1,
  "title": "Thế ban đầu minh họa hợp đồng",
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
    }
  ],
  "payload": {
    "kind": "position",
    "fen": "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1",
    "ruleset": "unresolved",
    "history": {
      "state": "unknown"
    },
    "origin": {
      "source_id": "source-demo",
      "pdf_page": 1,
      "printed_page": null,
      "region": null
    }
  }
}
```

## [image-review]

Thế được nhập trực tiếp để minh họa, không phải đầu ra OCR.

## [validation]

FEN chỉ được kiểm tra cấu trúc. Chưa chạy luật đầy đủ hoặc Pikafish.
