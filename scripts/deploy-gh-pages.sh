#!/usr/bin/env bash
# Publish the static site to GitHub Pages.
#
# Usage:
#   scripts/deploy-gh-pages.sh [--dry-run] [-y|--yes] [-h|--help]
#
# What it does:
#   1. Preflight — verifies git/npm, discovers the repo root + origin URL.
#   2. Build    — runs `GITHUB_PAGES=true npm run build` from frontend/.
#   3. Package  — adds 404.html (SPA fallback) and .nojekyll.
#   4. Push     — force-pushes frontend/dist/ to the `gh-pages` branch
#                 on origin, prompting first unless --yes / -y is given.
#
# Environment variables honored:
#   REPO     — override auto-detected origin URL.
#   BRANCH   — publish branch (default: gh-pages).
#
# The vite base path is set in frontend/vite.config.js via
# `process.env.GITHUB_PAGES === 'true'`; change it there if you rename
# the repository.

set -euo pipefail

# ---------- Pretty printing -----------------------------------------------

if [[ -t 1 ]] && command -v tput >/dev/null 2>&1 && [[ "$(tput colors 2>/dev/null || echo 0)" -ge 8 ]]; then
  C_BLUE=$(tput setaf 4); C_GREEN=$(tput setaf 2); C_YELLOW=$(tput setaf 3)
  C_RED=$(tput setaf 1);  C_BOLD=$(tput bold);    C_RESET=$(tput sgr0)
else
  C_BLUE=""; C_GREEN=""; C_YELLOW=""; C_RED=""; C_BOLD=""; C_RESET=""
fi

info() { printf '%s==>%s %s\n' "${C_BLUE}${C_BOLD}" "${C_RESET}" "$*"; }
ok()   { printf '%s ✓ %s%s\n' "${C_GREEN}${C_BOLD}" "${C_RESET}" "$*"; }
warn() { printf '%s !%s %s\n' "${C_YELLOW}${C_BOLD}" "${C_RESET}" "$*" >&2; }
die()  { printf '%s ✗%s %s\n' "${C_RED}${C_BOLD}"   "${C_RESET}" "$*" >&2; exit 1; }

usage() {
  sed -n '2,20p' "$0" | sed 's/^# \?//'
}

# ---------- Args ----------------------------------------------------------

DRY_RUN=0
ASSUME_YES=0
for arg in "$@"; do
  case "$arg" in
    --dry-run)        DRY_RUN=1 ;;
    -y|--yes)         ASSUME_YES=1 ;;
    -h|--help)        usage; exit 0 ;;
    *)                die "unknown flag: $arg (try --help)" ;;
  esac
done

# ---------- Locate repo root ---------------------------------------------

REPO_ROOT=$(git rev-parse --show-toplevel 2>/dev/null) \
  || die "not in a git repository"
cd "$REPO_ROOT"

FRONTEND_DIR="$REPO_ROOT/frontend"
DIST_DIR="$FRONTEND_DIR/dist"
PUBLISH_BRANCH="${BRANCH:-gh-pages}"

# ---------- Preflight -----------------------------------------------------

info "Preflight"

command -v npm >/dev/null || die "npm not found in PATH"
command -v git >/dev/null || die "git not found in PATH"
[[ -d "$FRONTEND_DIR" ]]               || die "missing directory: $FRONTEND_DIR"
[[ -f "$FRONTEND_DIR/package.json" ]]  || die "missing $FRONTEND_DIR/package.json"

# Discover origin URL unless overridden.
if [[ -n "${REPO:-}" ]]; then
  REMOTE_URL="$REPO"
else
  REMOTE_URL=$(git remote get-url origin 2>/dev/null) \
    || die "origin remote not configured (set REPO=<url> to override)"
fi

# Derive https://<user>.github.io/<repo>/ from any GitHub URL form.
if [[ "$REMOTE_URL" =~ github\.com[/:]([^/]+)/([^/.]+)(\.git)?$ ]]; then
  GH_USER="${BASH_REMATCH[1]}"
  GH_REPO="${BASH_REMATCH[2]}"
  LIVE_URL="https://${GH_USER}.github.io/${GH_REPO}/"
  SETTINGS_URL="https://github.com/${GH_USER}/${GH_REPO}/settings/pages"
