#!/usr/bin/env bash
# Steam Offline 1.0 door — lived client only.
# No POWRUSH_NET. No listen. No 0.0.0.0. Title Online stays grey.
# Contact: info@Rathor.ai
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

# Force Offline SKU: never inherit a lab net flag from the shell.
unset POWRUSH_NET || true
export POWRUSH_NET=off

echo "powrush: Offline door → cargo run -p powrush-client (POWRUSH_NET=off)"
echo "powrush: Title Online stays grey. No public bind."
exec cargo run -p powrush-client "$@"
