use googletest::prelude::*;
use googletest_json_serde::json as j;
use indoc::indoc;
use serde_json::json;

#[test]
fn is_contained_in_matches_empty_array() -> Result<()> {
    verify_that!(json!([]), j::is_contained_in![])
}

#[test]
fn is_contained_in_matches_empty_array_trailing_comma() -> Result<()> {
    verify_that!(json!([]), j::is_contained_in![,])
}

#[test]
fn is_contained_in_matches_with_parentheses() -> Result<()> {
    verify_that!(json!([2, 3]), j::is_contained_in!(ge(2), ge(3), ge(4)))
}

#[test]
fn is_contained_in_wrong_type_gives_error_message() -> Result<()> {
    let matcher = j::is_contained_in![ge(1)];
    verify_that!(
        matcher.explain_match(&json!({"not": "an array"})),
        displays_as(contains_substring("which is not a JSON array"))
    )
}

#[test]
fn is_contained_in_matches_subset() -> Result<()> {
    verify_that!(json!([2, 3]), j::is_contained_in![ge(2), ge(3), ge(4)])
}

#[test]
fn is_contained_in_supports_trailing_comma() -> Result<()> {
    verify_that!(json!([2, 3]), j::is_contained_in![ge(2), ge(3),])
}

#[test]
fn is_contained_in_matches_when_actual_is_empty() -> Result<()> {
    // Empty actual is always a subset of expected.
    verify_that!(json!([]), j::is_contained_in![ge(2), ge(3), ge(4)])
}

#[test]
fn is_contained_in_does_not_match_when_an_element_is_unmatched() -> Result<()> {
    verify_that!(
        json!([1, 2, 3]),
        not(j::is_contained_in![ge(2), ge(3), ge(4)])
    )
}

#[test]
fn is_contained_in_explains_mismatch_due_to_wrong_size() -> Result<()> {
    let matcher = j::is_contained_in![ge(2), ge(3)];
    verify_that!(
        matcher.explain_match(&json!([2, 3, 4])),
        displays_as(eq("which has size 3 (expected at most 2)"))
    )
}

#[test]
fn is_contained_in_explains_missing_element_in_mismatch() -> Result<()> {
    let matcher = j::is_contained_in![ge(2), ge(3), ge(4)];
    verify_that!(
        matcher.explain_match(&json!([1, 2, 3])),
        displays_as(eq("whose element #0 does not match any expected elements"))
    )
}

#[test]
fn is_contained_in_explains_missing_elements_in_mismatch() -> Result<()> {
    let matcher = j::is_contained_in![ge(2), ge(3), ge(4), ge(5)];
    verify_that!(
        matcher.explain_match(&json!([0, 1, 2, 3])),
        displays_as(eq(
            "whose elements #0, #1 do not match any expected elements"
        ))
    )
}

