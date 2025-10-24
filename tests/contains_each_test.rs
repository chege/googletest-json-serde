use googletest::Result;
use googletest::prelude::*;
use googletest_json_serde::json;
use indoc::indoc;
use serde_json::json as j;

#[test]
fn contains_each_matches_one_to_one() -> Result<()> {
    verify_that!(
        j!(["alpha", "bravo", "charlie"]),
        json::contains_each![starts_with("a"), starts_with("b"), starts_with("c")]
    )
}

#[test]
fn contains_each_trailing_comma() -> Result<()> {
    verify_that!(
        j!(["alpha", "bravo", "charlie"]),
        json::contains_each![starts_with("a"), starts_with("b"), starts_with("c"),]
    )
}

#[test]
fn contains_each_empty_matchers() -> Result<()> {
    verify_that!(j!(["a", "b", "c"]), json::contains_each![])
}

#[test]
fn contains_each_empty_matchers_trailing_comma() -> Result<()> {
    verify_that!(j!(["a", "b", "c"]), json::contains_each![,])
}

#[test]
fn contains_each_empty_input_and_matchers() -> Result<()> {
    verify_that!(j!([]), json::contains_each![])
}

#[test]
fn contains_each_excess_elements() -> Result<()> {
    verify_that!(
        j!(["admin", "beta", "cool", "delta"]),
        json::contains_each![starts_with("b"), starts_with("c"), starts_with("d")]
    )
}

#[test]
fn contains_each_unmatched_fails() -> Result<()> {
    verify_that!(
        j!(["alpha", "beta", "charlie"]),
        not(json::contains_each![
            starts_with("b"),
            starts_with("c"),
            starts_with("x")
        ])
    )
}

#[test]
fn contains_each_explains_mismatch_due_to_wrong_size() -> Result<()> {
    let matcher = json::contains_each![gt(2), eq(3), eq(4)];
    verify_that!(
        matcher.explain_match(&j!([2, 3])),
        displays_as(eq("which has size 2 (expected at least 3)"))
    )
}

#[test]
fn contains_each_explains_missing_element_in_mismatch() -> Result<()> {
    let matcher = json::contains_each![eq(2), eq(3), eq(4)];
    verify_that!(
        matcher.explain_match(&j!([1, 2, 3])),
        displays_as(eq("which has no element matching the expected element #2"))
    )
}

#[test]
fn contains_each_mixed_types_match() -> Result<()> {
    verify_that!(
        j!(["alpha", 1, true]),
        json::contains_each![starts_with("a"), eq(1), eq(true)]
    )
}

#[test]
fn contains_each_mixed_types_unmatch() -> Result<()> {
    verify_that!(
        j!(["bravo", 2, false]),
        not(json::contains_each![starts_with("b"), eq(2), eq(true)])
    )
}

#[test]
fn contains_each_with_parentheses() -> Result<()> {
    verify_that!(
        j!(["xeno", "yodel"]),
        json::contains_each!(starts_with("x"), starts_with("y"))
    )
}

#[test]
fn contains_each_empty_input_and_nonempty_matchers() -> Result<()> {
    verify_that!(j!([]), not(json::contains_each![starts_with("a")]))
}

#[test]
fn contains_each_duplicate_elements() -> Result<()> {
    verify_that!(
        j!(["alpha", "atom", "bravo"]),
        json::contains_each![starts_with("a"), starts_with("a"), starts_with("b")]
    )
}

#[test]
fn contains_each_input_smaller_than_matchers() -> Result<()> {
    verify_that!(
        j!(["alpha"]),
        not(json::contains_each![starts_with("a"), starts_with("b")])
    )
}

#[test]
fn contains_each_multiple_missing_elements_in_mismatch() -> Result<()> {
    let matcher = json::contains_each![eq(2), eq(3), eq(4), eq(5)];
    verify_that!(
        matcher.explain_match(&j!([2])),
        displays_as(eq("which has size 1 (expected at least 4)"))
    )
}

#[test]
fn contains_each_completely_unmatched_elements() -> Result<()> {
    verify_that!(
        j!(["xeno", "yodel", "zeta"]),
        not(json::contains_each![
            starts_with("a"),
            starts_with("b"),
            starts_with("c")
        ])
    )
}

#[test]
fn contains_each_wrong_type_failure_message() -> Result<()> {
    let result = verify_that!(j!({"a": 1, "b": 2}), json::contains_each![starts_with("a")]);
    verify_that!(
        result,
        err(displays_as(contains_substring("which is not a JSON array")))
    )
}

#[test]
fn contains_each_nested_full_match() -> Result<()> {
    verify_that!(
        j!([["x", "y"], ["a", "b"]]),
        json::contains_each![
            json::contains_each![eq("x"), eq("y")],
            json::contains_each![eq("a"), eq("b")]
        ]
    )
}
#[test]
fn contains_each_nested_partial_match() -> Result<()> {
    verify_that!(
        j!([["x", "y"], ["a", "b"]]),
        json::contains_each![json::contains_each![eq("x")], json::contains_each![eq("a")]]
    )
}

#[test]
fn contains_each_partial_nested_mismatch() -> Result<()> {
    verify_that!(
        j!([["x", "y"], ["a", "b"]]),
        not(json::contains_each![
            json::contains_each![eq("x"), eq("z")],
            json::contains_each![eq("a"), eq("b")]
        ])
    )
}

#[test]
fn contains_each_nested_wrong_type() -> Result<()> {
    verify_that!(
        j!([{"x": 1}, ["a", "b"]]),
        not(json::contains_each![
            json::contains_each![eq("x")],
            json::contains_each![eq("a"), eq("b")]
        ])
    )
}

#[test]
fn contains_each_empty_input_nested_matchers() -> Result<()> {
    verify_that!(
        j!([]),
        not(json::contains_each![json::contains_each![eq("a")]])
    )
}

#[test]
fn contains_each_produces_correct_failure_message() -> Result<()> {
    let result = verify_that!(
        j!(["a", "x", "c"]),
        json::contains_each![eq("a"), eq("b"), eq("c")]
    );
    verify_that!(
        result,
        err(displays_as(starts_with(indoc!(
            r#"
                Value of: j!(["a", "x", "c"])
                Expected: contains JSON array elements matching in any order:
                  0. is equal to "a"
                  1. is equal to "b"
                  2. is equal to "c"
                Actual: Array [String("a"), String("x"), String("c")],
                  which has no element matching the expected element #1"#
        ))))
    )
}
