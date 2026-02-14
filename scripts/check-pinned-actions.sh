#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
WORKFLOWS_DIR="${ROOT}/.github/workflows"

status=0

while IFS= read -r entry; do
  file="${entry%%:*}"
  line_no_and_text="${entry#*:}"
  line_no="${line_no_and_text%%:*}"
  line="${line_no_and_text#*:}"

  ref="$(printf '%s\n' "$line" | sed -E 's/.*uses:[[:space:]]*[^@[:space:]]+@([^[:space:]]+).*/\1/')"

  if [[ "$ref" =~ ^[0-9a-f]{40}$ ]]; then
    continue
  fi

  printf 'unpinned action reference: %s:%s -> %s\n' "$file" "$line_no" "$line" >&2
  status=1
done < <(rg -n '^[[:space:]]*uses:[[:space:]]*[^@[:space:]]+@[^[:space:]]+' \
  "${WORKFLOWS_DIR}" --glob '*.yml' --glob '*.yaml')

if [[ "$status" -ne 0 ]]; then
  echo "Found floating GitHub Action refs; pin all uses: entries to full commit SHAs." >&2
  exit 1
fi

echo "All GitHub Action refs are pinned to commit SHAs."
