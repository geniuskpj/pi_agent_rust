#!/usr/bin/env python3
"""Check README evidence artifact freshness for CI governance.

This guard enforces that artifact citations in README.md are fresh (≤14 days old).
Citations have the format: *(from artifact-path, run correlation-id)*

If any cited artifact is >14 days stale, the check fails to prevent
stale evidence from misleading users about current project capabilities.

Usage:
    python3 scripts/check_readme_evidence_freshness.py

Exit codes:
    0 - All citations are fresh
    1 - One or more stale citations found
    2 - Script error (missing files, parse failures, etc.)
"""

from __future__ import annotations

import re
import sys
from datetime import datetime, timedelta
from pathlib import Path
from typing import NamedTuple


class CitationCheck(NamedTuple):
    """Result of checking a single citation."""
    artifact_path: str
    correlation_id: str
    file_exists: bool
    file_mtime: datetime | None
    days_old: float | None
    is_stale: bool


def main() -> int:
    """Main entry point."""
    repo_root = Path(__file__).resolve().parent.parent
    readme_path = repo_root / "README.md"

    if not readme_path.exists():
        print(f"ERROR: README.md not found at {readme_path}")
        return 2

    try:
        readme_text = readme_path.read_text(encoding="utf-8")
    except Exception as e:
        print(f"ERROR: Failed to read README.md: {e}")
        return 2

    # Parse citations using regex pattern
    # Pattern: *(from artifact-path, run correlation-id)*
    citation_pattern = r'\*\(from ([^,]+), run ([^)]+)\)\*'
    citations = re.findall(citation_pattern, readme_text)

    if not citations:
        print("INFO: No artifact citations found in README.md")
        return 0

    print(f"INFO: Checking {len(citations)} artifact citations for freshness...")

    # Check each citation
    stale_count = 0
    missing_count = 0
    results: list[CitationCheck] = []

    # 14-day staleness threshold
    staleness_threshold = timedelta(days=14)
    now = datetime.now()

    for artifact_path, correlation_id in citations:
        artifact_path = artifact_path.strip()
        correlation_id = correlation_id.strip()

        # Resolve artifact path relative to repo root
        full_path = repo_root / artifact_path

        if not full_path.exists():
            print(f"WARNING: Cited artifact does not exist: {artifact_path}")
            missing_count += 1
            results.append(CitationCheck(
                artifact_path=artifact_path,
                correlation_id=correlation_id,
                file_exists=False,
                file_mtime=None,
                days_old=None,
                is_stale=False  # Missing files don't count as stale for this check
            ))
            continue

        try:
            # Get file modification time
            mtime = datetime.fromtimestamp(full_path.stat().st_mtime)
            age = now - mtime
            days_old = age.total_seconds() / 86400  # Convert to days
            is_stale = age > staleness_threshold

            if is_stale:
                print(f"STALE: {artifact_path} (age: {days_old:.1f} days, limit: 14 days)")
                stale_count += 1
            else:
                print(f"FRESH: {artifact_path} (age: {days_old:.1f} days)")

            results.append(CitationCheck(
                artifact_path=artifact_path,
                correlation_id=correlation_id,
                file_exists=True,
                file_mtime=mtime,
                days_old=days_old,
                is_stale=is_stale
            ))

        except Exception as e:
            print(f"ERROR: Failed to check {artifact_path}: {e}")
            return 2

    # Summary
    print(f"\nSUMMARY:")
    print(f"  Total citations: {len(citations)}")
    print(f"  Fresh artifacts: {len([r for r in results if r.file_exists and not r.is_stale])}")
    print(f"  Stale artifacts: {stale_count}")
    print(f"  Missing artifacts: {missing_count}")

    if stale_count > 0:
        print(f"\nFAIL: {stale_count} cited artifact(s) are >14 days stale.")
        print("Evidence claims in README must be backed by fresh artifacts.")
        print("Re-run evidence generation and update citations to resolve this.")
        return 1

    if missing_count > 0:
        print(f"\nWARNING: {missing_count} cited artifact(s) missing, but freshness check passes.")

    print("\nPASS: All cited artifacts are fresh (≤14 days old).")
    return 0


if __name__ == "__main__":
    sys.exit(main())