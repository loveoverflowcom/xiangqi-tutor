#!/usr/bin/env python3
"""Validate repository skills using a deliberately small, fail-closed YAML subset.

Frontmatter supports a JSON-quoted name and description. Agent metadata
supports only interface with three JSON-quoted strings. Local links are checked
separately by check_links.py. Files use LF or CRLF, with YAML printable Unicode
characters; raw NEL and Unicode line/paragraph separators are also excluded to
keep line boundaries identical for the checker and YAML readers.
"""
import argparse
import json
from pathlib import Path
import re
import sys

NAME = re.compile(r"[a-z0-9]+(?:-[a-z0-9]+)*")
FIELD = re.compile(r"([a-z_]+): (.+)")
PLACEHOLDER = re.compile(
    r"\b(?:TODO|TBD|FIXME)\b|\{\{[^{}]*\}\}|\[(?:INSERT|REPLACE|FILL IN)\b",
    re.IGNORECASE,
)
INTERFACE_KEYS = {"display_name", "short_description", "default_prompt"}
INVALID_RAW = re.compile(
    r"[^\t\n\r\x20-\x7E\xA0-\uD7FF\uE000-\uFFFD\U00010000-\U0010FFFF]|[\u2028\u2029]"
)


def read_lines(path, root, errors):
    try:
        text = path.read_bytes().decode("utf-8")
    except (OSError, UnicodeError) as exc:
        errors.append(f"{path.relative_to(root)}: cannot read: {exc}")
        return None
    invalid = INVALID_RAW.search(text)
    if invalid:
        errors.append(f"{path.relative_to(root)}: unsupported raw YAML character U+{ord(invalid.group()):04X}")
        return None
    text = text.replace("\r\n", "\n")
    if "\r" in text:
        errors.append(f"{path.relative_to(root)}: use LF or CRLF line endings")
        return None
    return text.splitlines()


def fields(lines, allowed, path, root, errors, indent=""):
    """Accept only explicit keys and single-line scalars; never infer YAML types."""
    values = {}
    label = path.relative_to(root)
    for line in lines:
        if not line.strip():
            continue
        match = FIELD.fullmatch(line[len(indent):]) if line.startswith(indent) else None
        if not match:
            errors.append(f"{label}: unsupported metadata syntax: {line!r}")
            continue
        key, raw = match.groups()
        if key not in allowed or key in values:
            errors.append(f"{label}: unknown or duplicate key {key!r}")
            continue
        values[key] = None
        try:
            value = json.loads(raw)
        except (ValueError, TypeError):
            value = None
        if not isinstance(value, str) or not raw.startswith('"'):
            errors.append(f"{label}: {key} must be a JSON double-quoted string")
            continue
        values[key] = value
        if not value.strip():
            errors.append(f"{label}: {key} must be nonempty")
        if PLACEHOLDER.search(value):
            errors.append(f"{label}: {key} contains a template placeholder")
    for key in sorted(allowed - values.keys()):
        errors.append(f"{label}: missing required key {key!r}")
    return values


def check_skill(directory, root, errors):
    path = directory / "SKILL.md"
    lines = read_lines(path, root, errors)
    if lines is not None:
        if not lines or lines[0] != "---" or "---" not in lines[1:]:
            errors.append(f"{path.relative_to(root)}: missing delimited frontmatter")
        else:
            end = lines.index("---", 1)
            meta = fields(lines[1:end], {"name", "description"}, path, root, errors)
            name, description = meta.get("name"), meta.get("description")
            if name is not None and (not NAME.fullmatch(name) or len(name) > 64 or name != directory.name):
                errors.append(f"{path.relative_to(root)}: name must match its directory and use <=64 lowercase letters, digits or single hyphens")
            if description is not None and (len(description) > 1024 or "<" in description or ">" in description):
                errors.append(f"{path.relative_to(root)}: description must be <=1024 characters without angle brackets")
            body = lines[end + 1:]
            if not any(line.strip() for line in body) or len(body) > 500:
                errors.append(f"{path.relative_to(root)}: body must be nonempty and <=500 lines")
            if PLACEHOLDER.search("\n".join(body)):
                errors.append(f"{path.relative_to(root)}: body contains a template placeholder")

    agent = directory / "agents" / "openai.yaml"
    lines = read_lines(agent, root, errors)
    if lines is None:
        return
    significant = [line for line in lines if line.strip()]
    if not significant or significant[0] != "interface:":
        errors.append(f"{agent.relative_to(root)}: expected interface mapping")
        return
    meta = fields(significant[1:], INTERFACE_KEYS, agent, root, errors, indent="  ")
    short = meta.get("short_description")
    if short is not None and not 25 <= len(short) <= 64:
        errors.append(f"{agent.relative_to(root)}: short_description must be 25-64 characters")
    prompt = meta.get("default_prompt")
    if prompt is not None and not re.search(r"\$" + re.escape(directory.name) + r"(?![a-zA-Z0-9_-])", prompt):
        errors.append(f"{agent.relative_to(root)}: default_prompt must mention ${directory.name} exactly")


def check_repository(root):
    errors = []
    base = root / ".agents" / "skills"
    directories = sorted(path for path in base.iterdir() if path.is_dir()) if base.is_dir() else []
    if not directories:
        errors.append(".agents/skills: no skill directories found")
    for directory in directories:
        check_skill(directory, root, errors)
    return len(directories), errors


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", nargs="?", type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()
    count, errors = check_repository(args.root.resolve())
    if errors:
        print("\n".join(errors), file=sys.stderr)
        return 1
    print(f"Checked {count} repository skills and agent interfaces.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
