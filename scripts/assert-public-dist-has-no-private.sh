#!/usr/bin/env bash
# PUBLIC leak guard: public offline artifacts must not contain private assets.
set -euo pipefail

DIST="${1:-}"
if [[ -z "${DIST}" || ! -d "${DIST}" ]]; then
  echo "[[public-dist-guard]]: dist directory required" >&2
  exit 1
fi

if [[ -e "${DIST}/assets/private" ]]; then
  echo "[[public-dist-guard]]: dist contains assets/private" >&2
  exit 1
fi

if find "${DIST}" -path '*stone-painterly-08*' -print -quit 2>/dev/null | grep -q .; then
  echo "[[public-dist-guard]]: dist contains stone-painterly-08 path" >&2
  exit 1
fi

exit 0
