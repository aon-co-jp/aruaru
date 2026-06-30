#!/usr/bin/env bash
set -euo pipefail

echo "aruaru basic check"

test -f README.md
test -d docs
find docs -name "*.md" -type f | sort

echo "OK"
