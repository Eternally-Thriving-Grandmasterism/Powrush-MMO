#!/usr/bin/env bash
# PRIVATE-BUILD-1: local / private-CI offline packager (never run from public CI).
set -euo pipefail

PIN="6f620dba981b4898b0b71a89ae71201c8ddabd0d"
ROOT="${POWRUSH_PACKAGE_ROOT:-$(cd "$(dirname "$0")/.." && pwd)}"
PDIR="${ROOT}/client/assets/private"

allowed_private_path() {
  local rel="$1"
  [[ "${rel}" == LICENCES.md || "${rel}" == ASSETS-NOTICE.md ]] && return 0
  [[ "${rel}" =~ ^textures/stone-painterly-08(/|$) ]] && return 0
  return 1
}

assert_stone_only_private_tree() {
  local bad=0 f rel
  while IFS= read -r -d '' f; do
    rel="${f#${PDIR}/}"
    if ! allowed_private_path "${rel}"; then
      echo "[[private-packager]]: disallowed private path: ${rel}" >&2
      bad=1
    fi
  done < <(find "${PDIR}" \( -path "${PDIR}/.git" -o -path "${PDIR}/.git/*" \) -prune -o -type f -print0)
  [[ "${bad}" -eq 0 ]] || exit 1
}

assert_dist_private_allowlist() {
  local dist="$1" bad=0 f rel
  while IFS= read -r -d '' f; do
    rel="${f#${dist}/assets/private/}"
    if ! allowed_private_path "${rel}"; then
      echo "[[private-packager]]: disallowed staged private path: ${rel}" >&2
      bad=1
    fi
  done < <(find "${dist}/assets/private" -type f -print0)
  [[ "${bad}" -eq 0 ]] || exit 1
}

stage_private_into_dist() {
  local dist="$1"
  mkdir -p "${dist}/assets/private/textures"
  cp -a "${PDIR}/textures/stone-painterly-08" "${dist}/assets/private/textures/"
  [[ -f "${PDIR}/LICENCES.md" ]] && cp "${PDIR}/LICENCES.md" "${dist}/assets/private/"
  [[ -f "${PDIR}/ASSETS-NOTICE.md" ]] && cp "${PDIR}/ASSETS-NOTICE.md" "${dist}/assets/private/"
  assert_dist_private_allowlist "${dist}"
}

ensure_private_tree() {
  if [[ -d "${PDIR}/textures/stone-painterly-08" ]]; then return 0; fi
  if [[ -n "${POWRUSH_ASSETS_TOKEN:-}" ]]; then
    bash "${ROOT}/scripts/fetch_private_assets.sh"
  else
    echo "[[private-packager]]: no private tree at PIN; set POWRUSH_ASSETS_TOKEN to fetch" >&2
    exit 1
  fi
  [[ -d "${PDIR}/textures/stone-painterly-08" ]] || {
    echo "[[private-packager]]: textures/stone-painterly-08 missing after fetch" >&2
    exit 1
  }
}

verify_pin() {
  if [[ ! -f "${PDIR}/.git/HEAD" && ! -f "${PDIR}/.git" ]]; then
    return 0
  fi
  local head
  head="$(git -C "${PDIR}" rev-parse HEAD)"
  [[ "${head}" == "${PIN}" ]] || {
    echo "[[private-packager]]: private tree at ${head}, expected pin ${PIN}" >&2
    exit 1
  }
}

package_dist() {
  local os="$1" bin="$2" sha8="$3" dist="$4" item base sha
  mkdir -p "${dist}/assets"
  if [[ "${os}" == "linux" ]]; then
    (cd "$(dirname "${bin}")" && tar czf "${dist}/powrush-offline-linux-${sha8}.tar.gz" "$(basename "${bin}")")
  else
    cp -f "${bin}" "${dist}/"
  fi
  cp -a "${ROOT}/assets/." "${dist}/assets/"
  for item in "${ROOT}/client/assets"/*; do
    base="$(basename "${item}")"
    [[ "${base}" == "private" ]] && continue
    cp -a "${item}" "${dist}/assets/"
  done
  stage_private_into_dist "${dist}"
  cp "${ROOT}/THIRD-PARTY-NOTICES.txt" "${dist}/"
  test -s "${dist}/THIRD-PARTY-NOTICES.txt" || { echo "[[private-packager]]: THIRD-PARTY-NOTICES missing" >&2; exit 1; }
  rm -f "${dist}/assets/music/epic_dark_ambient.ogg"
  rmdir "${dist}/assets/music" 2>/dev/null || true
  cat > "${dist}/README.txt" << 'EOF'
Offline build only.
Online is grey and disabled.
Early Access offline build. Single-player. Saves stay on your machine.
EOF
  sha="$(git -C "${ROOT}" rev-parse HEAD)"
  {
    echo "tag=none"; echo "sha=${sha}"; echo "os=${os}"
    (cd "${dist}" && find . -type f ! -name BUILD-ID.txt -print0 | sort -z | xargs -0 -r sha256sum)
  } > "${dist}/BUILD-ID.txt"
  [[ -z "$(find "${dist}" -iname '*.pdb' -print -quit)" ]] || {
    echo "[[private-packager]]: .pdb in dist" >&2; exit 1
  }
}

cd "${ROOT}"
ensure_private_tree
verify_pin
assert_stone_only_private_tree
bash "${ROOT}/scripts/check_asset_licenses.sh" --private "${ROOT}"

case "$(uname -s)" in
  Linux) os=linux ;;
  MINGW*|MSYS*|CYGWIN*) os=windows ;;
  *) echo "[[private-packager]]: unsupported OS" >&2; exit 1 ;;
esac

if [[ -n "${POWRUSH_PACKAGE_FIXTURE_BIN:-}" ]]; then
  bin="${POWRUSH_PACKAGE_FIXTURE_BIN}"
else
  bin="${ROOT}/target/release/powrush-client$([[ "${os}" == windows ]] && echo .exe)"
  cargo build --release -p powrush-client --locked
fi
[[ -f "${bin}" ]] || { echo "[[private-packager]]: release binary missing" >&2; exit 1; }

sha8="$(git -C "${ROOT}" rev-parse HEAD)"
sha8="${sha8:0:8}"
dist="${ROOT}/dist/powrush-offline-${os}-${sha8}-private"
rm -rf "${dist}"
package_dist "${os}" "${bin}" "${sha8}" "${dist}"
echo "[[private-packager]]: wrote ${dist}"
