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
    verify_that!(j!([2, 3]), json::is_contained_in!(ge(2), ge(3), ge(4)))
}

#[test]
fn is_contained_in_wrong_type_gives_error_message() -> Result<()> {
    let matcher = json::is_contained_in![ge(1)];
    verify_that!(
        matcher.explain_match(&j!({"not": "an array"})),
        displays_as(contains_substring("which is not a JSON array"))
    )
}

#[test]
fn is_contained_in_matches_subset() -> Result<()> {
    verify_that!(j!([2, 3]), json::is_contained_in![ge(2), ge(3), ge(4)])
}

#[test]
fn is_contained_in_supports_trailing_comma() -> Result<()> {
    verify_that!(j!([2, 3]), json::is_contained_in![ge(2), ge(3),])
}

#[test]
fn is_contained_in_matches_when_actual_is_empty() -> Result<()> {
    // Empty actual is always a subset of expected.
    verify_that!(j!([]), json::is_contained_in![ge(2), ge(3), ge(4)])
}

#[test]
fn is_contained_in_does_not_match_when_an_element_is_unmatched() -> Result<()> {
    verify_that!(
        j!([1, 2, 3]),
        not(json::is_contained_in![ge(2), ge(3), ge(4)])
    )
}

#[test]
fn is_contained_in_explains_mismatch_due_to_wrong_size() -> Result<()> {
    let matcher = json::is_contained_in![ge(2), ge(3)];
    verify_that!(
        matcher.explain_match(&j!([2, 3, 4])),
        displays_as(eq("which has size 3 (expected at most 2)"))
    )
}

#[test]
fn is_contained_in_explains_missing_element_in_mismatch() -> Result<()> {
    let matcher = json::is_contained_in![ge(2), ge(3), ge(4)];
    verify_that!(
        matcher.explain_match(&j!([1, 2, 3])),
        displays_as(eq("whose element #0 does not match any expected elements"))
    )
}

#[test]
fn is_contained_in_explains_missing_elements_in_mismatch() -> Result<()> {
    let matcher = json::is_contained_in![ge(2), ge(3), ge(4), ge(5)];
    verify_that!(
        matcher.explain_match(&j!([0, 1, 2, 3])),
        displays_as(eq(
            "whose elements #0, #1 do not match any expected elements"
        ))
    )
}

#[test]
fn is_contained_in_explains_mismatch_due_to_no_graph_matching_found() -> Result<()> {
    let matcher = json::is_contained_in![json::primitive!(ge(1_i64)), json::primitive!(ge(3_i64))];
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

#[test]
fn is_contained_in_matches_nested_arrays() -> Result<()> {
    let data = j!([[1, 2], [3, 4]]);
    verify_that!(
        data,
        json::is_contained_in![
            json::is_contained_in![eq(1), eq(2)],
            json::is_contained_in![eq(3), eq(4)]
        ]
    )
}

#[test]
fn is_contained_in_handles_mixed_types() -> Result<()> {
    let data = j!([1, "a", true]);
    verify_that!(
        data,
        json::is_contained_in![eq(1), eq("a"), eq(true), eq(false)]
    )
}

#[test]
fn is_contained_in_with_duplicates_should_fail() -> Result<()> {
    verify_that!(j!([1, 1, 2]), not(json::is_contained_in![eq(1), eq(2)]))
}

#[test]
fn is_contained_in_with_empty_expected_should_fail() -> Result<()> {
    verify_that!(j!([1, 2, 3]), not(json::is_contained_in![]))
}

#[test]
fn is_contained_in_explains_nested_mismatch() -> Result<()> {
    let matcher = json::is_contained_in![
        json::is_contained_in![eq(1), eq(999)],
        json::is_contained_in![eq(3), eq(4)]
    ];
    verify_that!(
        matcher.explain_match(&j!([[1, 2], [3, 4]])),
        displays_as(contains_substring("element #0"))
    )
}

#[test]
fn is_contained_in_produces_correct_failure_message() -> Result<()> {
    let result = verify_that!(j!([1, 2, 3]), json::is_contained_in![ge(2), ge(3), ge(4)]);
    verify_that!(
        result,
        err(displays_as(starts_with(indoc!(
            r#"
                Value of: j!([1, 2, 3])
                Expected: contains JSON array elements matching in any order:
                  0. is greater than or equal to 2
                  1. is greater than or equal to 3
                  2. is greater than or equal to 4
                Actual: Array [Number(1), Number(2), Number(3)],
                  whose element #0 does not match any expected elements
            "#
        ))))
    )
}
