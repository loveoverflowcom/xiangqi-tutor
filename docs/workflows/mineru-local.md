# MinerU local: cài đặt và PDF → Markdown/JSON

Status: **đã chạy công cụ độc lập** ngày 2026-10-15 trên macOS 26.5.2,
Apple Silicon arm64, RAM 16 GB. MinerU **4.0.12**, Python **3.12.15**,
uv **0.12.24**. Importer/adapter OCR của `xqt` vẫn chưa triển khai.

Quy trình này chuẩn bị công cụ cho [pilot 010](../work-plan/010-pilot-ingestion.md).
Đầu ra là OCR raw để đối chiếu theo [workflow ingestion](ingestion.md), chưa là thẻ
tri thức đã duyệt. Lượt chạy dưới đây dùng hai PDF một trang tự soạn; chưa xử lý sách thật,
nhận dạng bàn cờ, suy FEN hoặc đánh giá accuracy trên bộ chuẩn.

## 1. Cài vào môi trường riêng ngoài repo

Máy thử ban đầu chỉ có Python hệ thống 3.9.6. Homebrew đã có sẵn; cài uv rồi dùng
Python 3.12 riêng, giữ nguyên Rust/toolchain và dependencies của workspace.

```bash
HOMEBREW_NO_AUTO_UPDATE=1 brew install uv
mineru_root="$HOME/.local/share/xiangqi-tutor/ocr/mineru"
UV_SYSTEM_CERTS=true uv venv --python 3.12 "$mineru_root/.venv"
UV_SYSTEM_CERTS=true uv pip install --python "$mineru_root/.venv/bin/python" 'mineru==4.0.12'
source "$mineru_root/.venv/bin/activate"
mineru version --json
uv pip check --python "$mineru_root/.venv/bin/python"
uv pip freeze --python "$mineru_root/.venv/bin/python" > "$mineru_root/requirements.lock.txt"
```

`requirements.lock.txt` ghi các phiên bản dependency thực tế để lưu cùng báo cáo chạy;
pin MinerU không tự pin toàn bộ dependency hay revision của model. Trên máy thử,
122 packages qua `uv pip check`. Bản cài mặc định trên Apple Silicon đã gồm Torch;
không cần cài extra `full` cho quy trình này.

**Lỗi đã gặp:** uv tải Python báo `invalid peer certificate: UnknownIssuer`.
`UV_SYSTEM_CERTS=true` khắc phục bằng kho chứng chỉ hệ điều hành, vẫn xác minh TLS.
Nếu kho này chưa có CA cần thiết, sửa trust store theo môi trường trước khi thử lại.

Mỗi terminal mới cần đặt lại `mineru_root` rồi chạy lệnh `source` ở trên. Để gọi CLI từ terminal đã có
`~/.local/bin` trong `PATH`, có thể tạo liên kết sau; `ln -s` sẽ báo lỗi nếu tên đã tồn tại,
không ghi đè lệnh khác:

```bash
mkdir -p "$HOME/.local/bin"
ln -s "$mineru_root/.venv/bin/mineru" "$HOME/.local/bin/mineru"
ln -s "$mineru_root/.venv/bin/mineru-kit" "$HOME/.local/bin/mineru-kit"
ln -s "$mineru_root/.venv/bin/mineru-webui" "$HOME/.local/bin/mineru-webui"
```

## 2. Tải và xác minh model

Với cấu hình mới mặc định trên máy thử, small models dùng Torch/MPS và VLM dùng
llama.cpp. Bộ Standard đã tải khoảng **2,1 GB** weights:

| Model | Vai trò | Kết quả `models verify` |
| --- | --- | --- |
| `MinerU-4_models_torch` | OCR chữ, layout, công thức, bảng | ok |
| `MinerU2.5-Pro-2605-1.2B-GGUF` | VLM cho Standard/Advanced | ok |

```bash
HF_HUB_DISABLE_XET=1 mineru-kit models download --tier standard --source huggingface
mineru-kit models verify --tier standard
```

**Lỗi đã gặp:** tải qua Xet hoàn tất bộ small models nhưng hai weights GGUF đứng ở
0 byte. Đã dừng đúng process tải của lượt cài đó, rồi chạy lại bằng lệnh HTTP ở trên.
Small models được tái sử dụng; VLM tải xong và verify đều `ok`. Không chạy hai process
tải cùng bộ model, không tự ghi completion marker để vượt kiểm tra readiness.

Mặc định models ở `~/.mineru/models`; môi trường Python ở `mineru_root`. Chừa thêm
dung lượng cho thư viện, cache và kết quả ngoài tổng weights. Máy thử có khoảng
15 GB trống trước khi cài; đây là điều kiện quan sát, chưa là mức tối thiểu đã đo.

