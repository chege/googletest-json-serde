use googletest::Result;
use googletest::prelude::*;
use indoc::indoc;
use serde_json::json;

use googletest_json_serde::json;

#[test]
fn unordered_elements_are_matches_empty_array() -> Result<()> {
    let value = json!([]);
    verify_that!(value, json::unordered_elements_are![])
}

#[test]
fn unordered_elements_are_matches_empty_array_with_trailing_comma() -> Result<()> {
    let value = json!([]);
    verify_that!(value, json::unordered_elements_are![,])
}

#[test]
fn unordered_elements_are_matches_array() -> Result<()> {
    let value = json!(["a", "geddy", "c"]);
    verify_that!(
        value,
        json::unordered_elements_are![eq("a"), starts_with("g"), eq("c")]
    )
}

#[test]
fn unordered_elements_are_matches_unordered() -> Result<()> {
    let value = json!(["a", "b"]);
    verify_that!(value, json::unordered_elements_are![eq("b"), eq("a")])
}

#[test]
fn unordered_elements_are_matches_unordered_with_repetition() -> Result<()> {
    let value = json!(["a", "b", "a", "b", "a"]);
    verify_that!(
        value,
        json::unordered_elements_are![eq("a"), eq("a"), eq("a"), eq("b"), eq("b")]
    )
}

#[test]
fn unordered_elements_are_matches_with_trailing_comma_in_list() -> Result<()> {
    let value = json!(["a", "b", "c"]);
    verify_that!(
        value,
        json::unordered_elements_are![eq("a"), eq("b"), eq("c"),]
    )
}

#[test]
fn unordered_elements_are_explains_mismatch_due_to_wrong_size() -> Result<()> {
    let matcher = json::unordered_elements_are![eq("b"), eq("c"), eq("d")];
    verify_that!(
        matcher.explain_match(&json!(["b", "c"])),
        displays_as(eq("which has size 2 (expected 3)"))
    )
}

#[test]
fn unordered_elements_are_description_no_full_match() -> Result<()> {
    let matcher = json::unordered_elements_are![eq("a"), eq("b"), eq("b")];
    verify_that!(
        matcher.explain_match(&json!(["a", "a", "b"])),
        displays_as(eq(indoc!(
            "
            which does not have a perfect match with the expected elements. The best match found was:
              Actual element String(\"a\") at index 0 matched expected element `is equal to \"a\"` at index 0.
              Actual element String(\"b\") at index 2 matched expected element `is equal to \"b\"` at index 1.
              Actual element String(\"a\") at index 1 did not match any remaining expected element.
              Expected element `is equal to \"b\"` at index 2 did not match any remaining actual element."
        )))
    )
}

#[test]
fn unordered_elements_are_unmatchable_expected_description_mismatch() -> Result<()> {
    let matcher = json::unordered_elements_are![eq(1), eq(2), eq(3)];
    verify_that!(
        matcher.explain_match(&json!([1, 1, 3])),
        displays_as(eq("which has no element matching the expected element #1"))
    )
}

#[test]
fn unordered_elements_are_unmatchable_actual_description_mismatch() -> Result<()> {
    let matcher = json::unordered_elements_are![eq("a"), eq("a"), eq("c")];
    verify_that!(
        matcher.explain_match(&json!(["a", "b", "c"])),
        displays_as(eq("whose element #1 does not match any expected elements"))
    )
}
#[test]
fn unordered_elements_are_matches_when_expected_duplicates_are_fully_matched() -> Result<()> {
    let value = json!(["a", "b"]);
    verify_that!(
        value,
        json::unordered_elements_are![eq("a"), eq("a"), eq("b")]
    )
}
#[test]
fn unordered_elements_are_fails_with_extra_actual_elements() -> Result<()> {
    let value = json!(["a", "b", "c", "d"]);
    verify_that!(
        value,
        not(json::unordered_elements_are![eq("a"), eq("b"), eq("c")])
    )
}

#[test]
fn unordered_elements_are_matches_nested_unordered_arrays() -> Result<()> {
    let value = json!([["x", "y"], ["z"]]);
    verify_that!(
        value,
        json::unordered_elements_are![
            json::unordered_elements_are![eq("z")],
            json::unordered_elements_are![eq("y"), eq("x")]
        ]
    )
}
#[test]
fn unordered_elements_are_fails_and_includes_full_message() -> Result<()> {
    let result = verify_that!(
        json!(["a", "x", "c"]),
        json::unordered_elements_are![eq("a"), eq("b"), eq("c")]
    );
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!(["a", "x", "c"])
            Expected: contains JSON array elements matching in any order:
              0. is equal to "a"
              1. is equal to "b"
              2. is equal to "c"
            Actual: Array [String("a"), String("x"), String("c")],
              whose element #1 does not match any expected elements and no elements match the expected element #1
            "#
        ))))
    )
}

#[test]
fn unordered_elements_are_all_mismatch_unmatchable_message() -> Result<()> {
    let value = json!(["x", "y"]);
    let result = verify_that!(
        value,
        json::unordered_elements_are![eq("only-x"), eq("only-y")]
    );
    verify_that!(
        result,
        err(displays_as(contains_substring(
            "whose elements #0, #1 do not match any expected elements and no elements match the expected elements #0, #1"
        )))
    )
}

