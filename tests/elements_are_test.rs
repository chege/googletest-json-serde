use googletest::Result;
use googletest::prelude::*;
use googletest_json_serde::json as j;
use indoc::indoc;
use serde_json::{Value, json};

#[test]
fn elements_are_matches_json_array() -> Result<()> {
    let value = json!(["alex", "b", "c"]);
    verify_that!(value, j::elements_are![starts_with("a"), eq("b"), eq("c")])
}

#[test]
fn elements_are_matches_json_array_with_parentheses() -> Result<()> {
    let value = json!(["a", "b", "c"]);
    verify_that!(value, j::elements_are!(eq("a"), eq("b"), eq("c")))
}

#[test]
fn elements_are_supports_trailing_comma() -> Result<()> {
    let value = json!(["a", "b", "c"]);
    verify_that!(value, j::elements_are![eq("a"), eq("b"), eq("c"),])
}

#[test]
fn elements_are_size_mismatch_extra_expected() -> Result<()> {
    let value = json!(["a", "b"]);
    verify_that!(value, not(j::elements_are![eq("a"), eq("b"), eq("c")]))
}

#[test]
fn elements_are_input_not_array_failure_message() -> Result<()> {
    let result = verify_that!(json!("not-an-array"), j::elements_are![eq("a")]);
    verify_that!(
        result,
        err(displays_as(contains_substring("the type is not array")))
    )
}

#[test]
fn elements_are_input_wrong_type_number() -> Result<()> {
    let result = verify_that!(json!(42), j::elements_are![eq(42)]);
    verify_that!(
        result,
        err(displays_as(contains_substring("the type is not array")))
    )
}
#[test]
fn elements_are_returns_no_match_when_expected_and_actual_sizes_differ() -> Result<()> {
    let value = json!(["a", "b"]);
    verify_that!(value, not(j::elements_are![eq("a"), eq("b"), eq("c")]))
}

