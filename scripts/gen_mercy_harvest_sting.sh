#!/usr/bin/env bash
# Generate assets/audio/mercy_harvest_sting.ogg
# Soft C5–E5–G5 mercy triad, ~350ms, never harsh.
# Contact: info@Rathor.ai
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/assets/audio/mercy_harvest_sting.ogg"
mkdir -p "$ROOT/assets/audio"
ffmpeg -y \
  -f lavfi -i "sine=frequency=523.25:duration=0.18,afade=t=in:st=0:d=0.02,afade=t=out:st=0.12:d=0.06" \
  -f lavfi -i "sine=frequency=659.25:duration=0.22,afade=t=in:st=0:d=0.02,afade=t=out:st=0.14:d=0.08" \
  -f lavfi -i "sine=frequency=783.99:duration=0.35,afade=t=in:st=0:d=0.03,afade=t=out:st=0.22:d=0.13" \
  -filter_complex "[0][1]amix=inputs=2:duration=longest:dropout_transition=0.05,volume=0.55[a];[a][2]amix=inputs=2:duration=longest:weights=0.7 1.0:dropout_transition=0.08,volume=0.7,highpass=f=180,lowpass=f=4200" \
  -c:a libvorbis -q:a 4 "$OUT"
echo "wrote $OUT"
