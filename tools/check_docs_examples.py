#!/usr/bin/env python3

from __future__ import annotations

import html
import re
import subprocess
import sys
from pathlib import Path


EXAMPLE_RE = re.compile(
    r'(?s)<pre[^>]*data-rant-example="true"[^>]*data-expect="([^"]*)"[^>]*><code class="language-rant">(.*?)</code></pre>'
)


def run_example(code: str, cwd: Path) -> str:
    result = subprocess.run(
        ["cargo", "run", "--quiet", "--features", "cli", "--", "--eval", code],
        cwd=cwd,
        text=True,
        capture_output=True,
        check=False,
    )
    if result.returncode != 0:
        raise RuntimeError(result.stderr.strip() or f"example exited with {result.returncode}")
    return result.stdout.rstrip("\n")


def main() -> int:
    root = Path(sys.argv[1] if len(sys.argv) > 1 else "docs").resolve()
    repo_root = root.parent
    failures: list[str] = []
    checked = 0

    for html_file in sorted(root.rglob("*.html")):
        text = html_file.read_text(encoding="utf-8")
        for expected, code in EXAMPLE_RE.findall(text):
            checked += 1
            expected_text = html.unescape(expected)
            code_text = html.unescape(code)
            try:
              actual = run_example(code_text, repo_root)
            except RuntimeError as err:
              failures.append(f"{html_file.relative_to(root)}: {err}")
              continue
            if actual != expected_text:
              failures.append(
                  f"{html_file.relative_to(root)}: expected {expected_text!r}, got {actual!r}"
              )

    if failures:
        print("Documentation example failures:")
        for failure in failures:
            print(f"  {failure}")
        return 1

    print(f"Validated {checked} runnable documentation examples")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
