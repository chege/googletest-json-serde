use googletest::Result;
use googletest::prelude::*;
use googletest_json_serde::json as j;
use indoc::indoc;
use serde_json::json;

#[test]
fn contains_each_matches_one_to_one() -> Result<()> {
    verify_that!(
        json!(["alpha", "bravo", "charlie"]),
        j::contains_each![starts_with("a"), starts_with("b"), starts_with("c")]
    )
}

#[test]
fn contains_each_trailing_comma() -> Result<()> {
    verify_that!(
        json!(["alpha", "bravo", "charlie"]),
        j::contains_each![starts_with("a"), starts_with("b"), starts_with("c"),]
    )
}

#[test]
fn contains_each_empty_matchers() -> Result<()> {
    verify_that!(json!(["a", "b", "c"]), j::contains_each![])
}

#[test]
fn contains_each_empty_matchers_trailing_comma() -> Result<()> {
    verify_that!(json!(["a", "b", "c"]), j::contains_each![,])
}

#[test]
fn contains_each_empty_input_and_matchers() -> Result<()> {
    verify_that!(json!([]), j::contains_each![])
}

#[test]
fn contains_each_excess_elements() -> Result<()> {
    verify_that!(
        json!(["admin", "beta", "cool", "delta"]),
        j::contains_each![starts_with("b"), starts_with("c"), starts_with("d")]
    )
}

#[test]
fn contains_each_unmatched_fails() -> Result<()> {
    verify_that!(
        json!(["alpha", "beta", "charlie"]),
        not(j::contains_each![
            starts_with("b"),
            starts_with("c"),
            starts_with("x")
        ])
    )
}

#[test]
fn contains_each_explains_mismatch_due_to_wrong_size() -> Result<()> {
    let matcher = j::contains_each![gt(2), eq(3), eq(4)];
    verify_that!(
        matcher.explain_match(&json!([2, 3])),
        displays_as(eq("which has size 2 (expected at least 3)"))
    )
}

#[test]
fn contains_each_explains_missing_element_in_mismatch() -> Result<()> {
    let matcher = j::contains_each![eq(2), eq(3), eq(4)];
    verify_that!(
        matcher.explain_match(&json!([1, 2, 3])),
        displays_as(eq("which has no element matching the expected element #2"))
    )
}

#[test]
fn contains_each_mixed_types_match() -> Result<()> {
    verify_that!(
        json!(["alpha", 1, true]),
        j::contains_each![starts_with("a"), eq(1), eq(true)]
    )
}

#[test]
fn contains_each_mixed_types_unmatch() -> Result<()> {
    verify_that!(
        json!(["bravo", 2, false]),
        not(j::contains_each![starts_with("b"), eq(2), eq(true)])
    )
}

#[test]
fn contains_each_with_parentheses() -> Result<()> {
    verify_that!(
        json!(["xeno", "yodel"]),
        j::contains_each!(starts_with("x"), starts_with("y"))
    )
}

#[test]
fn contains_each_empty_input_and_nonempty_matchers() -> Result<()> {
    verify_that!(json!([]), not(j::contains_each![starts_with("a")]))
}

#[test]
fn contains_each_duplicate_elements() -> Result<()> {
    verify_that!(
        json!(["alpha", "atom", "bravo"]),
        j::contains_each![starts_with("a"), starts_with("a"), starts_with("b")]
    )
}

#[test]
fn contains_each_input_smaller_than_matchers() -> Result<()> {
    verify_that!(
        json!(["alpha"]),
        not(j::contains_each![starts_with("a"), starts_with("b")])
    )
}

#[test]
fn contains_each_multiple_missing_elements_in_mismatch() -> Result<()> {
    let matcher = j::contains_each![eq(2), eq(3), eq(4), eq(5)];
    verify_that!(
        matcher.explain_match(&json!([2])),
        displays_as(eq("which has size 1 (expected at least 4)"))
    )
}

#[test]
fn contains_each_completely_unmatched_elements() -> Result<()> {
    verify_that!(
        json!(["xeno", "yodel", "zeta"]),
        not(j::contains_each![
            starts_with("a"),
            starts_with("b"),
            starts_with("c")
        ])
    )
}

#[test]
fn contains_each_wrong_type_failure_message() -> Result<()> {
    let result = verify_that!(json!({"a": 1, "b": 2}), j::contains_each![starts_with("a")]);
    verify_that!(
        result,
        err(displays_as(contains_substring("which is not a JSON array")))
    )
}

#[test]
fn contains_each_nested_full_match() -> Result<()> {
    verify_that!(
        json!([["x", "y"], ["a", "b"]]),
        j::contains_each![
            j::contains_each![eq("x"), eq("y")],
            j::contains_each![eq("a"), eq("b")]
        ]
    )
}
#[test]
fn contains_each_nested_partial_match() -> Result<()> {
    verify_that!(
        json!([["x", "y"], ["a", "b"]]),
        j::contains_each![j::contains_each![eq("x")], j::contains_each![eq("a")]]
    )
}

