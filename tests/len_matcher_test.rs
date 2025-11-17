use googletest::prelude::*;
use googletest_json_serde::json;
use serde_json::json as j;

#[test]
fn len_matches_correct_length() -> Result<()> {
    verify_that!(j!(["a", "b", "c"]), json::len!(eq(3)))
}

#[test]
fn len_matches_using_gt() -> Result<()> {
    verify_that!(j!(["x", "y", "z"]), json::len!(gt(2)))
}

#[test]
fn len_matches_using_ge() -> Result<()> {
    verify_that!(j!(["a", "b"]), json::len!(ge(2)))
}

#[test]
fn len_unmatches_wrong_length() -> Result<()> {
    verify_that!(j!(["a", "b"]), not(json::len!(eq(3))))
}

#[test]
fn len_explain_match_includes_actual_length() -> Result<()> {
    let matcher = json::len!(eq(2));
    verify_that!(
        matcher.explain_match(&j!(["x", "y", "z"])),
        displays_as(eq("which has length 3, which isn't equal to 2"))
    )
}

#[test]
fn len_wrong_type_fails() -> Result<()> {
    let result = verify_that!(j!({"a": 1}), json::len!(eq(1)));
    verify_that!(
        result,
        err(displays_as(contains_substring("which is not a JSON array")))
    )
}

#[test]
fn len_matches_empty_array() -> Result<()> {
    verify_that!(j!([]), json::len!(eq(0)))
}

#[test]
fn len_unmatches_empty_array_when_expected_nonzero() -> Result<()> {
    verify_that!(j!([]), not(json::len!(gt(0))))
}

#[test]
fn len_nested_array_length() -> Result<()> {
    verify_that!(j!([["a"], ["b"], ["c"]]), json::len!(eq(3)))
}

#[test]
fn len_mixed_types_array() -> Result<()> {
    verify_that!(j!(["a", 1, true]), json::len!(eq(3)))
}

#[test]
fn len_owned_values() -> Result<()> {
    let arr = vec![j!("a"), j!(1), j!(true)];
    verify_that!(j!(arr), json::len!(eq(3)))
}

#[test]
fn len_borrowed_values() -> Result<()> {
    let a = j!("a");
    let b = j!(1);
    let c = j!(true);
    let borrowed = j!([&a, &b, &c]);
    verify_that!(borrowed, json::len!(eq(3)))
}

#[test]
fn len_inline_owned_literals() -> Result<()> {
    verify_that!(j!([j!("x"), j!("y")]), json::len!(ge(2)))
}

#[test]
fn len_inline_borrowed_literals() -> Result<()> {
    verify_that!(j!([&j!("x"), &j!("y")]), json::len!(eq(2)))
}

#[test]
fn len_input_number_fails() -> Result<()> {
    let result = verify_that!(j!(42), json::len!(eq(1)));
    verify_that!(
        result,
        err(displays_as(contains_substring("which is not a JSON array")))
    )
}

#[test]
fn len_input_string_fails() -> Result<()> {
    let result = verify_that!(j!("hello"), json::len!(le(2)));
    verify_that!(
        result,
        err(displays_as(contains_substring("which is not a JSON array")))
    )
}

#[test]
fn len_input_bool_fails() -> Result<()> {
    let result = verify_that!(j!(true), json::len!(eq(1)));
    verify_that!(
        result,
        err(displays_as(contains_substring("which is not a JSON array")))
    )
}

#[test]
fn len_explain_match_wrong_size_message() -> Result<()> {
    let matcher = json::len!(eq(2));
    verify_that!(
        matcher.explain_match(&j!(["a"])),
        displays_as(eq("which has length 1, which isn't equal to 2"))
    )
}

#[test]
fn len_matches_with_inline_borrowed_nested_literals() -> Result<()> {
    let x = j!("x");
    let y = j!("y");
    verify_that!(j!([&x, &y]), json::len!(eq(2)))
}

#[test]
fn len_matches_with_mixed_owned_and_borrowed() -> Result<()> {
    let a = j!("a");
    verify_that!(j!([&a, j!(1), j!(true)]), json::len!(eq(3)))
}

#[test]
fn len_matches_with_variable_in_matcher() -> Result<()> {
    let expected = 3usize;
    verify_that!(j!(["a", 1, true]), json::len!(eq(expected)))
}

#[test]
fn len_literal_exact_match() -> Result<()> {
    verify_that!(j!(["a", "b", "c"]), json::len!(3))
}

#[test]
fn len_literal_unmatch() -> Result<()> {
    verify_that!(j!(["a", "b"]), not(json::len!(3)))
}

#[test]
fn len_literal_on_empty_array() -> Result<()> {
    verify_that!(j!([]), json::len!(0))
}

#[test]
fn len_literal_wrong_type_fails() -> Result<()> {
    let result = verify_that!(j!({"x": 1}), json::len!(1));
    verify_that!(
        result,
        err(displays_as(contains_substring("which is not a JSON array")))
    )
}

#[test]
fn len_literal_nested_match() -> Result<()> {
    verify_that!(j!([["x"], ["y"]]), json::len!(2))
}
