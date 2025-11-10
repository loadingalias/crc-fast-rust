#!/usr/bin/env bash
#
# No-std Build Verification Script
#
# This script verifies that the library correctly builds in various no_std configurations.
# Expected behaviors:
#   - Library builds successfully with different feature combinations
#   - WASM target builds correctly
#   - Appropriate errors when missing allocator/panic handler (expected for lib-only builds)

set -e

echo "================================"
echo "No-std Build Verification"
echo "================================"
echo ""

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Test counter
TESTS_PASSED=0
TESTS_TOTAL=0

# Function to run a test
run_test() {
    local name="$1"
    local command="$2"
    local should_pass="$3"  # "pass" or "fail"

    TESTS_TOTAL=$((TESTS_TOTAL + 1))
    echo "[$TESTS_TOTAL] Testing: $name"
    echo "    Command: $command"

    if eval "$command" > /dev/null 2>&1; then
        if [ "$should_pass" = "pass" ]; then
            echo -e "    ${GREEN}✓ PASS${NC} (built successfully)"
            TESTS_PASSED=$((TESTS_PASSED + 1))
        else
            echo -e "    ${RED}✗ FAIL${NC} (expected build to fail but it passed)"
        fi
    else
        if [ "$should_pass" = "fail" ]; then
            echo -e "    ${YELLOW}✓ EXPECTED FAILURE${NC} (correctly requires allocator/panic handler from app)"
            TESTS_PASSED=$((TESTS_PASSED + 1))
        else
            echo -e "    ${RED}✗ FAIL${NC} (build failed unexpectedly)"
            return 1
        fi
    fi
    echo ""
}

echo "=== Standard Builds (with std) ==="
echo ""

run_test \
    "Standard build (default features)" \
    "cargo build --lib" \
    "pass"

run_test \
    "Standard build (all features)" \
    "cargo build --lib --all-features" \
    "pass"

echo "=== No-std Builds (library only) ==="
echo ""

run_test \
    "No-std core only (no allocator needed for lib check)" \
    "cargo check --lib --no-default-features" \
    "pass"

run_test \
    "No-std with alloc feature (lib check)" \
    "cargo check --lib --no-default-features --features alloc" \
    "pass"

run_test \
    "No-std with cache feature (lib check)" \
    "cargo check --lib --no-default-features --features cache" \
    "pass"

echo "=== WASM Target Builds ==="
echo ""

# Check if wasm32 target is installed
if rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
    run_test \
        "WASM build (no features - lib check)" \
        "cargo check --target wasm32-unknown-unknown --lib --no-default-features" \
        "pass"

    run_test \
        "WASM build (with alloc - lib check)" \
        "cargo check --target wasm32-unknown-unknown --lib --no-default-features --features alloc" \
        "pass"

    run_test \
        "WASM build (with cache - lib check)" \
        "cargo check --target wasm32-unknown-unknown --lib --no-default-features --features cache" \
        "pass"
else
    echo -e "${YELLOW}Skipping WASM tests (target not installed)${NC}"
    echo "Install with: rustup target add wasm32-unknown-unknown"
    echo ""
fi

echo "=== Architecture-Specific Checks ==="
echo ""

# Check current architecture capabilities
ARCH=$(rustc -Vv | grep host | cut -d' ' -f2)
echo "Current architecture: $ARCH"
echo ""

echo "================================"
echo "Summary"
echo "================================"
echo "Tests passed: $TESTS_PASSED / $TESTS_TOTAL"

if [ $TESTS_PASSED -eq $TESTS_TOTAL ]; then
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed!${NC}"
    exit 1
fi
