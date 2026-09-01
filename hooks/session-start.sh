#!/usr/bin/env bash
set -euo pipefail

# Emits the writing style guide on stdout which in turn should be added to the context.
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
GUIDE="${SCRIPT_DIR}/technical-writing.md"

if [ -f "${GUIDE}" ]; then
  cat "${GUIDE}"
fi

exit 0
