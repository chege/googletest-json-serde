#!/usr/bin/env bash
set -euo pipefail

printf '%s\n' "$NEEDS_JSON" | jq 'map_values(.result)'
printf '%s\n' "$NEEDS_JSON" | jq -e 'type == "object" and length > 0 and all(.[]; .result == "success")' > /dev/null
