#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TURBO_JSONC="${ROOT_DIR}/turbo.jsonc"

ensure_task_present() {
  local pattern="$1"
  local description="$2"

  if ! grep -q "${pattern}" "${TURBO_JSONC}"; then
    printf 'missing %s in turbo.jsonc\n' "${description}" >&2
    return 1
  fi
}

main() {
  ensure_task_present '"//#build:rust:fw-esp32c3"' 'root build task for fw-esp32c3'
  ensure_task_present '"//#test:rust:fw-esp32c3"' 'root test task for fw-esp32c3'
}

main "$@"

