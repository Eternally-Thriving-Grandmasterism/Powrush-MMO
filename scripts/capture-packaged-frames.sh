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
D="${POWRUSH_DISPLAY:-:199}"; W="${POWRUSH_CAPTURE_W:-1920}" H="${POWRUSH_CAPTURE_H:-1080}" N="${POWRUSH_CAPTURE_N:-6}" DL="${POWRUSH_CAPTURE_DEADLINE:-150}"
export XDG_RUNTIME_DIR="${XDG_RUNTIME_DIR:-/tmp/xdg-runtime}"; mkdir -p "$XDG_RUNTIME_DIR" && chmod 700 "$XDG_RUNTIME_DIR"
for icd in /usr/share/vulkan/icd.d/lvp_icd*.json; do [[ -f "$icd" ]] && export VK_ICD_FILENAMES="$icd" && break; done
[[ -n "${VK_ICD_FILENAMES:-}" ]] || { echo "FAIL: lavapipe ICD missing" >&2; exit 1; }
if ! DISPLAY="$D" xdpyinfo >/dev/null 2>&1; then Xvfb "$D" -screen 0 "${W}x${H}x24" -ac +extension GLX +render -noreset >/dev/null 2>&1 & sleep 2; fi
export DISPLAY="$D"
(cd "$ROOT" && exec ./powrush-client) > >(tee "${OUT}/.client.log") 2>&1 & pid=$!
end=$((SECONDS + DL)); up=0
while ((SECONDS < end )); do
  kill -0 "$pid" 2>/dev/null || { echo "FAIL: client died before window" >&2; exit 1; }
  xwininfo -root -tree 2>/dev/null | grep -qi Powrush && { up=1; break; }
  sleep 1
done
[[ "$up" -eq 1 ]] || { echo "FAIL: Powrush window missing" >&2; exit 1; }
wid=$(xdotool search --name Powrush 2>/dev/null | head -1 || true); [[ -n "$wid" ]] || { echo "FAIL: no Powrush window id" >&2; exit 1; }
xdotool windowmove "$wid" 0 0 windowsize --sync "$wid" "$W" "$H" windowfocus --sync "$wid" || { echo "FAIL: xdotool resize/focus" >&2; exit 1; }
echo "window ${wid} resized to $(xwininfo -id "$wid" | awk '/Width:/{w=$2} /Height:/{h=$2} END{print w "x" h}')" >&2
settle_end=$end; ready=0; sleep 2
while (( SECONDS < settle_end )); do
  kill -0 "$pid" 2>/dev/null || { echo "FAIL: client died before settle" >&2; exit 1; }
  grab "${OUT}/.probe.png"
  k=$(convert "${OUT}/.probe.png" -format %k info:); echo "probe colours=${k}" >&2
  [[ "$k" -gt 1 ]] && { ready=1; break; }
  (( SECONDS + 3 < settle_end )) || break; sleep 3
done
[[ "$ready" -eq 1 ]] || { echo "FAIL: never got a multi-colour frame" >&2; exit 1; }
until grab "${OUT}/.pre.png"; (( $(convert "${OUT}/.pre.png" -format %k info:) > 256 )); do (( SECONDS + 2 < end )) || { echo "FAIL: no rendered pre-Return frame" >&2; exit 1; }; sleep 2; done
echo "pre frame $(identify -format '%wx%h' "${OUT}/.pre.png") colours=$(convert "${OUT}/.pre.png" -format %k info:); sending XTEST Return" >&2; xdotool key Return; sleep 3
for n in $(seq 1 "$N"); do
  (( SECONDS < end )) || { echo "FAIL: deadline before frame $n" >&2; exit 1; }
  kill -0 "$pid" 2>/dev/null || { echo "FAIL: client died before frame $n" >&2; exit 1; }
  f="${OUT}/frame-$(printf '%02d' "$n").png"; grab "$f"
  [[ "$(identify -format '%wx%h' "$f")" == "${W}x${H}" ]] || { echo "FAIL: not ${W}x${H}: $f" >&2; exit 1; }
  [[ "$(convert "$f" -format %k info:)" -gt 1 ]] || { echo "FAIL: single colour: $f" >&2; exit 1; }
  for e in "East:8x${H}" "South:${W}x8" "West:8x${H}" "North:${W}x8"; do c=$(convert "$f" -gravity "${e%%:*}" -crop "${e#*:}+0+0" +repage -format %k info:); echo "frame ${n} $(identify -format '%wx%h' "$f") ${e%%:*} edge colours=${c}" >&2; [[ "$c" -gt 1 ]] || { echo "FAIL: blank ${e%%:*} edge (window not ${W}x${H}?): $f" >&2; exit 1; }; done
  (( n < N && SECONDS + 3 < end )) && sleep 3
done
ae=$(compare -metric AE "${OUT}/.pre.png" "${OUT}/frame-01.png" null: 2>&1 | awk '{printf "%d", $1}' || true); echo "pre/post AE=${ae}" >&2
[[ "${ae:-0}" -gt 0 ]] || { echo "FAIL: frame-01 identical to pre-Return frame" >&2; exit 1; }
nu=$(grep -cF 'lived UI camera off (POWRUSH_CAPTURE_NO_UI)' "${OUT}/.client.log" || true); echo "powrush::capture no-UI line count=${nu} (want $([[ -n "${POWRUSH_CAPTURE_NO_UI:-}" ]] && echo 1 || echo 0))" >&2; [[ "$nu" == "$([[ -n "${POWRUSH_CAPTURE_NO_UI:-}" ]] && echo 1 || echo 0)" ]] || { echo "FAIL: powrush::capture no-UI line count ${nu}" >&2; exit 1; }
printf 'sha=%s\n%sx%s lavapipe yard frames after Return%s; not a real-GPU or store claim\n' "${POWRUSH_CAPTURE_SHA:-unknown}" "$W" "$H" "${POWRUSH_CAPTURE_NO_UI:+, UI camera off}" >"$OUT/CAPTURE-ID.txt"
kill "$pid" 2>/dev/null || true; wait "$pid" 2>/dev/null || true; echo "PASS: ${N} ${W}x${H} frames after Return (pre/post AE=${ae}) in $OUT"