#[test]
fn elements_are_produces_correct_failure_message() -> Result<()> {
    let result = verify_that!(
        json!(["a", "x", "c"]),
        j::elements_are![eq("a"), eq("b"), eq("c")]
    );
    verify_that!(
        result,
        err(displays_as(starts_with(indoc!(
            r#"
                Value of: json!(["a", "x", "c"])
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
        json!([[0, 1], [1, 2]]),
        j::elements_are![
            j::elements_are![eq(1), eq(2)],
            j::elements_are![eq(2), eq(3)]
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
    let matcher = j::elements_are![eq("a")];
    verify_that!(
        matcher.explain_match(&json!(["a", "b"])),
        displays_as(eq("whose size is 2"))
    )
}

fn create_matcher() -> impl for<'v> Matcher<&'v Value> {
    j::elements_are![eq("a")]
}
#[test]
fn elements_are_works_when_matcher_is_created_in_subroutine() -> Result<()> {
    verify_that!(json!(["a"]), create_matcher())
}

#[test]
fn elements_are_nested_arrays_match() -> Result<()> {
    let value = json!([["moving", "y"], ["z"]]);
    verify_that!(
        value,
        j::elements_are![
            j::elements_are![starts_with("m"), eq("y")],
            j::elements_are![eq("z")]
        ]
    )
}

#[test]
fn elements_are_empty_matches_empty_array() -> Result<()> {
    let value = json!([]);
    verify_that!(value, j::elements_are![])
}

#[test]
fn elements_are_empty_does_not_match_nonempty_array() -> Result<()> {
    let value = json!(["unexpected"]);
    verify_that!(value, not(j::elements_are![]))
}

#[test]
fn elements_are_not_array_failure_message() -> Result<()> {
    let result = verify_that!(
        json!("not-an-array"),
        j::elements_are![eq("a"), eq("b"), eq("c")]
    );
    verify_that!(
        result,
        err(displays_as(starts_with(indoc!(
            r#"
                Value of: json!("not-an-array")
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
    let value = json!(["a", "c", "b"]);
    verify_that!(value, not(j::elements_are![eq("a"), eq("b"), eq("c")]))
}

#[test]
fn elements_are_mixed_types_match() -> Result<()> {
    let value = json!(["a", 1, true]);
    verify_that!(value, j::elements_are![eq("a"), eq(1), eq(true)])
}

#[test]
fn elements_are_mixed_types_unmatch() -> Result<()> {
    let value = json!(["a", 1, false]);
    verify_that!(value, not(j::elements_are![eq("a"), eq(1), eq(true)]))
}

#[test]
fn elements_are_wrong_type() -> Result<()> {
    let value = json!(42);
    verify_that!(value, not(j::elements_are![eq(42)]))
}

#[test]
fn elements_are_nested_arrays_unmatch() -> Result<()> {
    let value = json!([["a"], ["b", "c"]]);
    verify_that!(
        value,
        not(j::elements_are![
            j::elements_are![eq("a"), eq("b")],
            j::elements_are![eq("c")]
        ])
    )
}

#[test]
fn elements_are_dupes_match() -> Result<()> {
    let value = json!(["x", "x"]);
    verify_that!(value, j::elements_are![eq("x"), eq("x")])
}

#[test]
fn elements_are_dupes_unmatch() -> Result<()> {
    let value = json!(["x", "y"]);
    verify_that!(value, not(j::elements_are![eq("x"), eq("x")]))
}

#[test]
fn elements_are_mixed_types_match_with_owned_values() -> Result<()> {
    let value = json!(["a", 1, true]);
    let a = json!("a");
    let one = json!(1);
    let t = json!(true);
    verify_that!(value, j::elements_are![a, one, t])
}
#[test]
fn elements_are_mixed_types_match_with_borrowed_values() -> Result<()> {
    let value = json!(["a", 1, true]);
    let a = json!("a");
    let one = json!(1);
    let t = json!(true);
    verify_that!(value, j::elements_are![&a, &one, &t])
}
#[test]
fn elements_are_mixed_types_match_with_inline_borrowed_literals() -> Result<()> {
    verify_that!(
        json!(["a", 1, true]),
        j::elements_are![&json!("a"), &json!(1), &json!(true)]
    )
}
#[test]
fn elements_are_mixed_types_match_with_inline_owned_literals() -> Result<()> {
    verify_that!(
        json!(["a", 1, true]),
        j::elements_are![json!("a"), json!(1), json!(true)]
    )
}
#[test]
fn elements_are_mixed_types_match_with_mixed_owned_and_borrowed() -> Result<()> {
    let value = json!(["a", 1, true]);
    let a = json!("a");
    verify_that!(value, j::elements_are![a, json!(1), &json!(true)])
}

#[test]
fn elements_are_mixed_types_match_with_owned_values_unmatch() -> Result<()> {
    let value = json!(["a", 1, false]);
    let a = json!("a");
    let one = json!(1);
    let t = json!(true);
    verify_that!(value, not(j::elements_are![a, one, t]))
}
#[test]
fn elements_are_mixed_types_match_with_borrowed_values_unmatch() -> Result<()> {
    let value = json!(["a", 1, false]);
    let a = json!("a");
    let one = json!(1);
    let t = json!(true);
    verify_that!(value, not(j::elements_are![&a, &one, &t]))
}
#[test]
fn elements_are_mixed_types_match_with_inline_borrowed_literals_unmatch() -> Result<()> {
    verify_that!(
        json!(["a", 1, false]),
        not(j::elements_are![&json!("a"), &json!(1), &json!(true)])
    )
}
#[test]
fn elements_are_mixed_types_match_with_inline_owned_literals_unmatch() -> Result<()> {
    verify_that!(
        json!(["a", 1, false]),
        not(j::elements_are![json!("a"), json!(1), json!(true)])
    )
}
#[test]
fn elements_are_mixed_types_match_with_mixed_owned_and_borrowed_unmatch() -> Result<()> {
    let value = json!(["a", 1, false]);
    let a = json!("a");
    verify_that!(value, not(j::elements_are![a, json!(1), &json!(true)]))
}

#[test]
fn elements_are_matches_with_primitive_literals() -> Result<()> {
    let value = json!(["a", 1, true]);
    verify_that!(value, j::elements_are!["a", 1i64, true])
}

#[test]
fn elements_are_unmatch_with_primitive_literals() -> Result<()> {
    let value = json!(["a", 1, false]);
    verify_that!(value, not(j::elements_are!["a", 2i64, true]))
}

#[test]
fn elements_are_matches_with_mixed_literals_and_matchers() -> Result<()> {
    let a = 1i64;
    verify_that!(
        json!(["alex", 1, true]),
        j::elements_are![starts_with("a"), a, is_true()]
    )
}
