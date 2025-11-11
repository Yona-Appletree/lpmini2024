#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "${ROOT_DIR}/lib/github_checks.sh"

main() {
  tmpdir="$(mktemp -d)"
  trap '[[ -n "${tmpdir:-}" ]] && rm -rf "${tmpdir}"' EXIT

  local bin_dir="${tmpdir}/bin"
  local state_dir="${tmpdir}/state"
  mkdir -p "${bin_dir}" "${state_dir}"

  export PATH="${bin_dir}:$PATH"
  export GH_STATE_DIR="${state_dir}"
  export GH_MOCK_LOG="${state_dir}/gh_calls.log"
  export GH_EXPECT_HEAD_SHA="abc123"
  export GH_RUN_LIST_ATTEMPTS_BEFORE_FOUND=2
  export GH_RUN_WATCH_EXIT=0
  export GH_PR_CHECKS_EXIT=0
  : >"${GH_MOCK_LOG}"

  cat <<'GH_STUB' >"${bin_dir}/gh"
#!/usr/bin/env bash
set -euo pipefail

log_call() {
  printf '%s\n' "$*" >>"${GH_MOCK_LOG}"
}

log_call "$@"

cmd="$1"
shift || true

case "${cmd}" in
  run)
    subcmd="$1"
    shift || true
    case "${subcmd}" in
      list)
        attempt_file="${GH_STATE_DIR}/run_list_attempt"
        attempt=0
        if [[ -f "${attempt_file}" ]]; then
          attempt=$(<"${attempt_file}")
        fi
        attempt=$((attempt + 1))
        printf '%s' "${attempt}" >"${attempt_file}"
        if (( attempt <= ${GH_RUN_LIST_ATTEMPTS_BEFORE_FOUND:-0} )); then
          printf 'null\n'
        else
          printf '321\tCI\tBuild\n'
        fi
        ;;
      watch)
        exit ${GH_RUN_WATCH_EXIT:-0}
        ;;
      *)
        echo "unexpected gh run subcommand: ${subcmd}" >&2
        exit 1
        ;;
    esac
    ;;
  pr)
    subcmd="$1"
    shift || true
    case "${subcmd}" in
      checks)
        attempt_file="${GH_STATE_DIR}/run_list_attempt"
        if [[ ! -f "${attempt_file}" ]]; then
          echo "pr checks called before run list" >&2
          exit 1
        fi
        exit ${GH_PR_CHECKS_EXIT:-0}
        ;;
      *)
        echo "unexpected gh pr subcommand: ${subcmd}" >&2
        exit 1
        ;;
    esac
    ;;
  *)
    echo "unexpected gh command: ${cmd}" >&2
    exit 1
    ;;
 esac
GH_STUB
  chmod +x "${bin_dir}/gh"

  cat <<'JQ_STUB' >"${bin_dir}/jq"
#!/usr/bin/env bash
set -euo pipefail
# pass-through stub; we should not invoke external jq in this test
cat
JQ_STUB
  chmod +x "${bin_dir}/jq"

  if ! await_build_then_checks "abc123" 42 5; then
    echo "expected await_build_then_checks to succeed" >&2
    exit 1
  fi

  if ! grep -q '^run list' "${GH_MOCK_LOG}"; then
    echo "expected gh run list to be invoked" >&2
    exit 1
  fi

  if ! grep -q '^pr checks' "${GH_MOCK_LOG}"; then
    echo "expected gh pr checks to be invoked" >&2
    exit 1
  fi

  local last_run_list_line
  local checks_line
  last_run_list_line=$(grep -n '^run list' "${GH_MOCK_LOG}" | tail -n1 | cut -d: -f1)
  checks_line=$(grep -n '^pr checks' "${GH_MOCK_LOG}" | tail -n1 | cut -d: -f1)

  if [[ -z "${last_run_list_line}" || -z "${checks_line}" ]]; then
    echo "missing log line numbers" >&2
    exit 1
  fi

  if (( checks_line <= last_run_list_line )); then
    echo "expected pr checks to be invoked after run list returned a build" >&2
    exit 1
  fi
}

main "$@"
