#!/usr/bin/env python3
"""Verify relative Markdown links resolve inside the repository.

External URLs are intentionally skipped because this release gate verifies the
repository can be navigated without network availability. External-source
freshness and legal review remain separate evidence.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path
from urllib.parse import unquote, urlsplit

LINK_RE = re.compile(r"!?\[[^\]]*\]\(([^)]+)\)")
FENCE_RE = re.compile(r"^\s*(```|~~~)")
SKIPPED_SCHEMES = {"http", "https", "mailto", "tel"}


def markdown_without_fenced_code(text: str) -> str:
    output: list[str] = []
    fence: str | None = None
    for line in text.splitlines():
        match = FENCE_RE.match(line)
        if match:
            marker = match.group(1)
            if fence is None:
                fence = marker
            elif marker == fence:
                fence = None
            continue
        if fence is None:
            output.append(line)
    return "\n".join(output)


def extract_target(raw: str) -> str:
    target = raw.strip()
    if target.startswith("<") and ">" in target:
        return target[1 : target.index(">")] 
    # Markdown permits an optional quoted title after whitespace.
    if " \"" in target:
        target = target.split(" \"", 1)[0]
    if " '" in target:
        target = target.split(" '", 1)[0]
    return target


def main() -> int:
    root = Path(__file__).resolve().parents[1]
    failures: list[str] = []

    for document in sorted(root.rglob("*.md")):
        if any(part in {".git", "target"} for part in document.parts):
            continue
        text = markdown_without_fenced_code(document.read_text(encoding="utf-8"))
        for match in LINK_RE.finditer(text):
            target = extract_target(match.group(1))
            if not target or target.startswith("#"):
                continue
            parsed = urlsplit(target)
            if parsed.scheme.lower() in SKIPPED_SCHEMES or parsed.netloc:
                continue
            path_text = unquote(parsed.path)
            if not path_text:
                continue
            resolved = (document.parent / path_text).resolve()
            try:
                resolved.relative_to(root)
            except ValueError:
                failures.append(
                    f"{document.relative_to(root)}: link escapes repository: {target}"
                )
                continue
            if not resolved.exists():
                failures.append(
                    f"{document.relative_to(root)}: missing link target: {target}"
                )

    if failures:
        print("Relative Markdown link verification failed:", file=sys.stderr)
        for failure in failures:
            print(f"- {failure}", file=sys.stderr)
        return 1

    print("All relative Markdown links resolve inside the repository.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
