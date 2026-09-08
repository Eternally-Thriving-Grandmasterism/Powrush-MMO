#!/usr/bin/env bash
# Run the lived-hour client (`powrush-client`) on a headless Cloud Agent using
# a virtual X display + software Vulkan (lavapipe/llvmpipe). Renders exactly
# like `cargo run -p powrush-client` on a desktop, minus a physical GPU.
#
# Usage:
#   .cursor/run-client-headless.sh            # boots the game on DISPLAY :99
#   POWRUSH_GEN=light .cursor/run-client-headless.sh   # G0 light gen on
#
# Grab a screenshot from another shell with:  DISPLAY=:99 scrot out.png
set -euo pipefail

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
export VK_ICD_FILENAMES="${VK_ICD_FILENAMES:-/usr/share/vulkan/icd.d/lvp_icd.json}"
export WGPU_BACKEND="${WGPU_BACKEND:-vulkan}"

echo "Launching powrush-client (software Vulkan / lavapipe) on $DISPLAY ..."
exec cargo run -p powrush-client "$@"