Nếu máy có cấu hình MinerU từ trước, kiểm tra `mineru-kit models show` và cấu hình
VLM trước khi dùng tài liệu riêng tư: workflow này dùng inference local, không cấu hình
remote VLM, remote document service hay LLM hậu xử lý. `MINERU_MODEL_SOURCE=local`
chỉ ngăn tải model thiếu; nó không vô hiệu hóa một remote endpoint đã cấu hình sẵn.

## 3. Chọn đường chuyển đổi

Đặt đường dẫn của bản gốc và thư mục output mới, bên ngoài corpus công khai.
Giữ raw PDF bất biến, SHA-256, phiên bản tool/model/config và mapping trang.

```bash
pdf_path="/absolute/path/to/book.pdf"
ocr_output="/absolute/path/to/new-ocr-output"
mkdir -p "$ocr_output"
shasum -a 256 "$pdf_path" > "$ocr_output/source.sha256"
mineru version --json > "$ocr_output/tool-version.json"
```

PDF có text layer tốt, không chạy OCR/model:

```bash
MINERU_MODEL_SOURCE=local mineru-kit parse "$pdf_path" \
  -o "$ocr_output/book-text.md" --tier flash --ocr-mode txt
```

PDF scan, giữ cả Markdown và dữ liệu có cấu trúc để đối chiếu:

```bash
MINERU_MODEL_SOURCE=local mineru-kit parse "$pdf_path" \
  -o "$ocr_output/book.zip" --format zip --tier standard --ocr-mode ocr
```

ZIP của mẫu đã chạy chứa `markdown.md`, `middle_json.json`, `model_output.json`,
`structured_content.json`. Với tài liệu có hình, giữ cả tài nguyên hình được xuất.
Đối chiếu quy ước trang/vùng của MinerU trước khi ánh xạ sang SourceSpan của dự án;
chưa có adapter thực hiện chuyển đổi này. `mineru-kit parse` mặc định xử lý tất cả trang.
`flash --ocr-mode txt` không khôi phục chữ trên trang chỉ chứa ảnh.

Chỉ cần Markdown thì đổi `-o` thành tên `.md` và bỏ `--format zip`. Khi thử sách thật,
chọn phạm vi nhỏ trước, chẳng hạn thêm `--pages '1-5'`.

CLI có lệnh mở giao diện local:

```bash
mineru-kit webui --server-name 127.0.0.1 --server-port 7860
```

Lượt cài này đã kiểm tra `webui --help`; mở và chuyển đổi qua giao diện WebUI là **NOT_RUN**.

## 4. Tạo lại mẫu tự soạn

Chạy trong venv đã kích hoạt. `reportlab`, Pillow và pypdfium2 đã có trong môi trường
được cài ở trên. Tạo thư mục riêng để không ghi đè mẫu hay kết quả cũ:

```bash
export XQT_MINERU_SMOKE_DIR="$(mktemp -d "${TMPDIR:-/tmp}/xqt-mineru-smoke.XXXXXX")"
python - <<'PY'
from pathlib import Path
import hashlib
import json
import os
from PIL import Image, ImageDraw, ImageFont
from reportlab.pdfgen import canvas
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
import pypdfium2 as pdfium

root = Path(os.environ["XQT_MINERU_SMOKE_DIR"])
font_path = "/System/Library/Fonts/Supplemental/Arial Unicode.ttf"
lines = [
    "Xiangqi Tutor OCR smoke test",
    "中国象棋 基础练习",
    "Cờ tướng: bài tập đọc văn bản.",
    "English: preserve the original text and page number.",
    "自编示例。 Sample written for testing; no book content.",
    "Page 1 - OCR output requires human review.",
]
pdfmetrics.registerFont(TTFont("SampleUnicode", font_path))
c = canvas.Canvas(str(root / "native-text.pdf"), pagesize=(595, 842))
c.setTitle("Original synthetic OCR test")
c.setFont("SampleUnicode", 17)
for i, line in enumerate(lines):
    c.drawString(42, 770 - i * 55, line)
c.save()
img = Image.new("RGB", (1785, 2526), "white")
draw = ImageDraw.Draw(img)
font = ImageFont.truetype(font_path, 51)
for i, line in enumerate(lines):
    draw.text((126, 216 + i * 165), line, font=font, fill="black")
img.save(root / "scan-preview.png")
img.save(root / "image-only.pdf", "PDF", resolution=216)
manifest = {"rights": "original", "expected_lines": lines, "files": {}}
for name in ["native-text.pdf", "image-only.pdf"]:
    path = root / name
    manifest["files"][name] = hashlib.sha256(path.read_bytes()).hexdigest()
    doc = pdfium.PdfDocument(path)
    assert len(doc) == 1
    page = doc[0]
    text = page.get_textpage().get_text_range()
    assert (len(text) > 100) if name == "native-text.pdf" else (text == "")
    page.render(scale=1.5).to_pil().save(root / (name + ".png"))
(root / "fixture.json").write_text(
    json.dumps(manifest, ensure_ascii=False, indent=2) + "\n", encoding="utf-8"
)
print(root)
PY
```

