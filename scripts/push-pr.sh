#!/usr/bin/env bash

set -u
set -o pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${ROOT_DIR}"

LOG_ROOT="${ROOT_DIR}/.git_action_logs"
mkdir -p "${LOG_ROOT}"

info() {
  printf '[push-pr] %s\n' "$*"
}

warn() {
  printf '[push-pr][warn] %s\n' "$*" >&2
}

error() {
  printf '[push-pr][error] %s\n' "$*" >&2
}

require_command() {
  local cmd="$1"
  local install_hint="$2"
  if ! command -v "${cmd}" >/dev/null 2>&1; then
    error "Required command \"${cmd}\" is not available."
    warn "Install hint: ${install_hint}"
    warn "After installing \"${cmd}\", re-run scripts/push-pr.sh."
    exit 1
  fi
}

require_command git "https://git-scm.com/downloads"
require_command gh "brew install gh"
require_command cargo "rustup toolchain install stable"
require_command jq "brew install jq"

TURBO_CMD=()
if command -v turbo >/dev/null 2>&1; then
  TURBO_CMD=(turbo)
elif command -v pnpm >/dev/null 2>&1; then
  TURBO_CMD=(pnpm turbo)
else
  error '"turbo" CLI is required.'
  warn 'Either install Turbo globally (`npm install -g turbo`) or use pnpm (`pnpm add -g turbo`).'
  exit 1
fi

run_step() {
  local description="$1"
  shift
  local -a cmd=("$@")
  info "Running ${description}: ${cmd[*]}"
  if ! "${cmd[@]}"; then
    error "\"${description}\" failed."
    case "${description}" in
      "turbo validate")
        warn "Inspect the Turbo failure output above. Use \"${cmd[*]} --filter\" to narrow the scope if needed."
        ;;
      "cargo test")
        warn 'Run `cargo test -- --nocapture` locally to iterate on the failing tests.'
        ;;
      "git push")
        warn 'Resolve the git issue (e.g., rebase, credentials) and rerun this script.'
        ;;
      *)
        ;;
    esac
    exit 1
  fi
}

info "Repository root: ${ROOT_DIR}"

run_step "turbo validate" "${TURBO_CMD[@]}" validate
run_step "cargo test" cargo test

current_branch="$(git rev-parse --abbrev-ref HEAD)"
if [[ "${current_branch}" == "HEAD" ]]; then
  error "Detached HEAD detected. Checkout a branch before pushing."
  exit 1
fi

run_step "git push" git push origin "${current_branch}"

if ! gh auth status >/dev/null 2>&1; then
  error "GitHub CLI is not authenticated."
  warn 'Run `gh auth login` and re-run this script.'
  exit 1
fi

pr_url=""
if ! pr_url="$(gh pr view --json url --jq '.url' 2>/dev/null)"; then
  info "No open PR detected for ${current_branch}. Creating one."
  if ! gh pr create --fill --head "${current_branch}"; then
    error "Failed to create a pull request automatically."
    warn "Use \`gh pr create --fill --head ${current_branch}\` after resolving the issue."
    exit 1
  fi
  pr_url="$(gh pr view --json url --jq '.url')"
else
  info "Existing PR detected: ${pr_url}"
fi

commit_sha="$(git rev-parse HEAD)"
info "Waiting for GitHub Actions workflow for commit ${commit_sha}."

run_json="$(
  gh run list \
    --limit 20 \
    --json databaseId,headSha,status,conclusion,workflowName,displayTitle \
    2>/dev/null | jq --arg sha "${commit_sha}" 'map(select(.headSha == $sha)) | first' || true
)"
if [[ -z "${run_json}" || "${run_json}" == "null" ]]; then
  warn "No workflow run found for commit ${commit_sha}."
  warn "If workflows are expected, verify the GitHub Actions configuration or trigger the workflow manually."
  exit 0
fi

run_id="$(printf '%s' "${run_json}" | jq -r '.databaseId')"
workflow_name="$(printf '%s' "${run_json}" | jq -r '.workflowName // "workflow"')"
display_title="$(printf '%s' "${run_json}" | jq -r '.displayTitle // ""')"

if [[ -z "${run_id}" || "${run_id}" == "null" ]]; then
  warn "Unable to extract workflow run ID from GitHub CLI output."
  exit 0
fi

info "Monitoring workflow \"${workflow_name}\" (${display_title}) [run id: ${run_id}]."

if gh run watch "${run_id}" --exit-status; then
  info "Workflow completed successfully for commit ${commit_sha}."
  if [[ -n "${pr_url}" ]]; then
    info "PR ready: ${pr_url}"
  fi
  exit 0
fi

warn "Workflow failed for commit ${commit_sha}. Downloading logs."
log_dir="$(mktemp -d "${LOG_ROOT}/run_${commit_sha}_XXXXXX")"
if gh run download "${run_id}" --dir "${log_dir}"; then
  error "GitHub Actions run failed. Logs saved at: ${log_dir}"
  warn "Inspect the logs and iterate on the reported failures before re-running this script."
else
  warn "Failed to download logs automatically. Use \`gh run download ${run_id} --dir <path>\` to fetch them manually."
fi

exit 1