#[test]
fn unordered_elements_are_nested_arrays_match() -> Result<()> {
    let value = json!([[1, 2], [3, 4]]);
    verify_that!(
        value,
        json::unordered_elements_are![
            json::unordered_elements_are![eq(1), eq(2)],
            json::unordered_elements_are![eq(3), eq(4)]
        ]
    )
}

#[test]
fn unordered_elements_are_nested_arrays_unmatch() -> Result<()> {
    let value = json!([[1, 2], [3, 5]]);
    verify_that!(
        value,
        not(json::unordered_elements_are![
            json::unordered_elements_are![eq(1), eq(2)],
            json::unordered_elements_are![eq(3), eq(4)]
        ])
    )
}

#[test]
fn unordered_elements_are_produces_correct_failure_message_nested() -> Result<()> {
    let result = verify_that!(
        json!([[1, 2], [3, 5]]),
        json::unordered_elements_are![
            json::unordered_elements_are![eq(1), eq(2)],
            json::unordered_elements_are![eq(3), eq(4)]
        ]
    );
    verify_that!(
        result,
        err(displays_as(contains_substring(
            "whose element #1 does not match any expected elements and no elements match the expected element #1"
        )))
    )
}

#[test]
fn unordered_elements_are_mixed_types_match_with_owned_values() -> Result<()> {
    let value = json!([true, 1, "a"]);
    let a = json!("a");
    let one = json!(1);
    let t = json!(true);
    verify_that!(value, json::unordered_elements_are![a, one, t])
}

#[test]
fn unordered_elements_are_mixed_types_match_with_borrowed_values() -> Result<()> {
    let value = json!([1, true, "a"]);
    let a = json!("a");
    let one = json!(1);
    let t = json!(true);
    verify_that!(value, json::unordered_elements_are![&a, &one, &t])
}

#[test]
fn unordered_elements_are_mixed_types_match_with_inline_borrowed_literals() -> Result<()> {
    verify_that!(
        json!(["a", true, 1]),
        json::unordered_elements_are![&json!("a"), &json!(1), &json!(true)]
    )
}

#[test]
fn unordered_elements_are_mixed_types_match_with_inline_owned_literals() -> Result<()> {
    verify_that!(
        json!([true, "a", 1]),
        json::unordered_elements_are![json!("a"), json!(1), json!(true)]
    )
}

#[test]
fn unordered_elements_are_mixed_types_match_with_mixed_owned_and_borrowed() -> Result<()> {
    let value = json!([1, true, "a"]);
    let a = json!("a");
    verify_that!(
        value,
        json::unordered_elements_are![a, json!(1), &json!(true)]
    )
}

#[test]
fn unordered_elements_are_mixed_types_match_with_owned_values_unmatch() -> Result<()> {
    let value = json!([true, 2, "b"]);
    let a = json!("a");
    let one = json!(1);
    let t = json!(true);
    verify_that!(value, not(json::unordered_elements_are![a, one, t]))
}

#[test]
fn unordered_elements_are_mixed_types_match_with_borrowed_values_unmatch() -> Result<()> {
    let value = json!([false, "a", 2]);
    let a = json!("a");
    let one = json!(1);
    let t = json!(true);
    verify_that!(value, not(json::unordered_elements_are![&a, &one, &t]))
}

#[test]
fn unordered_elements_are_mixed_types_match_with_inline_borrowed_literals_unmatch() -> Result<()> {
    verify_that!(
        json!(["b", 2, false]),
        not(json::unordered_elements_are![
            &json!("a"),
            &json!(1),
            &json!(true)
        ])
    )
}

#[test]
fn unordered_elements_are_mixed_types_match_with_inline_owned_literals_unmatch() -> Result<()> {
    verify_that!(
        json!([false, "x", 9]),
        not(json::unordered_elements_are![
            json!("a"),
            json!(1),
            json!(true)
        ])
    )
}

#[test]
fn unordered_elements_are_mixed_types_match_with_mixed_owned_and_borrowed_unmatch() -> Result<()> {
    let value = json!([false, "z", 3]);
    let a = json!("a");
    verify_that!(
        value,
        not(json::unordered_elements_are![a, json!(1), &json!(true)])
    )
}

#[test]
fn unordered_elements_are_matches_with_primitive_literals() -> Result<()> {
    let value = json!(["a", 1, true]);
    verify_that!(value, json::unordered_elements_are!["a", 1i64, true])
}

#[test]
fn unordered_elements_are_unmatch_with_primitive_literals() -> Result<()> {
    let value = json!(["a", 1, false]);
    verify_that!(value, not(json::unordered_elements_are!["a", 2i64, true]))
}

#[test]
fn unordered_elements_are_matches_with_mixed_literals_and_matchers() -> Result<()> {
    let a = 1i64;
    verify_that!(
        json!(["alex", 1, true]),
        json::unordered_elements_are![starts_with("a"), a, is_true()]
    )
}