Font trên là font có thật trên máy thử macOS; máy thiếu font phải chọn font Unicode
đủ Trung/Việt và ghi lại tên/phiên bản. Xem hai PNG render để xác nhận không mất chữ
hoặc clipping trước khi dùng làm oracle. Mẫu scan là ảnh ở 216 DPI, không có text layer.

```bash
MINERU_MODEL_SOURCE=local mineru-kit parse "$XQT_MINERU_SMOKE_DIR/native-text.pdf" \
  -o "$XQT_MINERU_SMOKE_DIR/native-text.md" --tier flash --ocr-mode txt
MINERU_MODEL_SOURCE=local mineru-kit parse "$XQT_MINERU_SMOKE_DIR/image-only.pdf" \
  -o "$XQT_MINERU_SMOKE_DIR/image-only-basic.zip" --format zip --tier basic --ocr-mode ocr
MINERU_MODEL_SOURCE=local mineru-kit parse "$XQT_MINERU_SMOKE_DIR/image-only.pdf" \
  -o "$XQT_MINERU_SMOKE_DIR/image-only-standard.zip" --format zip --tier standard --ocr-mode ocr
```

So từng dòng trong `markdown.md` của ZIP với `expected_lines`, ghi riêng số dòng khớp
nguyên văn và số dòng khớp sau khi bỏ whitespace. Không sửa raw OCR để làm phép so sánh xanh.
PDF có thể chứa metadata thời gian; mỗi lần tạo lại cần ghi hash mới, không đòi hash giống lượt cũ.

## 5. Kết quả quan sát ngày 2026-10-15

| Đường xử lý | Thực thi | Dòng khớp nguyên văn | Khớp nếu bỏ whitespace |
| --- | --- | --- | --- |
| Native PDF, Flash/txt | PASS | 6/6 | 6/6 |
| Image-only PDF, Basic/ocr | PASS | 4/6 | 5/6 |
| Image-only PDF, Standard/ocr | PASS | 4/6 | 5/6 |

PASS ở đây là CLI hoàn tất và tạo output đọc được, không là accuracy đạt yêu cầu.
Nguồn `Cờ tướng: bài tập đọc văn bản.` bị Basic đọc thành `C tưng: bài tp đc văn bn.`;
Standard đọc thành `Cò tuống: bài tập đọc văn bản.`. Standard còn bỏ một khoảng trắng
sau dấu `。`; Basic bỏ khoảng trắng giữa hai cụm chữ Trung. Các dòng Trung/Anh còn lại
khớp nếu bỏ whitespace trong mẫu này, chưa đủ suy ra chất lượng trên sách.

Snapshot của lượt chạy được giữ trong kho local, không kèm PDF/weights trong Git:

| Artifact | SHA-256 quan sát |
| --- | --- |
| `native-text.pdf` | `b10faa5762e4d43a822081aedcc6b0d6f547b11b6ee8c4ec43dd887844afffef` |
| `image-only.pdf` | `e91c0c2f10535325bc357693ea98b4a208ae45eb264606800282d1c61e4df28d` |
| `image-only-basic.zip` | `ab472ca2f972655aa5d7d45260de19d4944f0199774255b9ef46195cf3b76660` |
| `image-only-standard.zip` | `e9e0a45c959e48f4c50057f1636bdfc2ee547c4a4cd3494024981d466a94fbb9` |

Hash nhận diện bytes của artifact đã chạy, không tự chứng minh nội dung hoặc độ đúng OCR.
Báo cáo local `installation.json`, `requirements.lock.txt`, `smoke-test/fixture.json` và
`smoke-test/ocr-checks.json` lưu cùng môi trường cài của lượt thử.

**NOT_RUN:** PDF sách thật, bảng/nhiều cột/hình cờ, board recognition/FEN, benchmark accuracy,
replay/Pikafish/Lean, importer sinh thẻ, embedding/Qdrant. Các phần còn lại theo
[pilot 010](../work-plan/010-pilot-ingestion.md); giữ raw/corrected/editorial và duyệt
chuyên môn trước khi đưa thẻ vào luồng index mặc định.

## Nguồn công cụ

Đã đối chiếu và dùng CLI của bản cài 4.0.12:
[Quick Start](https://opendatalab.github.io/MinerU/quick_start/),
[model download và xử lý lỗi Xet](https://opendatalab.github.io/MinerU/usage/model_source/),
[tiers và runtime](https://opendatalab.github.io/MinerU/usage/tiers/).
Các trang upstream có thể đổi; khi nâng phiên bản, kiểm tra `--help`, tải/verify model và
chạy lại mẫu, ghi kết quả mới thay vì kế thừa PASS ở tài liệu này.
