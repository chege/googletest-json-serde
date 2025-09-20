// tests/json_matchers_integration.rs

use googletest::prelude::*;
use googletest_json_serde::json;
// JSON matchers live under this module
use indoc::indoc;
use serde_json::json as j;

#[test]
fn is_contained_in_subset_matches() -> Result<()> {
    // Every actual element must be matched; extra matchers allowed.
    verify_that!(
        j!(["x", "y"]),
        json::is_contained_in![eq("y"), eq("x"), anything()]
    )
}

#[test]
fn is_contained_in_supports_trailing_comma() -> Result<()> {
    verify_that!(
        j!([1, 2]),
        json::is_contained_in![eq(1.0), eq(2.0), eq(3.0),]
    )
}

#[test]
fn is_contained_in_fails_when_container_too_large() -> Result<()> {
    verify_that!(
        &j!(["x", "y", "z"]),
        not(json::is_contained_in![eq("x"), eq("y")])
    )
}

#[test]
fn is_contained_in_reports_unmatched_element() -> Result<()> {
    verify_that!(&j!(["x", "y"]), not(json::is_contained_in![eq("y")]))
}

#[test]
fn is_contained_in_description_mismatch() -> Result<()> {
    let result = verify_that!(j!(["x", "y"]), json::is_contained_in![eq("y"), eq("z")]);
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            "
            Value of: j!([\"x\", \"y\"])
            Expected: contains each of the following elements (in any order):
              0. is equal to \"y\"
              1. is equal to \"z\"
            Actual: Array [String(\"x\"), String(\"y\")],
              which has no element matching the expected element #1
            "
        ))))
    )
}

#[test]
fn is_contained_in_matches_with_empty_array() -> Result<()> {
    verify_that!(j!([]), json::is_contained_in![])
}

#[test]
fn is_contained_in_matches_with_empty_array_and_trailing_comma() -> Result<()> {
    verify_that!(j!([]), json::is_contained_in![,])
}

#[test]
fn is_contained_in_matches_one_to_one_correspondence() -> Result<()> {
    verify_that!(
        j!(["x", "y", "z"]),
        json::is_contained_in![eq("x"), eq("y"), eq("z")]
    )
}

#[test]
fn is_contained_in_matches_with_excess_matchers() -> Result<()> {
    verify_that!(
        j!(["x", "y"]),
        json::is_contained_in![eq("y"), eq("x"), eq("z")]
    )
}

#[test]
fn is_contained_in_fails_when_unmatched_elements_present() -> Result<()> {
    verify_that!(
        j!(["a", "b"]),
        not(json::is_contained_in![eq("b"), eq("c"), eq("d")])
    )
}
