#!/usr/bin/env bash
# Steam Offline 1.0 door — lived client only.
# No POWRUSH_NET. No listen. No 0.0.0.0. Title Online stays grey.
# CARD Q1: `q1` / `--script-run` = machine six-landing pass (no interactive walk).
# Contact: info@Rathor.ai
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

# Force Offline SKU: never inherit a lab net flag from the shell.
unset POWRUSH_NET || true
export POWRUSH_NET=off

q1_lib_test_ok() {
  local name="$1"
  local log="$2"
  grep -E "${name}[[:space:]]+\\.\\.\\.[[:space:]]+ok$" "$log" >/dev/null 2>&1
}

q1_mark() {
  local ok="$1"
  if [[ "$ok" == "1" ]]; then
    echo "PASS"
  else
    echo "FAIL"
  fi
}

q1_script_run() {
  echo "powrush: Offline door → CARD Q1 SCRIPT-RUN (POWRUSH_NET=off)"
  echo "powrush: Title Online stays grey. No public bind. No interactive walk. No WASD."

  if [[ "${POWRUSH_NET:-}" != "off" ]]; then
    echo "FAIL: POWRUSH_NET must stay off (door refused Online)."
    exit 1
  fi

  local dated
  dated="$(date -u +%Y-%m-%d)"
  local core_shared=0
  local core_lib=0
  local people=0
  local garden_light=0
  local garden_stance=0
  local lib_log
  lib_log="$(mktemp)"
  trap 'rm -f "$lib_log"' RETURN

  set +e
  cargo test -p shared -p rsil-identity
  local shared_rc=$?
  set -e
  if [[ "$shared_rc" -eq 0 ]]; then
    core_shared=1
  fi

  set +e
  cargo test -p powrush-client --lib 2>&1 | tee "$lib_log"
  local lib_rc=${PIPESTATUS[0]}
  set -e
  if [[ "$lib_rc" -eq 0 ]]; then
    core_lib=1
  fi

  if q1_lib_test_ok "q0_each_people_landing_s1_f7_aftermath_line" "$lib_log"; then
    people=1
  fi
  if q1_lib_test_ok "q0_new_soul_light_sealed_continue_dress" "$lib_log"; then
    garden_light=1
  fi
  if q1_lib_test_ok "q0_f1_stance_four_values_garden_no_stance" "$lib_log"; then
    garden_stance=1
  fi

  local garden=0
  if [[ "$people" -eq 1 && "$garden_light" -eq 1 && "$garden_stance" -eq 1 ]]; then
    garden=1
  fi

  echo
  echo "CARD Q1 SCRIPT-RUN ${dated}"
  echo "door: ./scripts/play-offline.sh --script-run · POWRUSH_NET=off · no cargo run · no WASD"
  echo "verified: cargo test -p shared -p rsil-identity && cargo test -p powrush-client --lib (named q0_* landing proofs)"
  echo "core shared+rsil-identity: $(q1_mark "$core_shared")"
  echo "core powrush-client --lib: $(q1_mark "$core_lib")"
  echo "Human / Sanctuary yard: $(q1_mark "$people")"
  echo "Ambrosian / Sanctuary well-from-above: $(q1_mark "$people")"
  echo "Cydruid / Heartwood: $(q1_mark "$people")"
  echo "Quellorian / Threshold: $(q1_mark "$people")"
  echo "Draek / Depths (teal way-home): $(q1_mark "$people")"
  echo "Garden / light path: $(q1_mark "$garden")"

  if [[ "$core_shared" -eq 1 && "$core_lib" -eq 1 && "$people" -eq 1 && "$garden" -eq 1 ]]; then
    exit 0
  fi
  exit 1
}

MODE="${1:-}"
if [[ "$MODE" == "q1" || "$MODE" == "--script-run" ]]; then
  q1_script_run
fi

echo "powrush: Offline door → cargo run -p powrush-client (POWRUSH_NET=off)"
echo "powrush: Title Online stays grey. No public bind."
exec cargo run -p powrush-client "$@"
