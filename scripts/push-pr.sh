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

clear_spinner_line() {
  printf '\r%*s\r' "${COLUMNS:-80}" ''
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
require_command rustup "brew install rustup-init && rustup-init"
require_command jq "brew install jq"
require_command ldproxy "cargo install ldproxy --locked"

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

merge_pr=false

while [[ $# -gt 0 ]]; do
  case "$1" in
    --merge)
      merge_pr=true
      shift
      ;;
    --help|-h)
      cat <<'EOF'
Usage: scripts/push-pr.sh [OPTIONS]

Options:
  --merge   Merge the associated PR after checks pass.
  -h, --help
EOF
      exit 0
      ;;
    *)
      error "Unknown argument: $1"
      exit 1
      ;;
  esac
done

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
run_step "build:rust:fw-esp32c3" "${TURBO_CMD[@]}" build:rust:fw-esp32c3

current_branch="$(git rev-parse --abbrev-ref HEAD)"
if [[ "${current_branch}" == "HEAD" ]]; then
  error "Detached HEAD detected. Checkout a branch before pushing."
  exit 1
fi

info "Pushing branch ${current_branch} to origin."
push_output="$(git push origin "${current_branch}" 2>&1)"
if [[ $? -ne 0 ]]; then
  printf '%s\n' "${push_output}"
  error "\"git push\" failed."
  if grep -qi 'non-fast-forward' <<<"${push_output}"; then
    warn "Remote has commits that local history lacks. If you amended or rebased, run:"
    warn "  git push --force-with-lease origin ${current_branch}"
    warn "Otherwise, sync first via:"
    warn "  git pull --rebase origin ${current_branch}"
  elif grep -qi 'Authentication failed' <<<"${push_output}"; then
    warn "Authenticate with Git (e.g., \`gh auth refresh -h github.com -s repo\` or reconfigure your SSH keys) and rerun."
  else
    warn "Review the git error above, resolve it, then rerun this script."
  fi
  exit 1
fi
printf '%s\n' "${push_output}"

if ! gh auth status >/dev/null 2>&1; then
  error "GitHub CLI is not authenticated."
  warn 'Run `gh auth login` and re-run this script.'
  exit 1
fi

pr_info_json="$(gh pr view --json url,state,number 2>/dev/null || true)"
pr_url=""
pr_number=""

if [[ -z "${pr_info_json}" ]]; then
  info "No PR detected for ${current_branch}. Creating one."
elif [[ "${pr_info_json}" == "null" ]]; then
  info "No PR detected for ${current_branch}. Creating one."
else
  pr_state="$(printf '%s' "${pr_info_json}" | jq -r '.state')"
  pr_url="$(printf '%s' "${pr_info_json}" | jq -r '.url')"
  pr_number="$(printf '%s' "${pr_info_json}" | jq -r '.number')"

  if [[ "${pr_state}" == "OPEN" ]]; then
    info "Existing PR detected: ${pr_url}"
  else
    info "Current PR #${pr_number} is ${pr_state}. Creating a new PR."
    pr_url=""
  fi
fi

if [[ -z "${pr_url}" ]]; then
  if ! gh pr create --fill --head "${current_branch}"; then
    error "Failed to create a pull request automatically."
    warn "Use \`gh pr create --fill --head ${current_branch}\` after resolving the issue."
    exit 1
  fi
  pr_info_json="$(gh pr view --json url,state,number)"
  pr_url="$(printf '%s' "${pr_info_json}" | jq -r '.url')"
  pr_number="$(printf '%s' "${pr_info_json}" | jq -r '.number')"
  info "New PR created: ${pr_url}"
fi

commit_sha="$(git rev-parse HEAD)"
info "Waiting for GitHub Actions workflow for commit ${commit_sha}."

max_wait_seconds=30
spinner_frames=('|' '/' '-' '\')
elapsed_seconds=0
run_json=""

while (( elapsed_seconds < max_wait_seconds )); do
  run_json="$(
    gh run list \
      --limit 20 \
      --json databaseId,headSha,status,conclusion,workflowName,displayTitle \
      2>/dev/null | jq --arg sha "${commit_sha}" 'map(select(.headSha == $sha)) | first' || true
  )"

  if [[ -n "${run_json}" && "${run_json}" != "null" ]]; then
    break
  fi

  spinner_frame="${spinner_frames[elapsed_seconds % ${#spinner_frames[@]}]}"
  printf '\r[push-pr] Waiting for workflow run (%s) %s' "${commit_sha}" "${spinner_frame}"
  sleep 1
  ((elapsed_seconds++))
done

clear_spinner_line

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

  if [[ "${merge_pr}" == "true" ]]; then
    if [[ -z "${pr_number}" ]]; then
      warn "PR number unavailable; skipping merge."
    else
      merge_state="$(gh pr view "${pr_number}" --json state --jq '.state' 2>/dev/null || true)"
      if [[ "${merge_state}" != "OPEN" ]]; then
        warn "Cannot merge PR #${pr_number}; state is ${merge_state}."
      else
        info "Merging PR #${pr_number}."
        if gh pr merge "${pr_number}" --merge --auto; then
          info "PR #${pr_number} merged."
        else
          warn "Failed to merge PR #${pr_number}. Review GitHub CLI output."
        fi
      fi
    fi
  fi
  exit 0
fi

warn "Workflow failed for commit ${commit_sha}. Downloading logs."
log_dir="$(mktemp -d "${LOG_ROOT}/run_${commit_sha}_XXXXXX")"
log_file="${log_dir}/workflow.log"
if NO_COLOR=1 gh run view "${run_id}" --log >"${log_file}" 2>&1; then
  error "GitHub Actions run failed. Logs saved at: ${log_file}"
  warn "Inspect the logs and iterate on the reported failures before re-running this script."
  info "Extracting relevant log lines:"
  if command -v rg >/dev/null 2>&1; then
    rg --color=never --line-number --ignore-case --no-heading --regexp 'error' --regexp 'fail' --regexp 'panic' "${log_file}" || true
  else
    grep -n -i -E 'error|fail|panic' "${log_file}" || true
  fi
else
  warn "Failed to download logs automatically. Use \`NO_COLOR=1 gh run view ${run_id} --log > <path>\` to fetch them manually."
fi

exit 1

