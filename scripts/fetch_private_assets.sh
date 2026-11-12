#!/usr/bin/env bash
# ASSET-FETCH-1 / STONE-GROUND-1: opt-in fetch into client/assets/private/.
# Manual step only. Build, CI, and the play-offline doors never run it.
# LFS pull is only the include list. The client reads stone-painterly-08 when present.
#
# Token: POWRUSH_ASSETS_TOKEN, or `gh auth token` as a fallback.
# With no token it prints one line and exits 0 without changing anything.
# The token is never echoed, logged, put in a URL, or written to any file.
set -euo pipefail

PIN="c00c4fb8383804db7955be900aa3e37914ff25c5"
REPO_URL="https://github.com/Eternally-Thriving-Grandmasterism/Powrush-Assets-Private.git"
# Later asset cards extend this one include list.
LFS_INCLUDE="textures/stone-painterly-08/**"

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DEST="${ROOT}/client/assets/private"

TOKEN="${POWRUSH_ASSETS_TOKEN:-}"
if [ -z "${TOKEN}" ] && command -v gh >/dev/null 2>&1; then
  TOKEN="$(gh auth token 2>/dev/null || true)"
fi
if [ -z "${TOKEN}" ]; then
  echo "powrush: no POWRUSH_ASSETS_TOKEN or gh auth token; skipping private asset fetch."
  exit 0
fi

# Hand the token to git through an askpass helper that reads it from the
# environment, so it never lands in argv, a URL or .git/config.
ASKPASS="$(mktemp)"
trap 'rm -f "${ASKPASS}"' EXIT
cat > "${ASKPASS}" <<'ASK'
#!/usr/bin/env bash
case "$1" in
  Username*) echo "x-access-token" ;;
  *) echo "${POWRUSH_FETCH_SECRET}" ;;
esac
ASK
chmod 700 "${ASKPASS}"

run_git() {
  POWRUSH_FETCH_SECRET="${TOKEN}" GIT_ASKPASS="${ASKPASS}" GIT_TERMINAL_PROMPT=0 \
    git -c credential.helper= "$@"
}

if [ ! -d "${DEST}/.git" ]; then
  mkdir -p "${DEST}"
  git -C "${DEST}" init -q
  git -C "${DEST}" remote add origin "${REPO_URL}"
fi

if ! run_git -C "${DEST}" fetch -q --depth 1 origin "${PIN}"; then
  echo "powrush: ERROR could not fetch pinned commit ${PIN} from Powrush-Assets-Private." >&2
  exit 1
fi
GIT_LFS_SKIP_SMUDGE=1 git -C "${DEST}" checkout -q --detach FETCH_HEAD

HEAD_SHA="$(git -C "${DEST}" rev-parse HEAD)"
if [ "${HEAD_SHA}" != "${PIN}" ]; then
  echo "powrush: ERROR checked out ${HEAD_SHA}, expected pin ${PIN}." >&2
  exit 1
fi
if ! command -v git-lfs >/dev/null 2>&1; then
  echo "powrush: WARNING git-lfs is missing; PNGs stay pointers and the Sanctuary yard keeps its placeholder."
  exit 0
fi
if ! run_git -C "${DEST}" lfs pull --include "${LFS_INCLUDE}"; then
  echo "powrush: ERROR git lfs pull failed for ${LFS_INCLUDE}." >&2
  exit 1
fi
if [ ! -d "${DEST}/textures/stone-painterly-08" ]; then
  echo "powrush: ERROR textures/stone-painterly-08 is missing after git lfs pull." >&2
  exit 1
fi
sig="version https://git-lfs.github.com/spec/v1"
while IFS= read -r -d '' f; do
  if [ "$(head -c "${#sig}" "$f" 2>/dev/null || true)" = "${sig}" ]; then
    echo "powrush: ERROR an LFS pointer remains under textures/stone-painterly-08." >&2
    exit 1
  fi
done < <(find "${DEST}/textures/stone-painterly-08" -type f -print0)
echo "powrush: private assets at ${PIN} in client/assets/private/."
