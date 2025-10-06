#!/usr/bin/env bash
set -euo pipefail

## test-downloaded-crate.sh -- Sanity check for published googletest-json-serde crate
# This script creates a temporary Rust project, adds the published crate as a dependency,
# runs a test to verify published crate functionality, and cleans up afterward.

# temp project
TMP_DIR=$(mktemp -d)
pushd "$TMP_DIR" >/dev/null

cargo new json_serde_check --lib
cd json_serde_check

# Add dependencies required for running the sanity test against the published crate.
cargo add googletest-json-serde googletest serde_json

# add test
mkdir -p tests
cat > tests/sanity_test.rs <<'EOF'
use googletest::prelude::*;
use googletest_json_serde::json;
use serde_json::json as j;

#[test]
fn sanity_check() {
    let value = j!({"name": "Alice", "age": 30});
    assert_that!(
        value,
        json::pat!({
            "name": starts_with("Ali"),
            "age": ge(29),
            ..
        })
    );
}
EOF

# run
cargo test --quiet
echo "✅ Sanity check passed."

# cleanup
popd
rm -rf "$TMP_DIR"