#[test]
fn contains_each_partial_nested_mismatch() -> Result<()> {
    verify_that!(
        json!([["x", "y"], ["a", "b"]]),
        not(j::contains_each![
            j::contains_each![eq("x"), eq("z")],
            j::contains_each![eq("a"), eq("b")]
        ])
    )
}

#[test]
fn contains_each_nested_wrong_type() -> Result<()> {
    verify_that!(
        json!([{"x": 1}, ["a", "b"]]),
        not(j::contains_each![
            j::contains_each![eq("x")],
            j::contains_each![eq("a"), eq("b")]
        ])
    )
}

#[test]
fn contains_each_empty_input_nested_matchers() -> Result<()> {
    verify_that!(
        json!([]),
        not(j::contains_each![j::contains_each![eq("a")]])
    )
}

#[test]
fn contains_each_produces_correct_failure_message() -> Result<()> {
    let result = verify_that!(
        json!(["a", "x", "c"]),
        j::contains_each![eq("a"), eq("b"), eq("c")]
    );
    verify_that!(
        result,
        err(displays_as(starts_with(indoc!(
            r#"
                Value of: json!(["a", "x", "c"])
                Expected: contains JSON array elements matching in any order:
                  0. is equal to "a"
                  1. is equal to "b"
                  2. is equal to "c"
                Actual: Array [String("a"), String("x"), String("c")],
                  which has no element matching the expected element #1"#
        ))))
    )
}

#[test]
fn contains_each_mixed_types_match_with_owned_values() -> Result<()> {
    let arr = vec![json!("alpha"), json!(1), json!(true)];
    verify_that!(json!(arr), j::contains_each![eq("alpha"), eq(1), eq(true)])
}

#[test]
fn contains_each_mixed_types_match_with_borrowed_values() -> Result<()> {
    let arr = [&json!("alpha"), &json!(1), &json!(true)];
    verify_that!(
        json!([arr[0], arr[1], arr[2]]),
        j::contains_each![eq("alpha"), eq(1), eq(true)]
    )
}

#[test]
fn contains_each_mixed_types_match_with_inline_borrowed_literals() -> Result<()> {
    let a = json!("alpha");
    let b = json!(1);
    let c = json!(true);
    verify_that!(
        json!([&a, &b, &c]),
        j::contains_each![eq("alpha"), eq(1), eq(true)]
    )
}

#[test]
fn contains_each_mixed_types_match_with_inline_owned_literals() -> Result<()> {
    verify_that!(
        json!([json!("alpha"), json!(1), json!(true)]),
        j::contains_each![eq("alpha"), eq(1), eq(true)]
    )
}

#[test]
fn contains_each_mixed_types_match_with_mixed_owned_and_borrowed() -> Result<()> {
    let a = json!("alpha");
    verify_that!(
        json!([&a, json!(1), json!(true)]),
        j::contains_each![eq("alpha"), eq(1), eq(true)]
    )
}

#[test]
fn contains_each_mixed_types_match_with_owned_values_unmatch() -> Result<()> {
    let value = json!(["a", 1, false]);
    let a = json!("a");
    let one = json!(1);
    let t = json!(true);
    verify_that!(value, not(j::contains_each![a, one, t]))
}

#[test]
fn contains_each_mixed_types_match_with_borrowed_values_unmatch() -> Result<()> {
    let value = json!(["a", 1, false]);
    let a = json!("a");
    let one = json!(1);
    let t = json!(true);
    verify_that!(value, not(j::contains_each![&a, &one, &t]))
}

#[test]
fn contains_each_mixed_types_match_with_inline_borrowed_literals_unmatch() -> Result<()> {
    verify_that!(
        json!(["a", 1, false]),
        not(j::contains_each![&json!("a"), &json!(1), &json!(true)])
    )
}

#[test]
fn contains_each_mixed_types_match_with_inline_owned_literals_unmatch() -> Result<()> {
    verify_that!(
        json!(["a", 1, false]),
        not(j::contains_each![json!("a"), json!(1), json!(true)])
    )
}

#[test]
fn contains_each_mixed_types_match_with_mixed_owned_and_borrowed_unmatch() -> Result<()> {
    let value = json!(["a", 1, false]);
    let a = json!("a");
    verify_that!(value, not(j::contains_each![a, json!(1), &json!(true)]))
}

#[test]
fn contains_each_matches_with_primitive_literals() -> Result<()> {
    let value = json!(["x", "y", "z", 1, true]);
    verify_that!(value, j::contains_each!["x", 1i64, true])
}

#[test]
fn contains_each_unmatch_with_primitive_literals() -> Result<()> {
    let value = json!(["a", "b", "c", false]);
    verify_that!(value, not(j::contains_each!["x", 1i64, true]))
}

#[test]
fn contains_each_matches_with_mixed_literals_and_matchers() -> Result<()> {
    let a = 1i64;
    verify_that!(
        json!(["alex", "bravo", 1, true]),
        j::contains_each![starts_with("a"), a, is_true()]
    )
}
