#!/usr/bin/env python3
"""Check local Markdown link targets without fetching remote URLs."""
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
errors = []
count = 0
for path in sorted(ROOT.rglob("*.md")):
    if any(part in {"target", ".git", "private", "data", "build"} for part in path.relative_to(ROOT).parts):
        continue
    for target in re.findall(r"\[[^\]]*\]\(([^\s)]+)\)", path.read_text()):
        if ":" in target or target.startswith("#"):
            continue
        count += 1
        local = target.split("#", 1)[0]
        if not (path.parent / local).exists():
            errors.append(f"{path.relative_to(ROOT)}: missing {target}")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print(f"Checked {count} local Markdown link targets.")
