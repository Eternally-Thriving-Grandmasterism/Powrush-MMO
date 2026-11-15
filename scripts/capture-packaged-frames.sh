#!/usr/bin/env bash
set -euo pipefail
ROOT="${1:?usage: $0 <extracted-dist-dir> [out-dir]}"
OUT="${2:-${ROOT}/media-capture-frames}"
CLIENT="${ROOT}/powrush-client"
[[ -x "$CLIENT" ]] || { echo "FAIL: missing $CLIENT" >&2; exit 1; }
command -v import >/dev/null && command -v identify >/dev/null && command -v convert >/dev/null \
  || { echo "FAIL: need imagemagick import/identify/convert" >&2; exit 1; }
grab() { import -window root "$1"; }; mkdir -p "$OUT"
export POWRUSH_NET=off POWRUSH_USER_DIR="$(mktemp -d)" WGPU_BACKEND=vulkan
mkdir -p "$POWRUSH_USER_DIR"; printf '%s\n' '{"schema":"powrush_settings_v1","age_confirmed":true}' >"$POWRUSH_USER_DIR/powrush_settings.json"
export POWRUSH_Q2_FRAME="${OUT}/.q2skip.png"
D="${POWRUSH_DISPLAY:-:199}"
export XDG_RUNTIME_DIR="${XDG_RUNTIME_DIR:-/tmp/xdg-runtime}"; mkdir -p "$XDG_RUNTIME_DIR" && chmod 700 "$XDG_RUNTIME_DIR"
for icd in /usr/share/vulkan/icd.d/lvp_icd*.json; do [[ -f "$icd" ]] && export VK_ICD_FILENAMES="$icd" && break; done
[[ -n "${VK_ICD_FILENAMES:-}" ]] || { echo "FAIL: lavapipe ICD missing" >&2; exit 1; }
if ! DISPLAY="$D" xdpyinfo >/dev/null 2>&1; then Xvfb "$D" -screen 0 1280x800x24 -ac +extension GLX +render -noreset >/dev/null 2>&1 & sleep 2; fi
export DISPLAY="$D"
(cd "$ROOT" && ./powrush-client) & pid=$!
end=$((SECONDS + 85)); up=0
while ((SECONDS < end )); do
  kill -0 "$pid" 2>/dev/null || { echo "FAIL: client died before window" >&2; exit 1; }
  xwininfo -root -tree 2>/dev/null | grep -qi Powrush && { up=1; break; }
  sleep 1
done
[[ "$up" -eq 1 ]] || { echo "FAIL: Powrush window missing" >&2; exit 1; }
settle_end=$end; ready=0; sleep 2
while (( SECONDS < settle_end )); do
  kill -0 "$pid" 2>/dev/null || { echo "FAIL: client died before settle" >&2; exit 1; }
  grab "${OUT}/.probe.png"
  k=$(convert "${OUT}/.probe.png" -format %k info:); echo "probe colours=${k}" >&2
  [[ "$k" -gt 1 ]] && { ready=1; break; }
  (( SECONDS + 3 < settle_end )) || break; sleep 3
done
[[ "$ready" -eq 1 ]] || { echo "FAIL: never got a multi-colour frame" >&2; exit 1; }
for n in 1 2 3 4 5 6; do
  (( SECONDS < end )) || { echo "FAIL: deadline before frame $n" >&2; exit 1; }
  kill -0 "$pid" 2>/dev/null || { echo "FAIL: client died before frame $n" >&2; exit 1; }
  f="${OUT}/frame-$(printf '%02d' "$n").png"; grab "$f"
  [[ "$(identify -format '%wx%h' "$f")" == "1280x800" ]] || { echo "FAIL: not 1280x800: $f" >&2; exit 1; }
  [[ "$(convert "$f" -format %k info:)" -gt 1 ]] || { echo "FAIL: single colour: $f" >&2; exit 1; }
  (( n < 6 && SECONDS + 3 < end )) && sleep 3
done
printf 'sha=%s\n1280x800 lavapipe; not a real-GPU or store claim\n' "${POWRUSH_CAPTURE_SHA:-unknown}" >"$OUT/CAPTURE-ID.txt"
kill "$pid" 2>/dev/null || true; wait "$pid" 2>/dev/null || true; echo "PASS: 6 frames in $OUT"
