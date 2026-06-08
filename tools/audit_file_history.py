#!/usr/bin/env python3
"""
Powrush-MMO SHA History Auditing Tool
Version: v1.0 | 2026-06-08

Automates the mandatory History Sanity Check from docs/RESTORATION_AND_MERGE_PROTOCOL.md

Usage:
  python tools/audit_file_history.py game/resource_nodes.rs
  python tools/audit_file_history.py engine/gpu_patsagi_bridge.rs --limit 5

Features:
- Lists recent commits/SHAs for the file
- Fetches raw content at each SHA (via local git or GitHub raw)
- Detects common layering symptoms: duplicate structs, placeholder comments, mixed field names
- Outputs clear Restoration Report with recommendations
- Designed for both human developers and AI agents (Ra-Thor, PATSAGi Councils)

Run from repo root. Requires: git + python3 + requests (optional for GitHub API)
"""

import argparse
import subprocess
import sys
import re
from pathlib import Path

try:
    import requests
    HAS_REQUESTS = True
except ImportError:
    HAS_REQUESTS = False


SYMPTOM_PATTERNS = {
    "duplicate_struct": r"pub struct (ResourceNode|ResourceNodeManager|GpuPatsagiResponse)",
    "placeholder_comment": r"/\*\s*now\s*\+|/\*\s*\.\.\.|//\s*\.\.\. existing|//\s*\.\.\.",
    "mixed_field_id": r"\b(node_id|id)\b.*: u64",
    "mixed_position": r"position:\s*(Vec3|\(f32|f32, f32, f32\))",
    "legacy_harvest": r"last_harvested_ms|last_harvest_ms",
}


def run_git_command(args: list[str]) -> str:
    """Run a git command and return stdout."""
    try:
        result = subprocess.run(
            ["git"] + args,
            capture_output=True,
            text=True,
            check=True,
            cwd=Path.cwd(),
        )
        return result.stdout.strip()
    except subprocess.CalledProcessError as e:
        print(f"Git error: {e.stderr}", file=sys.stderr)
        sys.exit(1)


def get_recent_commits(file_path: str, limit: int = 8) -> list[tuple[str, str]]:
    """Return list of (sha, message) for recent commits touching the file."""
    output = run_git_command([
        "log", "--oneline", f"-{limit}", "--", file_path
    ])
    commits = []
    for line in output.splitlines():
        if line.strip():
            parts = line.split(" ", 1)
            if len(parts) == 2:
                commits.append((parts[0], parts[1]))
    return commits


def get_file_at_sha(file_path: str, sha: str) -> str:
    """Get full file content at a specific commit SHA using local git show."""
    try:
        return run_git_command(["show", f"{sha}:{file_path}"])
    except SystemExit:
        return ""


def analyze_content(content: str, sha: str) -> dict:
    """Scan content for known layering symptoms."""
    findings = {"sha": sha, "issues": []}
    for name, pattern in SYMPTOM_PATTERNS.items():
        matches = re.findall(pattern, content)
        if matches:
            findings["issues"].append({
                "type": name,
                "count": len(matches),
                "examples": matches[:3],  # first few examples
            })
    # Extra heuristic: count occurrences of key struct definitions
    struct_count = len(re.findall(r"^pub struct (ResourceNode|ResourceNodeManager)", content, re.MULTILINE))
    if struct_count > 1:
        findings["issues"].append({
            "type": "multiple_struct_definitions",
            "count": struct_count,
        })
    return findings


def generate_restoration_report(file_path: str, analyses: list[dict], commits: list[tuple[str, str]]) -> str:
    """Produce human- and AI-readable restoration report."""
    report = [
        f"# Restoration Audit Report for {file_path}",
        f"Generated: {subprocess.check_output(['date']).decode().strip()}",
        f"Commits analyzed: {len(commits)}",
        "",
        "## Recent Commit History",
    ]
    for sha, msg in commits:
        report.append(f"- {sha}  {msg}")

    report.append("
## Symptom Analysis per Commit (newest first)")
    has_issues = False
    for analysis in analyses:
        sha = analysis["sha"]
        issues = analysis["issues"]
        if issues:
            has_issues = True
            report.append(f"\n### {sha}")
            for issue in issues:
                report.append(f"  - [{issue['type']}] count={issue.get('count', 1)}")
                if "examples" in issue:
                    report.append(f"    examples: {issue['examples']}")

    if not has_issues:
        report.append("\n✅ No layering symptoms detected in recent history. Clean state.")
    else:
        report.append(
            "\n⚠️  Layering symptoms detected. Recommended action per RESTORATION_AND_MERGE_PROTOCOL.md:"
        )
        report.append("1. Perform full intelligent merge of valuable logic from all listed commits.")
        report.append("2. Produce ONE ultimate clean full file (no duplicates, no placeholders).")
        report.append("3. Push to dedicated restoration branch and open PR with this report.")
        report.append("4. Reference this audit in the PR description.")

    report.append(
        "\n## Next Steps for AI Agents (Ra-Thor / PATSAGi Councils)"
    )
    report.append("- Use this report as input to council deliberation.")
    report.append("- Generate the merged full file following the protocol.")
    report.append("- Never discard concrete gameplay systems (harvest, regenerate, etc.).")

    return "\n".join(report)


def main():
    parser = argparse.ArgumentParser(description="Automate SHA history auditing for Powrush-MMO restoration protocol")
    parser.add_argument("file_path", help="Path to the file to audit (e.g. game/resource_nodes.rs)")
    parser.add_argument("--limit", type=int, default=8, help="Number of recent commits to analyze (default 8)")
    args = parser.parse_args()

    file_path = args.file_path
    if not Path(file_path).exists() and not file_path.startswith(("game/", "engine/", "shared/", "client/")):
        print(f"Warning: {file_path} not found locally. Will try git history anyway.", file=sys.stderr)

    print(f"Auditing history for: {file_path} (last {args.limit} commits)")

    commits = get_recent_commits(file_path, args.limit)
    if not commits:
        print("No commits found for this file.", file=sys.stderr)
        sys.exit(1)

    analyses = []
    for sha, _msg in commits:
        content = get_file_at_sha(file_path, sha)
        if content:
            analysis = analyze_content(content, sha)
            analyses.append(analysis)

    report = generate_restoration_report(file_path, analyses, commits)
    print(report)

    # Also write to file for easy inclusion in PRs
    report_path = Path("audit_report_" + Path(file_path).name.replace("/", "_") + ".md")
    report_path.write_text(report)
    print(f"\nReport also written to: {report_path}")


if __name__ == "__main__":
    main()
