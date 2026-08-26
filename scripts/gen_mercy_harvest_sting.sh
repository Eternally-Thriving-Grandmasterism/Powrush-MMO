#!/usr/bin/env bash
# Generate shared + climate mercy harvest stings (~350ms Vorbis).
# Contact: info@Rathor.ai
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DIR="$ROOT/assets/audio"
mkdir -p "$DIR"

sting() {
  local out="$1" f1="$2" f2="$3" f3="$4"
  ffmpeg -y \
    -f lavfi -i "sine=frequency=${f1}:duration=0.18,afade=t=in:st=0:d=0.02,afade=t=out:st=0.12:d=0.06" \
    -f lavfi -i "sine=frequency=${f2}:duration=0.22,afade=t=in:st=0:d=0.02,afade=t=out:st=0.14:d=0.08" \
    -f lavfi -i "sine=frequency=${f3}:duration=0.35,afade=t=in:st=0:d=0.03,afade=t=out:st=0.22:d=0.13" \
    -filter_complex "[0][1]amix=inputs=2:duration=longest:dropout_transition=0.05,volume=0.55[a];[a][2]amix=inputs=2:duration=longest:weights=0.7 1.0:dropout_transition=0.08,volume=0.7,highpass=f=180,lowpass=f=4200" \
    -c:a libvorbis -q:a 4 "$out"
  echo "wrote $out"
}

# Shared C5 E5 G5
sting "$DIR/mercy_harvest_sting.ogg" 523.25 659.25 783.99
# Sanctuary — warmer, slightly lower
sting "$DIR/mercy_harvest_sting_sanctuary.ogg" 392.00 523.25 659.25
# Verdant — brighter lift
sting "$DIR/mercy_harvest_sting_verdant.ogg" 523.25 698.46 880.00
# Horizon — cooler, more open
sting "$DIR/mercy_harvest_sting_horizon.ogg" 440.00 554.37 739.99