#[test]
fn is_contained_in_explains_mismatch_due_to_no_graph_matching_found() -> Result<()> {
    let matcher = j::is_contained_in![j::primitive!(ge(1_i64)), j::primitive!(ge(3_i64))];
    verify_that!(
        matcher.explain_match(&json!([1, 2])),
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
    let data = json!([[1, 2], [3, 4]]);
    verify_that!(
        data,
        j::is_contained_in![
            j::is_contained_in![eq(1), eq(2)],
            j::is_contained_in![eq(3), eq(4)]
        ]
    )
}

#[test]
fn is_contained_in_handles_mixed_types() -> Result<()> {
    let data = json!([1, "a", true]);
    verify_that!(
        data,
        j::is_contained_in![eq(1), eq("a"), eq(true), eq(false)]
    )
}

#[test]
fn is_contained_in_with_duplicates_should_fail() -> Result<()> {
    verify_that!(json!([1, 1, 2]), not(j::is_contained_in![eq(1), eq(2)]))
}

#[test]
fn is_contained_in_with_empty_expected_should_fail() -> Result<()> {
    verify_that!(json!([1, 2, 3]), not(j::is_contained_in![]))
}

#[test]
fn is_contained_in_explains_nested_mismatch() -> Result<()> {
    let matcher = j::is_contained_in![
        j::is_contained_in![eq(1), eq(999)],
        j::is_contained_in![eq(3), eq(4)]
    ];
    verify_that!(
        matcher.explain_match(&json!([[1, 2], [3, 4]])),
        displays_as(contains_substring("element #0"))
    )
}

#[test]
fn is_contained_in_produces_correct_failure_message() -> Result<()> {
    let result = verify_that!(json!([1, 2, 3]), j::is_contained_in![ge(2), ge(3), ge(4)]);
    verify_that!(
        result,
        err(displays_as(starts_with(indoc!(
            r#"
                Value of: json!([1, 2, 3])
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

#[test]
fn is_contained_in_mixed_types_match_with_owned_values() -> Result<()> {
    let value = json!(["a", 1]);
    let a = json!("a");
    let one = json!(1);
    let t = json!(true);
    verify_that!(value, j::is_contained_in![a, one, t])
}

#[test]
fn is_contained_in_mixed_types_match_with_borrowed_values() -> Result<()> {
    let value = json!(["a", 1]);
    let a = json!("a");
    let one = json!(1);
    let t = json!(true);
    verify_that!(value, j::is_contained_in![&a, &one, &t])
}

#[test]
fn is_contained_in_mixed_types_match_with_inline_borrowed_literals() -> Result<()> {
    verify_that!(
        json!(["a", 1]),
        j::is_contained_in![&json!("a"), &json!(1), &json!(true)]
    )
}

#[test]
fn is_contained_in_mixed_types_match_with_inline_owned_literals() -> Result<()> {
    verify_that!(
        json!(["a", 1]),
        j::is_contained_in![json!("a"), json!(1), json!(true)]
    )
}

#[test]
fn is_contained_in_mixed_types_match_with_mixed_owned_and_borrowed() -> Result<()> {
    let value = json!(["a", 1]);
    let a = json!("a");
    verify_that!(value, j::is_contained_in![a, json!(1), &json!(true)])
}

#[test]
fn is_contained_in_mixed_types_match_with_owned_values_unmatch() -> Result<()> {
    let value = json!(["x", 2]);
    let a = json!("a");
    let one = json!(1);
    let t = json!(true);
    verify_that!(value, not(j::is_contained_in![a, one, t]))
}

#[test]
fn is_contained_in_mixed_types_match_with_borrowed_values_unmatch() -> Result<()> {
    let value = json!(["x", 2]);
    let a = json!("a");
    let one = json!(1);
    let t = json!(true);
    verify_that!(value, not(j::is_contained_in![&a, &one, &t]))
}

#[test]
fn is_contained_in_mixed_types_match_with_inline_borrowed_literals_unmatch() -> Result<()> {
    verify_that!(
        json!(["x", 2]),
        not(j::is_contained_in![&json!("a"), &json!(1), &json!(true)])
    )
}

#[test]
fn is_contained_in_mixed_types_match_with_inline_owned_literals_unmatch() -> Result<()> {
    verify_that!(
        json!(["x", 2]),
        not(j::is_contained_in![json!("a"), json!(1), json!(true)])
    )
}

#[test]
fn is_contained_in_mixed_types_match_with_mixed_owned_and_borrowed_unmatch() -> Result<()> {
    let value = json!(["x", 2]);
    let a = json!("a");
    verify_that!(value, not(j::is_contained_in![a, json!(1), &json!(true)]))
}

#[test]
fn is_contained_in_matches_with_primitive_literals() -> Result<()> {
    let value = json!(["a", 1, true]);
    verify_that!(value, j::is_contained_in!["a", 1i64, true, 22])
}

#[test]
fn is_contained_in_unmatch_with_primitive_literals() -> Result<()> {
    let value = json!(["a", 1, false]);
    verify_that!(value, not(j::is_contained_in!["a", 2i64, true, 2]))
}

#[test]
fn is_contained_in_matches_with_mixed_literals_and_matchers() -> Result<()> {
    let a = 1i64;
    verify_that!(
        json!(["alex", 1, true]),
        j::is_contained_in![starts_with("a"), a, is_true(), 23]
    )
}
