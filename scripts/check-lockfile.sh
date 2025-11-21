#!/usr/bin/env bash

# Color codes
RED='\033[0;31m'
YELLOW='\033[1;33m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

# Run pnpm install to ensure lockfile is up to date
pnpm install

# Check if lockfile was modified
if [[ -n "$(git status --porcelain pnpm-lock.yaml)" ]]; then
  # If CI is set, always fail on lockfile changes
  if [[ -n "$CI" ]]; then
    echo -e "${RED}Error: pnpm-lock.yaml was modified. Please commit the updated lockfile.${NC}"
    exit 1
  fi

  # In local dev, allow lockfile changes if there are also changes to package.json files
  package_json_changes=$(git status --porcelain | grep 'package\.json' || true)
  if [[ -n "$package_json_changes" ]]; then
    echo
    echo -e "${YELLOW}Warning: pnpm-lock.yaml was modified, but package.json files were also changed.${NC}"
    echo
    echo "This is allowed in local development. Please commit both the package.json and pnpm-lock.yaml changes together."
    echo
    exit 0
  fi

  # Lockfile changed but no package.json changes - this shouldn't happen
  echo
  echo -e "${RED}Error: pnpm-lock.yaml was modified without corresponding package.json changes.${NC}"
  echo "This may indicate a problem. Please review the changes."
  echo
  exit 1
fi

exit 0

