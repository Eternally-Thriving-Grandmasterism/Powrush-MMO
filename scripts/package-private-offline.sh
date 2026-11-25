#!/usr/bin/env bash
# PRIVATE-BUILD-1: local / private-CI offline packager (never run from public CI).
set -euo pipefail

PIN="bee7281b75fd834a72e717c1fe32ff64474086fd"
ROOT="${POWRUSH_PACKAGE_ROOT:-$(cd "$(dirname "$0")/.." && pwd)}"
PDIR="${ROOT}/client/assets/private"

allowed_private_path() {
  local rel="$1"
  [[ "${rel}" == LICENCES.md || "${rel}" == ASSETS-NOTICE.md ]] && return 0
  [[ "${rel}" =~ ^textures/stone-painterly-08(/|$) ]] && return 0
  [[ "${rel}" =~ ^animations/quaternius-ual-1/(LICENSE\.txt|License\.txt|Unreal-Godot/UAL1_Standard\.glb)$ ]] && return 0
  [[ "${rel}" =~ ^audio/music/(after-the-war\.ogg|WBA-NOTICE\.md)$ ]] && return 0
  return 1
}

require_private_pack_inputs() {
  [[ -f "${PDIR}/LICENCES.md" ]] || {
    echo "[[private-packager]]: missing private LICENCES.md" >&2
    exit 1
  }
  [[ -f "${PDIR}/textures/stone-painterly-08/ASSETS-NOTICE.md" ]] || {
    echo "[[private-packager]]: missing textures/stone-painterly-08/ASSETS-NOTICE.md" >&2
    exit 1
  }
  [[ -d "${PDIR}/textures/stone-painterly-08" ]] || {
    echo "[[private-packager]]: textures/stone-painterly-08 missing" >&2
    exit 1
  }
  if ! find "${PDIR}/textures/stone-painterly-08" -type f -print -quit | grep -q .; then
    echo "[[private-packager]]: no files under textures/stone-painterly-08" >&2
    exit 1
  fi
  [[ -f "${PDIR}/animations/quaternius-ual-1/LICENSE.txt" ]] || { echo "[[private-packager]]: missing animations/quaternius-ual-1/LICENSE.txt" >&2; exit 1; }
  [[ -f "${PDIR}/animations/quaternius-ual-1/License.txt" ]] || { echo "[[private-packager]]: missing animations/quaternius-ual-1/License.txt" >&2; exit 1; }
  [[ -f "${PDIR}/animations/quaternius-ual-1/Unreal-Godot/UAL1_Standard.glb" ]] || { echo "[[private-packager]]: missing animations/quaternius-ual-1/Unreal-Godot/UAL1_Standard.glb" >&2; exit 1; }
  [[ -f "${PDIR}/audio/music/after-the-war.ogg" ]] || { echo "[[private-packager]]: missing audio/music/after-the-war.ogg" >&2; exit 1; }
  [[ "$(head -c 4 "${PDIR}/audio/music/after-the-war.ogg")" == OggS ]] || { echo "[[private-packager]]: audio/music/after-the-war.ogg is not Ogg (LFS pointer?)" >&2; exit 1; }
  [[ -f "${PDIR}/audio/music/WBA-NOTICE.md" ]] || { echo "[[private-packager]]: missing audio/music/WBA-NOTICE.md" >&2; exit 1; }
}

log_other_private_packs() {
  local f rel
  while IFS= read -r -d '' f; do
    rel="${f#${PDIR}/}"
    if ! allowed_private_path "${rel}"; then
      echo "[[private-packager]]: other private packs present; only stone-08 and UAL1 will be copied" >&2
      return
    fi
  done < <(find "${PDIR}" \( -path "${PDIR}/.git" -o -path "${PDIR}/.git/*" \) -prune -o -type f -print0)
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
  mkdir -p "${dist}/assets/private/animations/quaternius-ual-1/Unreal-Godot"
  cp "${PDIR}/animations/quaternius-ual-1/LICENSE.txt" "${dist}/assets/private/animations/quaternius-ual-1/"
  cp "${PDIR}/animations/quaternius-ual-1/License.txt" "${dist}/assets/private/animations/quaternius-ual-1/"
  cp "${PDIR}/animations/quaternius-ual-1/Unreal-Godot/UAL1_Standard.glb" "${dist}/assets/private/animations/quaternius-ual-1/Unreal-Godot/"
  mkdir -p "${dist}/assets/private/audio/music"
  cp "${PDIR}/audio/music/after-the-war.ogg" "${dist}/assets/private/audio/music/"
  cp "${PDIR}/audio/music/WBA-NOTICE.md" "${dist}/assets/private/audio/music/"
  [[ -f "${PDIR}/LICENCES.md" ]] && cp "${PDIR}/LICENCES.md" "${dist}/assets/private/"
  [[ -f "${PDIR}/ASSETS-NOTICE.md" ]] && cp "${PDIR}/ASSETS-NOTICE.md" "${dist}/assets/private/"
  assert_dist_private_allowlist "${dist}"
  [[ -f "${dist}/assets/private/animations/quaternius-ual-1/Unreal-Godot/UAL1_Standard.glb" ]] || { echo "[[private-packager]]: animations/quaternius-ual-1/Unreal-Godot/UAL1_Standard.glb not staged" >&2; exit 1; }
  [[ -f "${dist}/assets/private/audio/music/after-the-war.ogg" ]] || { echo "[[private-packager]]: audio/music/after-the-war.ogg not staged" >&2; exit 1; }
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
Private build, do not share.
Offline build only.
Online is grey and disabled.
Early Access offline build. Single-player. Saves stay on your machine.
EOF
  if [[ "${os}" == windows ]]; then echo "Windows: run powrush-client.exe (next to assets/)." >> "${dist}/README.txt"; fi
  printf '\nCredits (from assets/private/LICENCES.md):\n' >> "${dist}/README.txt"
  # One credit line per manifest row: path, author, licence, source.
  awk -F'|' '/^\| `/ {
    for (i = 2; i <= 6; i++) { gsub(/^[ \t]+|[ \t]+$/, "", $i) }
    gsub(/`/, "", $2)
    print "- " $2 ": " $4 ", " $5 " (" $3 ")"
  }' "${PDIR}/LICENCES.md" >> "${dist}/README.txt"
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
require_private_pack_inputs
log_other_private_packs
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
