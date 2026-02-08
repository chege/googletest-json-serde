#!/usr/bin/env bash
set -euo pipefail

if git rev-parse --verify --quiet '@{upstream}' >/dev/null; then
	range='@{upstream}..HEAD'
elif git show-ref --verify --quiet refs/remotes/origin/main; then
	range='origin/main..HEAD'
else
	range='HEAD~1..HEAD'
fi

commits=$(git rev-list --reverse "$range" 2>/dev/null || true)
if [[ -z "$commits" ]]; then
	exit 0
fi

for commit in $commits; do
	subject=$(git log -1 --pretty=%s "$commit")
	echo "commitlint: $commit $subject"
	git log -1 --pretty=%B "$commit" | npx --no -- commitlint
done
