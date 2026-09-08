#!/usr/bin/env bash
# Quiet Peace-yard bed + well sting (Vorbis).
# Contact: info@Rathor.ai
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DIR="$ROOT/assets/audio"
mkdir -p "$DIR"

# Integer-period sines so Bevy LOOP does not click (128 Hz * 8 s = 1024 cycles).
ffmpeg -y -hide_banner -loglevel error \
  -f lavfi -i "sine=frequency=128:duration=8,volume=0.18" \
  -f lavfi -i "sine=frequency=192:duration=8,volume=0.11" \
  -filter_complex "[0][1]amix=inputs=2:duration=longest,alimiter=limit=0.12,volume=0.45,highpass=f=80,lowpass=f=1200" \
  -c:a libvorbis -q:a 2 "$DIR/peace_yard_bed.ogg"
echo "wrote $DIR/peace_yard_bed.ogg"

# Soft short triad — quieter than the harvest sting.
ffmpeg -y -hide_banner -loglevel error \
  -f lavfi -i "sine=frequency=392.00:duration=0.16,afade=t=in:st=0:d=0.02,afade=t=out:st=0.10:d=0.06" \
  -f lavfi -i "sine=frequency=523.25:duration=0.20,afade=t=in:st=0:d=0.02,afade=t=out:st=0.12:d=0.08" \
  -f lavfi -i "sine=frequency=659.25:duration=0.28,afade=t=in:st=0:d=0.03,afade=t=out:st=0.16:d=0.12" \
  -filter_complex "[0][1]amix=inputs=2:duration=longest:dropout_transition=0.04,volume=0.40[a];[a][2]amix=inputs=2:duration=longest:weights=0.7 1.0:dropout_transition=0.06,volume=0.45,highpass=f=180,lowpass=f=3600" \
  -c:a libvorbis -q:a 3 "$DIR/peace_well_sting.ogg"
echo "wrote $DIR/peace_well_sting.ogg"
