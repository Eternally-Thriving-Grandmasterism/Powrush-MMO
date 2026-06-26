#!/bin/bash
# scripts/verify_build.sh
# Powrush-MMO Build Verification Script
# Usage: ./scripts/verify_build.sh [--gpu] [--release]
#
# Runs cargo check across the workspace with proper feature flags.
# Part of RELEASE-CHECKLIST.md item 1.

set -euo pipefail

FEATURES=""
PROFILE="dev"

while [[ $# -gt 0 ]]; do
  case $1 in
    --gpu)
      FEATURES="gpu"
      shift
      ;;
    --release)
      PROFILE="release"
      shift
      ;;
    *)
      echo "Unknown option: $1"
      exit 1
      ;;
  esac
done

if [ -n "$FEATURES" ]; then
  echo "=== Verifying build with features: $FEATURES (profile: $PROFILE) ==="
  cargo check --workspace --features "$FEATURES" --profile "$PROFILE"
else
  echo "=== Verifying build (CPU fallback path, profile: $PROFILE) ==="
  cargo check --workspace --profile "$PROFILE"
fi

echo ""
echo "✅ Build verification complete. Thunder locked in. Yoi ⚡"
