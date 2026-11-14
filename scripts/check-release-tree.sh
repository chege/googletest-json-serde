#!/usr/bin/env bash
set -euo pipefail

if ! git diff --quiet "$RELEASE_SHA" "$PR_HEAD"; then
  echo "::error::Merged tree differs from the release PR. Update and review the release PR before merging."
  exit 1
fi
if [[ "$RELEASE_SHA" != "$PR_HEAD" ]] && git merge-base --is-ancestor "$PR_HEAD" "$RELEASE_SHA"; then
  echo "::error::release-plz would publish a different commit."
  exit 1
fi
