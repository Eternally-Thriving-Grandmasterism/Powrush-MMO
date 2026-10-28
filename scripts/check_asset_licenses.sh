#!/usr/bin/env bash
# Licensed-asset guard: every model/texture under assets/ needs a filled row
# in assets/asset-licenses.csv (path,source,license,author,url).
# Usage: check_asset_licenses.sh [ROOT]
set -u
ROOT="${1:-.}"
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
  case "$ext" in glb|gltf|bin|png|jpg|jpeg|ktx2|webp|basis) ;; *) continue ;; esac
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
done <<<"$FILES"
exit "$fail"
