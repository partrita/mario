#!/usr/bin/env bash
#
# sync-upstream.sh — Pull upstream rio changes into the mario branch while
# preserving mario-specific customizations (rename rio→mario, mustache logo,
# CI/linker tweaks, release structure).
#
# Strategy:
#   1. Fetch the latest from upstream (raphamorim/rio).
#   2. Merge upstream/main into a temporary sync branch started from mario.
#   3. Re-apply mario-specific commits on top so mario customizations always win.
#   4. Fast-forward mario to the result and push to origin.
#
# Usage:
#   ./misc/scripts/sync-upstream.sh          # sync and push
#   ./misc/scripts/sync-upstream.sh --no-push # sync locally only
#
# Requirements: git, and write access to origin.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

NO_PUSH=0
if [[ "${1:-}" == "--no-push" ]]; then
  NO_PUSH=1
fi

cd "$REPO_ROOT"

MARIO_BRANCH="feat/rename-rio-to-mario"
UPSTREAM_BRANCH="upstream/main"
SYNC_BRANCH="sync-upstream-$(date +%Y%m%d%H%M%S)"

echo "==> Current branch: $(git branch --show-current)"
echo "==> Mario branch:   $MARIO_BRANCH"
echo "==> Upstream:       $UPSTREAM_BRANCH"

# Sanity checks
if ! git rev-parse --verify "$UPSTREAM_BRANCH" >/dev/null 2>&1; then
  echo "ERROR: $UPSTREAM_BRANCH not found. Run: git fetch upstream" >&2
  exit 1
fi

if ! git rev-parse --verify "$MARIO_BRANCH" >/dev/null 2>&1; then
  echo "ERROR: $MARIO_BRANCH not found." >&2
  exit 1
fi

# Make sure mario is clean
if [[ -n "$(git status --porcelain)" ]]; then
  echo "ERROR: Working tree is dirty. Commit or stash changes first." >&2
  exit 1
fi

# Fetch latest upstream
echo "==> Fetching upstream..."
git fetch upstream

# Create sync branch from mario
echo "==> Creating sync branch $SYNC_BRANCH from $MARIO_BRANCH..."
git checkout "$MARIO_BRANCH"
git checkout -b "$SYNC_BRANCH"

# Merge upstream/main into the sync branch
echo "==> Merging $UPSTREAM_BRANCH into $SYNC_BRANCH..."
if ! git merge --no-ff -m "merge: sync upstream rio ($(date +%F))" "$UPSTREAM_BRANCH"; then
  echo "==> Merge conflicts detected. Resolve them, then re-run:"
  echo "      git add <resolved files>"
  echo "      git merge --continue"
  echo "      git checkout $MARIO_BRANCH"
  echo "      git merge $SYNC_BRANCH"
  exit 1
fi

# Re-apply mario-specific customizations on top so they always win.
# Mario commits are everything reachable from mario but not from upstream/main.
echo "==> Re-applying mario customizations on top of upstream sync..."
MARIO_COMMITS=$(git rev-list upstream/main.."$MARIO_BRANCH" --reverse)
if [[ -n "$MARIO_COMMITS" ]]; then
  # Cherry-pick each mario commit onto the sync branch.
  while IFS= read -r commit; do
    echo "    cherry-pick $commit"
    if ! git cherry-pick "$commit"; then
      echo "==> Conflict cherry-picking $commit. Resolve, then:"
      echo "      git add <resolved files>"
      echo "      git cherry-pick --continue"
      exit 1
    fi
  done <<< "$MARIO_COMMITS"
fi

# Fast-forward mario to the sync result
echo "==> Fast-forwarding $MARIO_BRANCH to synced state..."
git checkout "$MARIO_BRANCH"
git merge --ff-only "$SYNC_BRANCH"

# Cleanup sync branch
git branch -D "$SYNC_BRANCH"

if [[ "$NO_PUSH" -eq 0 ]]; then
  echo "==> Pushing $MARIO_BRANCH to origin..."
  git push origin "$MARIO_BRANCH"
fi

echo "==> Sync complete. $MARIO_BRANCH is now up to date with upstream + mario changes."
