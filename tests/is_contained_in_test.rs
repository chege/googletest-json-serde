use googletest::prelude::*;
use googletest_json_serde::json;
use indoc::indoc;
use serde_json::json as j;

#[test]
fn is_contained_in_matches_empty_array() -> Result<()> {
    verify_that!(j!([]), json::is_contained_in![])
}

#[test]
fn is_contained_in_matches_empty_array_trailing_comma() -> Result<()> {
    verify_that!(j!([]), json::is_contained_in![,])
}

#[test]
fn is_contained_in_matches_with_parentheses() -> Result<()> {
    verify_that!(j!([2, 3]), json::is_contained_in!(eq(2), eq(3), eq(4)))
}

#[test]
fn is_contained_in_wrong_type_gives_error_message() -> Result<()> {
    let matcher = json::is_contained_in![eq(1)];
    verify_that!(
        matcher.explain_match(&j!({"not": "an array"})),
        displays_as(contains_substring("which is not a JSON array"))
    )
}

#[test]
fn is_contained_in_matches_subset() -> Result<()> {
    verify_that!(j!([2, 3]), json::is_contained_in![eq(2), eq(3), eq(4)])
}

#[test]
fn is_contained_in_supports_trailing_comma() -> Result<()> {
    verify_that!(j!([2, 3]), json::is_contained_in![eq(2), eq(3),])
}

#[test]
fn is_contained_in_matches_when_actual_is_empty() -> Result<()> {
    // Empty actual is always a subset of expected.
    verify_that!(j!([]), json::is_contained_in![eq(2), eq(3), eq(4)])
}

#[test]
fn is_contained_in_does_not_match_when_an_element_is_unmatched() -> Result<()> {
    verify_that!(
        j!([1, 2, 3]),
        not(json::is_contained_in![eq(2), eq(3), eq(4)])
    )
}

#[test]
fn is_contained_in_explains_mismatch_due_to_wrong_size() -> Result<()> {
    let matcher = json::is_contained_in![eq(2), eq(3)];
    verify_that!(
        matcher.explain_match(&j!([2, 3, 4])),
        displays_as(eq("which has size 3 (expected at most 2)"))
    )
}

#[test]
fn is_contained_in_explains_missing_element_in_mismatch() -> Result<()> {
    let matcher = json::is_contained_in![eq(2), eq(3), eq(4)];
    verify_that!(
        matcher.explain_match(&j!([1, 2, 3])),
        displays_as(eq("whose element #0 does not match any expected elements"))
    )
}

#[test]
fn is_contained_in_explains_missing_elements_in_mismatch() -> Result<()> {
    let matcher = json::is_contained_in![eq(2), eq(3), eq(4), eq(5)];
    verify_that!(
        matcher.explain_match(&j!([0, 1, 2, 3])),
        displays_as(eq(
            "whose elements #0, #1 do not match any expected elements"
        ))
    )
}

#[test]
fn is_contained_in_explains_mismatch_due_to_no_graph_matching_found() -> Result<()> {
    let matcher = json::is_contained_in![json::value!(ge(1_i64)), json::value!(ge(3_i64))];
    verify_that!(
        matcher.explain_match(&j!([1, 2])),
        displays_as(eq(indoc!(
            "
            which does not have a subset match with the expected elements. The best match found was:
              Actual element Number(1) at index 0 matched expected element `is greater than or equal to 1` at index 0.
              Actual element Number(2) at index 1 did not match any remaining expected element.
              Expected element `is greater than or equal to 3` at index 1 did not match any remaining actual element."))
    ))
}
