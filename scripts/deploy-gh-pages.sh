#!/bin/bash
# Deploy the static site to GitHub Pages.
# Usage: ./scripts/deploy-gh-pages.sh
#
# Edit REPO below to point at the GitHub repository that will host the
# gh-pages branch. The vite base path is set via GITHUB_PAGES=true in
# frontend/vite.config.js — change the path there if you rename the repo.

set -e

REPO="https://github.com/rajeshpillai/rust-patterns.git"
DIST_DIR="frontend/dist"

echo "=== Building static site for GitHub Pages ==="
cd frontend
GITHUB_PAGES=true npm run build
cd ..

echo "=== Preparing deployment ==="
cd "$DIST_DIR"

# GitHub Pages SPA hack: copy index.html to 404.html so hash-less
# refreshes on sub-routes fall back to the SPA entry point.
cp index.html 404.html

# .nojekyll prevents GitHub from running Jekyll over the dist output
# (which would mangle files starting with an underscore).
touch .nojekyll

git init
git checkout -b gh-pages
git add -A
git commit -m "Deploy to GitHub Pages"

echo "=== Pushing to gh-pages branch ==="
git push -f "$REPO" gh-pages

cd ../..
rm -rf "$DIST_DIR/.git"

echo ""
echo "=== Deployed! ==="
echo "1. Go to ${REPO%.git}/settings/pages"
echo "2. Set Source: Deploy from a branch → gh-pages → / (root)"
echo "3. Site will be live at: https://rajeshpillai.github.io/rust-patterns/"
