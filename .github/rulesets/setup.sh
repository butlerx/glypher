#!/usr/bin/env bash
# Adds admin bypass to the "main" repository ruleset so @butlerx/owners can push directly.
# Run: .github/rulesets/setup.sh

set -euo pipefail

RULESET_FILE="$(cd "$(dirname "$0")" && pwd)/admin-push-to-main.json"
REPO="butlerx/glypher"

if [ ! -f "$RULESET_FILE" ]; then
  echo "ERROR: Ruleset file not found: $RULESET_FILE" >&2
  exit 1
fi

# Find existing ruleset ID
RULESET_ID=$(gh api repos/$REPO/rulesets \
  -H "Accept: application/vnd.github+json" \
  -H "X-GitHub-Api-Version: 2022-11-28" \
  --jq '.rulesets[] | select(.name == "main") | .id')

if [ -z "$RULESET_ID" ]; then
  echo "ERROR: No ruleset named 'main' found." >&2
  exit 1
fi

echo "Updating ruleset ID $RULESET_ID..."
gh api repos/$REPO/rulesets/$RULESET_ID \
  -X PATCH \
  -H "Accept: application/vnd.github+json" \
  -H "X-GitHub-Api-Version: 2022-11-28" \
  --input "$RULESET_FILE" > /dev/null

echo "Done. @butlerx/owners can now push directly to main, bypassing PRs and checks."
