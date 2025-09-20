use googletest::Result;
use googletest::prelude::*;
use googletest_serde_json::json;
use indoc::indoc;
use serde_json::{Value, json as j};

#[test]
fn elements_are_matches_json_array() -> Result<()> {
    let value = j!(["a", "b", "c"]);
    verify_that!(value, json::elements_are![eq("a"), eq("b"), eq("c")])
}

#[test]
fn elements_are_supports_trailing_comma() -> Result<()> {
    let value = j!(["a", "b", "c"]);
    verify_that!(value, json::elements_are![eq("a"), eq("b"), eq("c"),])
}

#[test]
fn elements_are_returns_no_match_when_expected_and_actual_sizes_differ() -> Result<()> {
    let value = j!(["a", "b"]);
    verify_that!(value, not(json::elements_are![eq("a"), eq("b"), eq("c")]))
}

#[test]
fn elements_are_produces_correct_failure_message() -> Result<()> {
    let result = verify_that!(
        j!(["a", "x", "c"]),
        json::elements_are![eq("a"), eq("b"), eq("c")]
    );
    verify_that!(
        result,
        err(displays_as(starts_with(indoc!(
            r#"
                Value of: j!(["a", "x", "c"])
                Expected: has JSON array elements:
                  0. is equal to "a"
                  1. is equal to "b"
                  2. is equal to "c"
                Actual: Array [String("a"), String("x"), String("c")],
                  where element #1 is String("x"), which isn't equal to "b""#
        ))))
    )
}

#[test]
fn elements_are_produces_correct_failure_message_nested() -> Result<()> {
    let result = verify_that!(
        j!([[0, 1], [1, 2]]),
        json::elements_are![
            json::elements_are![eq(1), eq(2)],
            json::elements_are![eq(2), eq(3)]
        ]
    );
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
                Expected: has JSON array elements:
                  0. has JSON array elements:
                       0. is equal to 1
                       1. is equal to 2
                  1. has JSON array elements:
                       0. is equal to 2
                       1. is equal to 3
                Actual: Array [
                    Array [
                        Number(0),
                        Number(1),
                    ],
                    Array [
                        Number(1),
                        Number(2),
                    ],
                ],
                  where:
                    * element #0 is Array [Number(0), Number(1)], where:
                        * element #0 is Number(0), which isn't equal to 1
                        * element #1 is Number(1), which isn't equal to 2
                    * element #1 is Array [Number(1), Number(2)], where:
                        * element #0 is Number(1), which isn't equal to 2
                        * element #1 is Number(2), which isn't equal to 3"#
        ))))
    )
}

#[test]
fn elements_are_explain_match_wrong_size() -> Result<()> {
    let matcher = json::elements_are![eq("a")];
    verify_that!(
        matcher.explain_match(&j!(["a", "b"])),
        displays_as(eq("whose size is 2"))
    )
}

fn create_matcher() -> impl for<'v> Matcher<&'v Value> {
    json::elements_are![eq("a")]
}
#[test]
fn elements_are_works_when_matcher_is_created_in_subroutine() -> Result<()> {
    verify_that!(j!(["a"]), create_matcher())
}

#[test]
fn elements_are_nested_arrays_match() -> Result<()> {
    let value = j!([["x", "y"], ["z"]]);
    verify_that!(
        value,
        json::elements_are![
            json::elements_are![eq("x"), eq("y")],
            json::elements_are![eq("z")]
        ]
    )
}

#[test]
fn elements_are_empty_matches_empty_array() -> Result<()> {
    let value = j!([]);
    verify_that!(value, json::elements_are![])
}

#[test]
fn elements_are_empty_does_not_match_nonempty_array() -> Result<()> {
    let value = j!(["unexpected"]);
    verify_that!(value, not(json::elements_are![]))
}

#[test]
fn elements_are_not_array_failure_message() -> Result<()> {
    let result = verify_that!(
        j!("not-an-array"),
        json::elements_are![eq("a"), eq("b"), eq("c")]
    );
    verify_that!(
        result,
        err(displays_as(starts_with(indoc!(
            r#"
                Value of: j!("not-an-array")
                Expected: has JSON array elements:
                  0. is equal to "a"
                  1. is equal to "b"
                  2. is equal to "c"
                Actual: String("not-an-array"),
                  where the type is not array"#
        ))))
    )
}

#[test]
fn elements_are_wrong_order() -> Result<()> {
    let value = j!(["a", "c", "b"]);
    verify_that!(value, not(json::elements_are![eq("a"), eq("b"), eq("c")]))
}

#[test]
fn elements_are_mixed_types_match() -> Result<()> {
    let value = j!(["a", 1, true]);
    verify_that!(value, json::elements_are![eq("a"), eq(1), eq(true)])
}

#[test]
fn elements_are_mixed_types_unmatch() -> Result<()> {
    let value = j!(["a", 1, false]);
    verify_that!(value, not(json::elements_are![eq("a"), eq(1), eq(true)]))
}

#[test]
fn elements_are_wrong_type() -> Result<()> {
    let value = j!(42);
    verify_that!(value, not(json::elements_are![eq(42)]))
}

#[test]
fn elements_are_nested_arrays_unmatch() -> Result<()> {
    let value = j!([["a"], ["b", "c"]]);
    verify_that!(
        value,
        not(json::elements_are![
            json::elements_are![eq("a"), eq("b")],
            json::elements_are![eq("c")]
        ])
    )
}

#[test]
fn elements_are_dupes_match() -> Result<()> {
    let value = j!(["x", "x"]);
    verify_that!(value, json::elements_are![eq("x"), eq("x")])
}

#[test]
fn elements_are_dupes_unmatch() -> Result<()> {
    let value = j!(["x", "y"]);
    verify_that!(value, not(json::elements_are![eq("x"), eq("x")]))
}
