#!/usr/bin/env python3
"""Mutation checks for the repository skill metadata contract."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

from check_skills import check_repository


class SkillMetadataTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.skill = self.root / ".agents" / "skills" / "sample-skill"
        (self.skill / "agents").mkdir(parents=True)
        self.markdown = self.skill / "SKILL.md"
        self.agent = self.skill / "agents" / "openai.yaml"
        self.write_skill()
        self.write_agent()

    def write_skill(self, name="sample-skill", description='"Review a typed boundary."', extra="", body="# Workflow\n\nRead the contract and check its invariants.\n"):
        self.markdown.write_text(f"---\nname: {json.dumps(name)}\ndescription: {description}\n{extra}---\n\n{body}", encoding="utf-8")

    def write_agent(self, prompt="Use $sample-skill to review a typed boundary.", extra=""):
        self.agent.write_text(
            'interface:\n  display_name: "Sample Skill"\n'
            '  short_description: "Review a typed boundary and its invariants"\n'
            f"  default_prompt: {json.dumps(prompt)}\n{extra}", encoding="utf-8")

    def assert_invalid(self, message):
        count, errors = check_repository(self.root)
        self.assertEqual(count, 1)
        self.assertTrue(any(message in error for error in errors), errors)
        self.assertTrue(all(error.startswith(".agents/skills/") for error in errors), errors)

    def test_valid_and_cli_root(self):
        self.assertEqual(check_repository(self.root), (1, []))
        result = subprocess.run([sys.executable, str(Path(__file__).with_name("check_skills.py")), str(self.root)], capture_output=True, text=True)
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_invalid_and_duplicate_frontmatter(self):
        for description in ("[not, scalar]", "unquoted", '"bad <tag>"', '""', json.dumps("a" * 1025)):
            with self.subTest(description=description):
                self.write_skill(description=description)
                self.assert_invalid("SKILL.md")
        for extra in ('name: sample-skill\n', 'unexpected: "value"\n'):
            with self.subTest(extra=extra):
                self.write_skill(extra=extra)
                self.assert_invalid("unknown or duplicate key")

    def test_name_must_match_directory(self):
        for name in ("other-skill", "Sample-skill", "sample--skill", "a" * 65):
            with self.subTest(name=name):
                self.write_skill(name=name)
                self.assert_invalid("name must match")

    def test_names_that_yaml_would_infer_require_quotes(self):
        for name in ("true", "null", "123", "0xff"):
            with self.subTest(name=name):
                destination = self.skill.parent / name
                self.skill.rename(destination)
                self.skill = destination
                self.markdown = self.skill / "SKILL.md"
                self.agent = self.skill / "agents" / "openai.yaml"
                self.write_skill(name=name)
                self.write_agent(prompt=f"Use ${name} to review the contract.")
                self.assertEqual(check_repository(self.root), (1, []))
                self.markdown.write_text(self.markdown.read_text().replace(f'name: "{name}"', f"name: {name}"), encoding="utf-8")
                self.assert_invalid("name must be a JSON double-quoted string")

    def test_forbidden_raw_yaml_characters(self):
        for character in ("\x7f", "\x80", "\ufffe", "\x85", "\u2028", "\u2029"):
            for target in (self.markdown, self.agent):
                with self.subTest(character=hex(ord(character)), target=target.name):
                    self.write_skill()
                    self.write_agent()
                    text = target.read_text()
                    text = text.replace("boundary.", f"boundary{character}.")
                    target.write_text(text, encoding="utf-8")
                    self.assert_invalid("unsupported raw YAML character")

    def test_vietnamese_and_explicit_line_endings(self):
        self.write_skill(description=json.dumps("Kiểm tra dữ liệu cờ tướng.", ensure_ascii=False))
        self.agent.write_text(self.agent.read_text().replace("Sample Skill", "Gia sư cờ tướng"), encoding="utf-8")
        self.assertEqual(check_repository(self.root), (1, []))
        for target in (self.markdown, self.agent):
            target.write_bytes(target.read_bytes().replace(b"\n", b"\r\n"))
        self.assertEqual(check_repository(self.root), (1, []))
        self.agent.write_bytes(self.agent.read_bytes().replace(b"\r\n", b"\r", 1))
        self.assert_invalid("use LF or CRLF line endings")

    def test_missing_metadata(self):
        self.markdown.write_text("# Missing frontmatter\n", encoding="utf-8")
        self.assert_invalid("frontmatter")
        self.write_skill()
        self.agent.unlink()
        self.assert_invalid("cannot read")

    def test_missing_and_inexact_default_prompt(self):
        self.write_agent()
        self.agent.write_text(self.agent.read_text().split("  default_prompt:")[0], encoding="utf-8")
        self.assert_invalid("missing required key 'default_prompt'")
        for prompt in ("Use sample-skill.", "Use $sample-skill-extra."):
            with self.subTest(prompt=prompt):
                self.write_agent(prompt=prompt)
                self.assert_invalid("must mention $sample-skill exactly")

    def test_duplicate_unknown_and_non_scalar_agent_fields(self):
        for extra in ('  display_name: "Duplicate"\n', '  unknown: "value"\n', 'interface:\n'):
            with self.subTest(extra=extra):
                self.write_agent(extra=extra)
                self.assert_invalid("openai.yaml")
        self.write_agent()
        self.agent.write_text(self.agent.read_text().replace('"Sample Skill"', "[Sample, Skill]"), encoding="utf-8")
        self.assert_invalid("JSON double-quoted string")

    def test_body_and_template_stubs(self):
        for body in ("\n", "line\n" * 501, "[TODO: Complete the workflow]\n", "Use {{skill_name}} here.\n"):
            with self.subTest(body=body[:40]):
                self.write_skill(body=body)
                self.assert_invalid("body")

    def test_invalid_cli_exit(self):
        self.write_skill(description="[template]")
        result = subprocess.run([sys.executable, str(Path(__file__).with_name("check_skills.py")), str(self.root)], capture_output=True, text=True)
        self.assertEqual(result.returncode, 1)
        self.assertIn(".agents/skills/sample-skill/SKILL.md", result.stderr)


if __name__ == "__main__":
    unittest.main()
