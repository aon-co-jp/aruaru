#!/usr/bin/env bash
set -euo pipefail

echo "[check] repository basic check"

test -f README.md
test -d docs
test -d aruaru-core
test -d aruaru-ai-core
test -d aruaru-ai

echo "[ok] basic files exist"