else
  warn "origin is not a recognizable github.com URL — skipping URL derivation"
  GH_USER=""; GH_REPO=""; LIVE_URL=""; SETTINGS_URL=""
fi

# Warn if working tree isn't clean; we deploy whatever the committed
# source currently produces, but uncommitted changes should be noted.
if ! git diff --quiet --exit-code 2>/dev/null || ! git diff --cached --quiet --exit-code 2>/dev/null; then
  warn "working tree has uncommitted changes — the deploy uses what's in the files NOW, not HEAD"
fi

# Warn if current branch is ahead of its upstream.
CUR_BRANCH=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "")
if [[ -n "$CUR_BRANCH" ]] && git rev-parse --verify "origin/${CUR_BRANCH}" >/dev/null 2>&1; then
  AHEAD=$(git rev-list --count "origin/${CUR_BRANCH}..HEAD")
  if [[ "$AHEAD" -gt 0 ]]; then
    warn "${CUR_BRANCH} is ${AHEAD} commit(s) ahead of origin/${CUR_BRANCH} — push source first"
  fi
fi

ok "remote:    ${REMOTE_URL}"
ok "branch:    ${PUBLISH_BRANCH}"
[[ -n "$LIVE_URL" ]] && ok "live URL:  ${LIVE_URL}"

# ---------- Build ---------------------------------------------------------

info "Build"
cd "$FRONTEND_DIR"

if [[ ! -d node_modules ]]; then
  info "npm ci (first-time install)"
  npm ci
fi

GITHUB_PAGES=true npm run build

[[ -f "$DIST_DIR/index.html" ]] || die "build produced no $DIST_DIR/index.html"
ok "built $DIST_DIR"

# ---------- Package -------------------------------------------------------

info "Package"

# SPA fallback: GitHub Pages serves 404.html for unknown paths.
# Our app uses hash routing, but this still helps for deep-link refreshes.
cp "$DIST_DIR/index.html" "$DIST_DIR/404.html"

# Prevent Jekyll from mangling files prefixed with _ (asset hashes, etc.).
touch "$DIST_DIR/.nojekyll"

ok "added 404.html + .nojekyll"

# ---------- Confirm -------------------------------------------------------

if [[ "$DRY_RUN" -eq 1 ]]; then
  echo
  warn "dry run — stopping before push"
  echo "  dist/: ${DIST_DIR}"
  echo "  would push to: ${REMOTE_URL} (${PUBLISH_BRANCH})"
  exit 0
fi

if [[ "$ASSUME_YES" -eq 0 ]]; then
  echo
  warn "About to ${C_BOLD}force-push${C_RESET}${C_YELLOW} to ${C_BOLD}${PUBLISH_BRANCH}${C_RESET}${C_YELLOW} on ${REMOTE_URL}"
  printf "Continue? [y/N] "
  read -r REPLY </dev/tty || REPLY=""
  [[ "$REPLY" =~ ^[Yy]$ ]] || die "aborted"
fi

# ---------- Push ----------------------------------------------------------

info "Push"
cd "$DIST_DIR"

# Build a throwaway git repo inside dist/ so we can commit exactly the
# built artifacts (no history) and force-push them to gh-pages.
rm -rf .git
git init -q -b "$PUBLISH_BRANCH"
git add -A

COMMIT_MSG="Deploy $(date -u +%Y-%m-%dT%H:%M:%SZ)"
git \
  -c user.name='rust-patterns deploy' \
  -c user.email='deploy@noreply' \
  commit -q -m "$COMMIT_MSG"

git push -f -q "$REMOTE_URL" "$PUBLISH_BRANCH"

rm -rf "$DIST_DIR/.git"
ok "pushed $COMMIT_MSG"

# ---------- Post ----------------------------------------------------------

echo
ok "${C_BOLD}Deployed.${C_RESET}"

if [[ -n "$LIVE_URL" ]]; then
  cat <<EOF

  ${C_BOLD}Live:${C_RESET}     ${LIVE_URL}
  ${C_BOLD}Settings:${C_RESET} ${SETTINGS_URL}

EOF
  cat <<'EOF'
First deploy? Enable Pages on the repo:
  1. Go to the Settings URL above.
  2. Under "Source", pick "Deploy from a branch".
  3. Branch: gh-pages  /  (root).
  4. Click Save. Site goes live in ~30s.

EOF
fi
