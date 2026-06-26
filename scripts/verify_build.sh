#!/bin/bash
# scripts/verify_build.sh
# Powrush-MMO Build Verification Script
#
# Usage examples:
#   ./scripts/verify_build.sh                    # Basic check (no steam, dev profile)
#   ./scripts/verify_build.sh --steam          # Check with steam feature
#   ./scripts/verify_build.sh --no-steam       # Explicitly check without steam
#   ./scripts/verify_build.sh --gpu --release  # Full GPU + release profile
#   ./scripts/verify_build.sh --clippy         # Run clippy as well
#
# Part of RELEASE-CHECKLIST.md item 1.

set -euo pipefail

FEATURES=""
PROFILE="dev"
RUN_CLIPPY=false

print_usage() {
    echo "Usage: $0 [options]"
    echo ""
    echo "Options:"
    echo "  --steam          Enable steam feature"
    echo "  --no-steam       Explicitly disable steam feature (default)"
    echo "  --gpu            Enable gpu feature"
    echo "  --release        Use release profile"
    echo "  --clippy         Also run cargo clippy"
    echo "  -h, --help       Show this help message"
}

while [[ $# -gt 0 ]]; do
    case $1 in
        --steam)
            FEATURES="steam"
            shift
            ;;
        --no-steam)
            FEATURES=""
            shift
            ;;
        --gpu)
            if [ -n "$FEATURES" ]; then
                FEATURES+=",gpu"
            else
                FEATURES="gpu"
            fi
            shift
            ;;
        --release)
            PROFILE="release"
            shift
            ;;
        --clippy)
            RUN_CLIPPY=true
            shift
            ;;
        -h|--help)
            print_usage
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            print_usage
            exit 1
            ;;
    esac
done

# Build feature string
if [ -n "$FEATURES" ]; then
    FEATURE_ARG="--features $FEATURES"
    echo "=== Verifying build with features: $FEATURES (profile: $PROFILE) ==="
else
    FEATURE_ARG=""
    echo "=== Verifying build WITHOUT steam feature (profile: $PROFILE) ==="
fi

# Run cargo check
cargo check --workspace $FEATURE_ARG --profile "$PROFILE"

# Optionally run clippy
if [ "$RUN_CLIPPY" = true ]; then
    echo ""
    echo "=== Running cargo clippy ==="
    cargo clippy --workspace $FEATURE_ARG --profile "$PROFILE" -- -D warnings
fi

echo ""
echo "✅ Build verification complete. Thunder locked in. Yoi ⚡"
