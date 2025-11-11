#!/usr/bin/env bash

# shellcheck shell=bash

await_build_then_checks() {
  local commit_sha="$1"
  local pr_number="${2:-}"
  local max_wait_seconds="${3:-30}"

  AWAIT_BUILD_FOUND_RUN="false"
  AWAIT_BUILD_RUN_ID=""
  AWAIT_BUILD_WORKFLOW_NAME=""
  AWAIT_BUILD_DISPLAY_TITLE=""
  AWAIT_BUILD_CHECKS_RAN="false"

  if [[ -z "${commit_sha}" ]]; then
    echo "await_build_then_checks: missing commit SHA" >&2
    return 1
  fi

  local spinner_frames=('|' '/' '-' '\\')
  local elapsed_seconds=0
  local run_id=""
  local workflow_name=""
  local display_title=""

  local filter
  printf -v filter 'map(select(.headSha == "%s")) | first | (if . == null then null else [.databaseId, (.workflowName // "workflow"), (.displayTitle // "")] | @tsv end)' "${commit_sha}"

  while (( elapsed_seconds < max_wait_seconds )); do
    local run_fields
    run_fields="$(gh run list \
      --limit 20 \
      --json databaseId,workflowName,displayTitle,headSha \
      --jq "${filter}" 2>/dev/null || true)"

    if [[ -n "${run_fields}" && "${run_fields}" != "null" ]]; then
      IFS=$'\t' read -r run_id workflow_name display_title <<<"${run_fields}"
      break
    fi

    if [[ -n "${spinner_frames[*]}" ]]; then
      local spinner_frame="${spinner_frames[elapsed_seconds % ${#spinner_frames[@]}]}"
      printf '\r[push-pr] Waiting for workflow run (%s) %s' "${commit_sha}" "${spinner_frame}"
    fi

    sleep 1
    ((elapsed_seconds++))
  done

  if [[ -n "$(type -t clear_spinner_line 2>/dev/null)" ]]; then
    clear_spinner_line
  fi

  if [[ -z "${run_id}" ]]; then
    if [[ -n "$(type -t warn 2>/dev/null)" ]]; then
      warn "No workflow run found for commit ${commit_sha}."
      warn "If workflows are expected, verify the GitHub Actions configuration or trigger the workflow manually."
    else
      printf '[push-pr][warn] No workflow run found for commit %s\n' "${commit_sha}" >&2
    fi
    return 10
  fi

  AWAIT_BUILD_FOUND_RUN="true"
  AWAIT_BUILD_RUN_ID="${run_id}"
  AWAIT_BUILD_WORKFLOW_NAME="${workflow_name}"
  AWAIT_BUILD_DISPLAY_TITLE="${display_title}"

  if [[ -n "$(type -t info 2>/dev/null)" ]]; then
    info "Monitoring workflow \"${workflow_name}\" (${display_title}) [run id: ${run_id}]."
  else
    printf '[push-pr] Monitoring workflow "%s" (%s) [run id: %s].\n' "${workflow_name}" "${display_title}" "${run_id}"
  fi

  if ! gh run watch "${run_id}" --exit-status; then
    return 11
  fi

  if [[ -n "${pr_number}" ]]; then
    if [[ -n "$(type -t info 2>/dev/null)" ]]; then
      info "Watching checks for PR #${pr_number}."
    else
      printf '[push-pr] Watching checks for PR #%s.\n' "${pr_number}"
    fi

    AWAIT_BUILD_CHECKS_RAN="true"

    if ! gh pr checks "${pr_number}" --watch --interval 10; then
      return 12
    fi
  fi

  return 0
}
