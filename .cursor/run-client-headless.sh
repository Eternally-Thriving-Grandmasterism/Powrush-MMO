#!/usr/bin/env bash
# Run the lived-hour client (`powrush-client`) on a headless Cloud Agent using
# a virtual X display + software Vulkan (lavapipe/llvmpipe). Renders exactly
# like `cargo run -p powrush-client` on a desktop, minus a physical GPU.
#
# Usage:
#   .cursor/run-client-headless.sh            # boots the game on DISPLAY :99
#   POWRUSH_GEN=light .cursor/run-client-headless.sh   # G0 light gen on
#
# Q2 / CI one-frame (timeout-friendly):
#   POWRUSH_NET=off POWRUSH_Q2_FRAME=/tmp/powrush-q2.png \
#     timeout 45s .cursor/run-client-headless.sh
# PASS = process started AND that frame file exists.
# Grab a screenshot from another shell with:  DISPLAY=:99 scrot /tmp/powrush-q2.png
# Do not compare pixels to a golden. Lavapipe is software Vulkan — not a
# Steam Deck / real GPU claim. Title Online stays grey.
set -euo pipefail

# Title Online grey. Unset/off only — this door does not listen.
unset POWRUSH_NET || true
export POWRUSH_NET=off

DISPLAY_NUM="${POWRUSH_DISPLAY:-:99}"
export XDG_RUNTIME_DIR="${XDG_RUNTIME_DIR:-/tmp/xdg-runtime}"
mkdir -p "$XDG_RUNTIME_DIR" && chmod 700 "$XDG_RUNTIME_DIR"

# Start a virtual display sized to the Steam Deck title resolution if one
# is not already running.
if ! DISPLAY="$DISPLAY_NUM" xdpyinfo >/dev/null 2>&1; then
  echo "Starting Xvfb on $DISPLAY_NUM ..."
  Xvfb "$DISPLAY_NUM" -screen 0 1280x800x24 -ac +extension GLX +render -noreset \
    >/tmp/xvfb.log 2>&1 &
  sleep 2
fi

export DISPLAY="$DISPLAY_NUM"
# Force the lavapipe software Vulkan ICD so wgpu picks it deterministically.
if [[ -z "${VK_ICD_FILENAMES:-}" ]]; then
  for icd in \
    /usr/share/vulkan/icd.d/lvp_icd.json \
    /usr/share/vulkan/icd.d/lvp_icd.x86_64.json \
    /usr/share/vulkan/icd.d/lvp_icd.i686.json
  do
    if [[ -f "$icd" ]]; then
      export VK_ICD_FILENAMES="$icd"
      break
    fi
  done
fi
if [[ -z "${VK_ICD_FILENAMES:-}" || ! -f "${VK_ICD_FILENAMES}" ]]; then
  echo "FAIL: lavapipe Vulkan ICD not found (install mesa-vulkan-drivers)."
  exit 1
fi
export WGPU_BACKEND="${WGPU_BACKEND:-vulkan}"

echo "Launching powrush-client (software Vulkan / lavapipe) on $DISPLAY ..."
echo "POWRUSH_NET=$POWRUSH_NET (Title Online grey; no listen)."

Q2_FRAME="${POWRUSH_Q2_FRAME:-}"
if [[ -z "$Q2_FRAME" ]]; then
  exec cargo run -p powrush-client "$@"
fi

# --- Q2 capture-if-path: boot, wait for the window, one frame, then exit ---
echo "Q2 frame path: $Q2_FRAME (scrot; not a golden; lavapipe ≠ Steam Deck)"
cargo run -p powrush-client "$@" &
client_pid=$!

cleanup() {
  if kill -0 "$client_pid" 2>/dev/null; then
    kill "$client_pid" 2>/dev/null || true
    wait "$client_pid" 2>/dev/null || true
  fi
}
trap cleanup EXIT INT TERM

grab_frame() {
  local out="$1"
  if command -v scrot >/dev/null 2>&1; then
    rm -f "$out"
    scrot "$out"
  elif command -v import >/dev/null 2>&1; then
    import -window root "$out"
  else
    echo "FAIL: neither scrot nor imagemagick import is on PATH"
    return 1
  fi
}

deadline=$((SECONDS + 35))
window_up=0
while (( SECONDS < deadline )); do
  if ! kill -0 "$client_pid" 2>/dev/null; then
    wait "$client_pid" || true
    echo "FAIL: process died before the frame"
    exit 1
  fi
  if xwininfo -root -tree 2>/dev/null | grep -qi 'Powrush'; then
    window_up=1
    break
  fi
  sleep 1
done

if [[ "$window_up" -ne 1 ]]; then
  echo "FAIL: window never appears"
  exit 1
fi

# One presentable frame — do not compare pixels.
sleep 2
if ! kill -0 "$client_pid" 2>/dev/null; then
  echo "FAIL: process died before the frame"
  exit 1
fi

grab_frame "$Q2_FRAME"
if [[ ! -s "$Q2_FRAME" ]]; then
  echo "FAIL: frame file missing or empty: $Q2_FRAME"
  exit 1
fi

echo "Q2 PASS: process started and frame exists at $Q2_FRAME ($(wc -c < "$Q2_FRAME") bytes; no pixel compare)"
exit 0
