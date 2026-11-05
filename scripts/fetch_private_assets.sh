#!/usr/bin/env bash
# ASSET-FETCH-1: opt-in fetch of Powrush-Assets-Private into client/assets/private/.
# Manual step only. Nothing in the build, CI or the play-offline doors runs it,
# and nothing in the client reads client/assets/private/ yet.
#
# Token: POWRUSH_ASSETS_TOKEN, or `gh auth token` as a fallback.
# With no token it prints one line and exits 0 without changing anything.
# The token is never echoed, logged, put in a URL, or written to any file.
set -euo pipefail

PIN="3701c50523a661200f879c544ce459d99c59820d"
REPO_URL="https://github.com/Eternally-Thriving-Grandmasterism/Powrush-Assets-Private.git"

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
git -C "${DEST}" checkout -q --detach FETCH_HEAD

HEAD_SHA="$(git -C "${DEST}" rev-parse HEAD)"
if [ "${HEAD_SHA}" != "${PIN}" ]; then
  echo "powrush: ERROR checked out ${HEAD_SHA}, expected pin ${PIN}." >&2
  exit 1
fi
echo "powrush: private assets at ${PIN} in client/assets/private/."
