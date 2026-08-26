#!/usr/bin/env python3
"""Generate a reviewable v4.2 release-note skeleton from repository metadata."""
from pathlib import Path
import datetime

root = Path(__file__).resolve().parents[1]
version = "4.2.0-rc.1"
changelog = root / "CHANGELOG.md"
output = root / "docs" / "releases" / "generated-v4.2.0-rc.1.md"
text = changelog.read_text(encoding="utf-8")
heading = f"## [{version}]"
section = text[text.find(heading):]
if section.startswith(heading):
    section = section.split("\n## ", 1)[0]
else:
    section = "No matching changelog section found."
output.write_text(
    "# Nexus Robotics OS 4.2.0 Release Candidate\n\n"
    f"Generated on {datetime.date.today().isoformat()} from repository metadata.\n\n"
    "## Changelog evidence\n\n" + section +
    "\n\n## Review requirements\n\n"
    "- Confirm version, compatibility, validation, known limitations, and artifact claims.\n"
    "- Replace `NOT RUN` only with evidence from the release environment.\n",
    encoding="utf-8",
)
print(output.relative_to(root))
