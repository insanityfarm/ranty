#!/usr/bin/env python3

from __future__ import annotations

import re
import sys
from pathlib import Path


LINK_RE = re.compile(r'(?:href|src)="([^"]+)"')


def main() -> int:
    root = Path(sys.argv[1] if len(sys.argv) > 1 else "docs").resolve()
    failures: list[str] = []

    for html_file in sorted(root.rglob("*.html")):
        text = html_file.read_text(encoding="utf-8")
        for target in LINK_RE.findall(text):
            if target.startswith(("http://", "https://", "mailto:", "data:", "javascript:")):
                continue
            if target.startswith("#"):
                continue
            clean = target.split("#", 1)[0].split("?", 1)[0]
            if not clean:
                continue
            resolved = (html_file.parent / clean).resolve()
            if not resolved.exists():
                failures.append(f"{html_file.relative_to(root)} -> {target}")

    if failures:
        print("Broken documentation links:")
        for failure in failures:
            print(f"  {failure}")
        return 1

    print(f"Validated documentation links in {root}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
