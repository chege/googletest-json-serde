// tests/json_matchers_integration.rs

use googletest::prelude::*;
use serde_json::json as j;                  // macro for building JSON values
use googletest_serde_json::json;            // JSON matchers live under this module
use indoc::indoc;

// =============================
// elements_are! (ordered)
// =============================

#[test]
fn json_elements_are_matches_empty_array() -> Result<()> {
    verify_that!(j!([]), json::elements_are![])
}

#[test]
fn json_elements_are_matches_ordered_values() -> Result<()> {
    verify_that!(j!(["a", "b", "c"]), json::elements_are![eq("a"), eq("b"), eq("c")])
}

#[test]
fn json_elements_are_supports_trailing_comma() -> Result<()> {
    verify_that!(j!([1, 2, 3]), json::elements_are![eq(1.0), eq(2.0), eq(3.0),])
}

#[test]
fn json_elements_are_reports_length_mismatch() {
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
fn json_elements_are_reports_first_mismatch_explanation() {
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
fn json_unordered_elements_are_matches_unordered_values() -> Result<()> {
    verify_that!(
        j!(["a", "b", "c"]),
        json::unordered_elements_are![eq("c"), eq("a"), eq("b")]
    )
}

#[test]
fn json_unordered_elements_are_supports_trailing_comma() -> Result<()> {
    verify_that!(
        j!(["a", "b", "c"]),
        json::unordered_elements_are![eq("a"), eq("b"), eq("c"),]
    )
}

#[test]
fn json_unordered_elements_are_reports_unmatchable_expected() {
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
fn json_unordered_elements_are_reports_unmatchable_actual() {
    let value = j!(["x", "y"]);
    let result = verify_that!(
        &value,
        json::unordered_elements_are![eq("only-x"), eq("only-y")]
    );

    assert_that!(
        result,
        err(displays_as(all![
            contains_substring("Value of: &value"),
            contains_substring("does not match any expected elements"),
        ]))
    );
}

// =============================
// contains_each!
// =============================

#[test]
fn json_contains_each_subset_matches() -> Result<()> {
    verify_that!(j!(["a", "b", "c", "d"]), json::contains_each![eq("a"), eq("c")])
}

#[test]
fn json_contains_each_supports_trailing_comma() -> Result<()> {
    verify_that!(j!([1, 2, 3, 4]), json::contains_each![eq(2.0), eq(4.0),])
}

#[test]
fn json_contains_each_fails_when_container_too_small() {
    let value = j!(["x"]);
    let result = verify_that!(&value, json::contains_each![eq("x"), eq("y")]);

    assert_that!(
        result,
        err(displays_as(all![
            contains_substring("Value of: &value"),
            contains_substring("expected at least"),
        ]))
    );
}

#[test]
fn json_contains_each_explains_missing_expected_match() {
    let matcher = json::contains_each![eq("x"), eq("y"), eq("z")];
    assert_that!(
        matcher.explain_match(&j!(["x", "z"])),
        displays_as(contains_substring("has no element matching the expected element #1"))
    );
}

// =============================
// is_contained_in!
// =============================

#[test]
fn json_is_contained_in_subset_matches() -> Result<()> {
    // Every actual element must be matched; extra matchers allowed.
    verify_that!(j!(["x", "y"]), json::is_contained_in![eq("y"), eq("x"), anything()])
}

#[test]
fn json_is_contained_in_supports_trailing_comma() -> Result<()> {
    verify_that!(j!([1, 2]), json::is_contained_in![eq(1.0), eq(2.0), eq(3.0),])
}

#[test]
fn json_is_contained_in_fails_when_container_too_large() {
    let value = j!(["x", "y", "z"]);
    let result = verify_that!(&value, json::is_contained_in![eq("x"), eq("y")]);

    assert_that!(
        result,
        err(displays_as(all![
            contains_substring("Value of: &value"),
            contains_substring("expected at most"),
        ]))
    );
}

#[test]
fn json_is_contained_in_reports_unmatched_element() {
    let value = j!(["x", "y"]);
    let result = verify_that!(&value, json::is_contained_in![eq("y")]);

    assert_that!(
        result,
        err(displays_as(all![
            contains_substring("Value of: &value"),
            contains_substring("is contained in the following element set"),
            contains_substring("does not match any expected elements"),
        ]))
    );
}

// =============================
// A couple of nested-array checks (ordered vs unordered)
// =============================

#[test]
fn json_nested_arrays_elements_are() -> Result<()> {
    verify_that!(
        j!([["x", "y"], ["z"]]),
        json::elements_are![
            json::elements_are![eq("x"), eq("y")],
            json::elements_are![eq("z")],
        ]
    )
}

#[test]
fn json_nested_arrays_unordered_elements_are() -> Result<()> {
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
fn json_unordered_elements_are_integration_message_snapshot() {
    let value = j!(["x", "y"]);
    let result = verify_that!(
        &value,
        json::unordered_elements_are![eq("y"), eq("z")]
    );

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