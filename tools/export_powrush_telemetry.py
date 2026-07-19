#!/usr/bin/env python3
"""
export_powrush_telemetry.py

Emit powrush_telemetry_v1 / powrush_telemetry_batch_v1 JSON for Ra-Thor
reality-thriving-transfer ingest.

Contract mirror:
  https://github.com/Eternally-Thriving-Grandmasterism/Ra-Thor
  crates/reality-thriving-transfer/POWRUSH_TELEMETRY_CONTRACT.md

Usage:
  python3 tools/export_powrush_telemetry.py
  python3 tools/export_powrush_telemetry.py --batch -o /tmp/batch.json
  python3 tools/export_powrush_telemetry.py --profile high_mercy

Contact: info@Rathor.ai
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Any, Dict, List

PROFILES: Dict[str, Dict[str, Any]] = {
    "high_mercy": {
        "label": "high_mercy_council_session",
        "telemetry": {
            "gameplay_hours": 86.5,
            "rbe_decision_quality_avg": 0.91,
            "peaceful_resolution_rate": 0.94,
            "collaboration_events": 420,
            "ethical_choice_score": 0.89,
            "adaptation_events": 175,
            "abundance_velocity_signals": 1.55,
            "innovation_contribution": 0.81,
        },
    },
    "marginal": {
        "label": "marginal_ethics_session",
        "telemetry": {
            "gameplay_hours": 14.0,
            "rbe_decision_quality_avg": 0.48,
            "peaceful_resolution_rate": 0.51,
            "collaboration_events": 42,
            "ethical_choice_score": 0.44,
            "adaptation_events": 18,
            "abundance_velocity_signals": 0.72,
            "innovation_contribution": 0.35,
        },
    },
    "early": {
        "label": "early_player_onboarding",
        "telemetry": {
            "gameplay_hours": 3.25,
            "rbe_decision_quality_avg": 0.62,
            "peaceful_resolution_rate": 0.70,
            "collaboration_events": 12,
            "ethical_choice_score": 0.66,
            "adaptation_events": 8,
            "abundance_velocity_signals": 0.95,
            "innovation_contribution": 0.40,
        },
    },
}


def envelope(profile_key: str) -> Dict[str, Any]:
    p = PROFILES[profile_key]
    return {
        "schema": "powrush_telemetry_v1",
        "source": "powrush-mmo",
        "label": p["label"],
        "telemetry": p["telemetry"],
    }


def batch_envelope(keys: List[str]) -> Dict[str, Any]:
    sessions = []
    for k in keys:
        p = PROFILES[k]
        sessions.append({"label": p["label"], "telemetry": p["telemetry"]})
    return {
        "schema": "powrush_telemetry_batch_v1",
        "source": "powrush-mmo",
        "label": "batch_" + "_".join(keys),
        "sessions": sessions,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description="Export Powrush telemetry for Ra-Thor")
    parser.add_argument(
        "--profile",
        choices=list(PROFILES.keys()),
        default="high_mercy",
        help="Single-session profile (default: high_mercy)",
    )
    parser.add_argument(
        "--batch",
        action="store_true",
        help="Export all profiles as powrush_telemetry_batch_v1",
    )
    parser.add_argument(
        "-o",
        "--output",
        type=Path,
        help="Write JSON to file (default: stdout)",
    )
    args = parser.parse_args()

    payload = batch_envelope(list(PROFILES.keys())) if args.batch else envelope(args.profile)
    text = json.dumps(payload, indent=2) + "\n"

    if args.output:
        args.output.write_text(text, encoding="utf-8")
        print(f"Wrote {args.output}", file=sys.stderr)
    else:
        sys.stdout.write(text)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
