#!/usr/bin/env bash
# Licensed-asset guard: every model/texture under assets/ needs a filled row
# in assets/asset-licenses.csv (path,source,license,author,url).
# Models and textures pass only as CC0 or self-made AG-SML-1.1 (VIS-MODELS-GATE-1).
# Usage: check_asset_licenses.sh [ROOT]
#        check_asset_licenses.sh --private [ROOT]   (opt-in: client/assets/private/LICENCES.md)
set -u
MODE=public
if [ "${1:-}" = "--private" ]; then MODE=private; shift; fi
ROOT="${1:-.}"
MODEL_EXT='glb|gltf|bin|fbx|dae|png|jpg|jpeg|ktx2|webp|basis'

licence_ok() {
  case "$1" in CC0|CC0-1.0|"CC0 1.0"|AG-SML-1.1) return 0 ;; *) return 1 ;; esac
}

if [ "$MODE" = "private" ]; then
  PDIR="$ROOT/client/assets/private"
  if [ ! -d "$PDIR" ]; then
    echo "asset-licenses: no client/assets/private/; private check skipped."; exit 0
  fi
  MAN="$PDIR/LICENCES.md"
  if [ ! -f "$MAN" ]; then
    echo "asset-licenses: missing $MAN"; exit 1
  fi
  (cd "$PDIR" && find . -path ./.git -prune -o -type f -print) | sed 's|^\./||' \
    | awk -v man="$MAN" -v exts="$MODEL_EXT" '
    function trim(s) { gsub(/^[ \t`]+|[ \t`\r]+$/, "", s); return s }
    function lok(l) { return l == "CC0" || l == "CC0-1.0" || l == "CC0 1.0" || l == "AG-SML-1.1" }
    BEGIN {
      n = 0; hdr = 0
      while ((getline line < man) > 0) {
        if (line !~ /^\|/) continue
        k = split(line, c, "|")
        if (!hdr) {
          for (i = 2; i < k; i++) {
            h = tolower(trim(c[i]))
            if (h == "path") cp = i; else if (h == "licence") cl = i
            else if (h == "cost") cc = i; else if (h == "notes") cn = i
          }
          if (!cp || !cl || !cc) { print "asset-licenses: bad header in " man; bad = 1; exit }
          hdr = 1; continue
        }
        if (line ~ /^\|[ \t]*-+/) continue
        n++; P[n] = trim(c[cp]); L[n] = trim(c[cl]); C[n] = trim(c[cc]); N[n] = cn ? trim(c[cn]) : ""
      }
      if (!hdr) { print "asset-licenses: no table in " man; bad = 1; exit }
    }
    {
      f = $0; e = tolower(f); sub(/.*\./, "", e)
      if (e !~ ("^(" exts ")$")) next
      best = 0
      for (i = 1; i <= n; i++)
        if ((f == P[i] || index(f, P[i] "/") == 1) && (!best || length(P[i]) > length(P[best]))) best = i
      if (!best) { print "asset-licenses: no LICENCES.md row for " f; bad = 1; next }
      if (seen[best]++) next
      if (C[best] != "$0") { print "asset-licenses: pack " P[best] " cost is not $0"; bad = 1 }
      if (!lok(L[best])) { print "asset-licenses: pack " P[best] " licence " L[best] " is not CC0 or AG-SML-1.1"; bad = 1 }
      if (tolower(L[best] " " N[best]) ~ /pending/) { print "asset-licenses: pack " P[best] " is pending"; bad = 1 }
    }
    END { exit bad }'
  exit $?
fi

CSV="$ROOT/assets/asset-licenses.csv"
if [ ! -f "$CSV" ]; then
  echo "asset-licenses: missing $CSV"; exit 1
fi
if [ "$(head -n 1 "$CSV" | tr -d '\r')" != "path,source,license,author,url" ]; then
  echo "asset-licenses: bad header in $CSV"; exit 1
fi
if git -C "$ROOT" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  FILES="$(git -C "$ROOT" ls-files assets)"
else
  FILES="$(cd "$ROOT" && find assets -type f)"
fi
fail=0
while IFS= read -r f; do
  [ -z "$f" ] && continue
  ext="${f##*.}"; ext="${ext,,}"
  case "$ext" in glb|gltf|bin|fbx|dae|png|jpg|jpeg|ktx2|webp|basis) ;; *) continue ;; esac
  row="$(tail -n +2 "$CSV" | tr -d '\r' | awk -F, -v p="$f" '$1==p {print; exit}')"
  if [ -z "$row" ]; then
    echo "asset-licenses: no row for $f"; fail=1; continue
  fi
  IFS=, read -r _ src lic auth url <<<"$row"
  for v in "$src" "$lic" "$auth" "$url"; do
    if [ -z "$v" ] || [[ "$v" == *TBD* ]] || [[ "$v" == *"["* ]]; then
      echo "asset-licenses: incomplete row for $f"; fail=1; break
    fi
  done
  if ! licence_ok "$lic"; then
    echo "asset-licenses: licence $lic not allowed for $f (CC0 or AG-SML-1.1 only)"; fail=1
  fi
done <<<"$FILES"
exit "$fail"
