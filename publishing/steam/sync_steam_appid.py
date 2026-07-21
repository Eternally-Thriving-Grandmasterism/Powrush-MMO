#!/usr/bin/env python3
"""Write steam_appid.txt from publishing/steam/steam_cloud_config.json

Usage:
  python publishing/steam/sync_steam_appid.py --development   # 480 Spacewar
  python publishing/steam/sync_steam_appid.py --shipping      # requires app_id.shipping set
  python publishing/steam/sync_steam_appid.py --check         # print resolution, no write

Never ship steam_appid.txt in production depots.
Contact: info@Rathor.ai
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CONFIG_PATH = Path(__file__).resolve().parent / "steam_cloud_config.json"
TARGETS = [
    ROOT / "steam_appid.txt",
    ROOT / "client" / "steam_appid.txt",
]


def load_config() -> dict:
    with CONFIG_PATH.open(encoding="utf-8") as f:
        return json.load(f)


def resolve_app_id(cfg: dict, mode: str) -> int:
    app = cfg.get("app_id") or {}
    if mode == "shipping":
        shipping = app.get("shipping")
        if shipping is None:
            raise SystemExit(
                "ERROR: app_id.shipping is null in steam_cloud_config.json.\n"
                "Set the real Steam AppID, then re-run with --shipping."
            )
        return int(shipping)
    return int(app.get("development") or 480)


def write_appid(app_id: int) -> None:
    text = f"{app_id}\n"
    for path in TARGETS:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")
        print(f"Wrote {path} → {app_id}")


def main() -> None:
    parser = argparse.ArgumentParser(description="Sync steam_appid.txt from partner config")
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--development", action="store_true", help="Use development AppID (480)")
    group.add_argument("--shipping", action="store_true", help="Use shipping AppID (must be set)")
    group.add_argument("--check", action="store_true", help="Print resolved IDs only")
    args = parser.parse_args()

    cfg = load_config()
    app = cfg.get("app_id") or {}
    print(f"Config: {CONFIG_PATH}")
    print(f"  development: {app.get('development')}")
    print(f"  shipping:    {app.get('shipping')}")

    if args.check:
        return

    mode = "shipping" if args.shipping else "development"
    app_id = resolve_app_id(cfg, mode)
    if mode == "development" and app_id == 480:
        print("WARNING: Using Spacewar (480). Do not ship this AppID.")
    write_appid(app_id)
    print("Done. Shipping depots must strip steam_appid.txt.")


if __name__ == "__main__":
    main()
