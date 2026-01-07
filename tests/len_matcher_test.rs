use googletest::prelude::*;
use googletest_json_serde::json as j;
use serde_json::json;

#[test]
fn len_matches_correct_length() -> Result<()> {
    verify_that!(json!(["a", "b", "c"]), j::len!(eq(3)))
}

#[test]
fn len_matches_using_gt() -> Result<()> {
    verify_that!(json!(["x", "y", "z"]), j::len!(gt(2)))
}

#[test]
fn len_matches_using_ge() -> Result<()> {
    verify_that!(json!(["a", "b"]), j::len!(ge(2)))
}

#[test]
fn len_unmatches_wrong_length() -> Result<()> {
    verify_that!(json!(["a", "b"]), not(j::len!(eq(3))))
}

#[test]
fn len_explain_match_includes_actual_length() -> Result<()> {
    let matcher = j::len!(eq(2));
    verify_that!(
        matcher.explain_match(&json!(["x", "y", "z"])),
        displays_as(eq("which has length 3, which isn't equal to 2"))
    )
}

#[test]
fn len_wrong_type_fails() -> Result<()> {
    let result = verify_that!(json!({"a": 1}), j::len!(eq(1)));
    verify_that!(
        result,
        err(displays_as(contains_substring("which is not a JSON array")))
    )
}

#[test]
fn len_matches_empty_array() -> Result<()> {
    verify_that!(json!([]), j::len!(eq(0)))
}

#[test]
fn len_unmatches_empty_array_when_expected_nonzero() -> Result<()> {
    verify_that!(json!([]), not(j::len!(gt(0))))
}

#[test]
fn len_nested_array_length() -> Result<()> {
    verify_that!(json!([["a"], ["b"], ["c"]]), j::len!(eq(3)))
}

#[test]
fn len_mixed_types_array() -> Result<()> {
    verify_that!(json!(["a", 1, true]), j::len!(eq(3)))
}

#[test]
fn len_owned_values() -> Result<()> {
    let arr = vec![json!("a"), json!(1), json!(true)];
    verify_that!(json!(arr), j::len!(eq(3)))
}

#[test]
fn len_borrowed_values() -> Result<()> {
    let a = json!("a");
    let b = json!(1);
    let c = json!(true);
    let borrowed = json!([&a, &b, &c]);
    verify_that!(borrowed, j::len!(eq(3)))
}

#[test]
fn len_inline_owned_literals() -> Result<()> {
    verify_that!(json!([json!("x"), json!("y")]), j::len!(ge(2)))
}

#[test]
fn len_inline_borrowed_literals() -> Result<()> {
    verify_that!(json!([&json!("x"), &json!("y")]), j::len!(eq(2)))
}

#[test]
fn len_input_number_fails() -> Result<()> {
    let result = verify_that!(json!(42), j::len!(eq(1)));
    verify_that!(
        result,
        err(displays_as(contains_substring("which is not a JSON array")))
    )
}

#[test]
fn len_input_string_fails() -> Result<()> {
    let result = verify_that!(json!("hello"), j::len!(le(2)));
    verify_that!(
        result,
        err(displays_as(contains_substring("which is not a JSON array")))
    )
}

#[test]
fn len_input_bool_fails() -> Result<()> {
    let result = verify_that!(json!(true), j::len!(eq(1)));
    verify_that!(
        result,
        err(displays_as(contains_substring("which is not a JSON array")))
    )
}

#[test]
fn len_explain_match_wrong_size_message() -> Result<()> {
    let matcher = j::len!(eq(2));
    verify_that!(
        matcher.explain_match(&json!(["a"])),
        displays_as(eq("which has length 1, which isn't equal to 2"))
    )
}

#[test]
fn len_matches_with_inline_borrowed_nested_literals() -> Result<()> {
    let x = json!("x");
    let y = json!("y");
    verify_that!(json!([&x, &y]), j::len!(eq(2)))
}

#[test]
fn len_matches_with_mixed_owned_and_borrowed() -> Result<()> {
    let a = json!("a");
    verify_that!(json!([&a, json!(1), json!(true)]), j::len!(eq(3)))
}

#[test]
fn len_matches_with_variable_in_matcher() -> Result<()> {
    let expected = 3usize;
    verify_that!(json!(["a", 1, true]), j::len!(eq(expected)))
}

#[test]
fn len_literal_exact_match() -> Result<()> {
    verify_that!(json!(["a", "b", "c"]), j::len!(3))
}

#[test]
fn len_literal_unmatch() -> Result<()> {
    verify_that!(json!(["a", "b"]), not(j::len!(3)))
}

#[test]
fn len_literal_on_empty_array() -> Result<()> {
    verify_that!(json!([]), j::len!(0))
}

#[test]
fn len_literal_wrong_type_fails() -> Result<()> {
    let result = verify_that!(json!({"x": 1}), j::len!(1));
    verify_that!(
        result,
        err(displays_as(contains_substring("which is not a JSON array")))
    )
}

#[test]
fn len_literal_nested_match() -> Result<()> {
    verify_that!(json!([["x"], ["y"]]), j::len!(2))
}
