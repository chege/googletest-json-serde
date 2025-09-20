// tests/json_matchers_integration.rs

use googletest::prelude::*;
use googletest_serde_json::json;
// JSON matchers live under this module
use indoc::indoc;
use serde_json::json as j;
// macro for building JSON values

// =============================
// elements_are! (ordered)
// =============================

#[test]
fn elements_are_matches_empty_array() -> Result<()> {
    verify_that!(j!([]), json::elements_are![])
}

#[test]
fn elements_are_matches_ordered_values() -> Result<()> {
    verify_that!(
        j!(["a", "b", "c"]),
        json::elements_are![eq("a"), eq("b"), eq("c")]
    )
}

#[test]
fn elements_are_supports_trailing_comma() -> Result<()> {
    verify_that!(
        j!([1, 2, 3]),
        json::elements_are![eq(1.0), eq(2.0), eq(3.0),]
    )
}

#[test]
fn elements_are_reports_length_mismatch() {
    let value = j!(["x", "y"]);
    let result = verify_that!(&value, json::elements_are![eq("x"), eq("y"), eq("z")]);

    assert_that!(
        result,
        err(displays_as(all![
            contains_substring("Value of: &value"),
            contains_substring("has JSON array elements"),
            contains_substring("size"),
        ]))
    );
}

#[test]
fn elements_are_reports_first_mismatch_explanation() {
    let value = j!(["x", "y", "z"]);
    let result = verify_that!(&value, json::elements_are![eq("x"), eq("WRONG"), eq("z")]);

    assert_that!(
        result,
        err(displays_as(all![
            contains_substring("Value of: &value"),
            contains_substring("element #1 is String(\"y\")"),
            contains_substring("isn't equal to \"WRONG\""),
        ]))
    );
}

// =============================
// unordered_elements_are!
// =============================

#[test]
fn unordered_elements_are_matches_unordered_values() -> Result<()> {
    verify_that!(
        j!(["a", "b", "c"]),
        json::unordered_elements_are![eq("c"), eq("a"), eq("b")]
    )
}

#[test]
fn unordered_elements_are_supports_trailing_comma() -> Result<()> {
    verify_that!(
        j!(["a", "b", "c"]),
        json::unordered_elements_are![eq("a"), eq("b"), eq("c"),]
    )
}

#[test]
fn unordered_elements_are_reports_unmatchable_expected() {
    let value = j!(["x", "y", "z"]);
    let result = verify_that!(
        &value,
        json::unordered_elements_are![eq("x"), eq("y"), eq("nope")]
    );

    assert_that!(
        result,
        err(displays_as(all![
            contains_substring("Value of: &value"),
            contains_substring("elements matching in any order"),
            contains_substring("did not match any remaining actual element"),
        ]))
    );
}

#[test]
fn unordered_elements_are_reports_unmatchable_actual() -> Result<()> {
    let value = j!(["x", "y"]);
    let result = verify_that!(
        &value,
        json::unordered_elements_are![eq("only-x"), eq("only-y")]
    );

    verify_that!(
        result,
        err(displays_as(eq(indoc!(
            "
            Value of: &value
            Expected: contains JSON array elements matching in any order:
              0. is equal to \"only-x\"
              1. is equal to \"only-y\"
            Actual: Array [String(\"x\"), String(\"y\")],
              which does not have a perfect match with the expected elements. The best match found was:
                Actual element String(\"x\") at index 0 did not match any remaining expected element.
                Actual element String(\"y\") at index 1 did not match any remaining expected element.
                Expected element `is equal to \"only-x\"` at index 0 did not match any remaining actual element.
                Expected element `is equal to \"only-y\"` at index 1 did not match any remaining actual element.
              at tests/unordered_elements_test.rs:106:18
            "
        ))))
    )
}

// =============================
// A couple of nested-array checks (ordered vs unordered)
// =============================

#[test]
fn nested_arrays_elements_are() -> Result<()> {
    verify_that!(
        j!([["x", "y"], ["z"]]),
        json::elements_are![
            json::elements_are![eq("x"), eq("y")],
            json::elements_are![eq("z")],
        ]
    )
}

#[test]
fn nested_arrays_unordered_elements_are() -> Result<()> {
    verify_that!(
        j!([["x", "y"], ["z"]]),
        json::unordered_elements_are![
            json::unordered_elements_are![eq("y"), eq("x")],
            json::unordered_elements_are![eq("z")],
        ]
    )
}

// =============================
// Snapshot of a typical mismatch message for documentation value
// =============================

#[test]
fn unordered_elements_are_integration_message_snapshot() {
    let value = j!(["x", "y"]);
    let result = verify_that!(&value, json::unordered_elements_are![eq("y"), eq("z")]);

    assert_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            "
            Value of: &value
            Expected: contains JSON array elements matching in any order:
            "
        ))))
    );
}
