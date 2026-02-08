use googletest::matcher::MatcherResult;
use googletest::prelude::*;
use googletest_json_serde::json as j;
use serde_json::json;

#[test]
fn each_matches_all_elements() -> Result<()> {
    verify_that!(json!([1, 2, 3]), j::each!(gt(0)))
}

#[test]
fn each_unmatches_when_any_element_fails() -> Result<()> {
    verify_that!(json!([1, -2, 3]), not(j::each!(gt(0))))
}

#[test]
fn each_accepts_mixed_types_as_long_as_matcher_handles_them() -> Result<()> {
    verify_that!(json!(["abc", "ax", "aaa"]), j::each!(starts_with("a")))
}

#[test]
fn each_literal_number() -> Result<()> {
    verify_that!(json!([5, 5, 5]), j::each!(5))
}

#[test]
fn each_literal_string() -> Result<()> {
    verify_that!(json!(["x", "x"]), j::each!("x"))
}

#[test]
fn each_literal_bool() -> Result<()> {
    verify_that!(json!([true, true, true]), j::each!(true))
}

#[test]
fn each_literal_number_unmatch() -> Result<()> {
    verify_that!(json!([1, 2, 3]), not(j::each!(1)))
}

#[test]
fn each_literal_string_unmatch() -> Result<()> {
    verify_that!(json!(["a", "b"]), not(j::each!("a")))
}

#[test]
fn each_literal_bool_unmatch() -> Result<()> {
    verify_that!(json!([true, false]), not(j::each!(true)))
}

#[test]
fn each_fails_on_non_array() -> Result<()> {
    let result = verify_that!(json!(123), j::each!(gt(0)));
    verify_that!(
        result,
        err(displays_as(contains_substring("not a JSON array")))
    )
}

#[test]
fn each_fails_on_object() -> Result<()> {
    let result = verify_that!(json!({"a":1}), j::each!(gt(0)));
    verify_that!(
        result,
        err(displays_as(contains_substring("not a JSON array")))
    )
}

#[test]
fn each_fails_on_null() -> Result<()> {
    let result = verify_that!(json!(null), j::each!(starts_with("a")));
    verify_that!(
        result,
        err(displays_as(contains_substring("not a JSON array")))
    )
}

#[test]
fn each_matches_empty_array() -> Result<()> {
    verify_that!(json!([]), j::each!(gt(0)))
}

#[test]
fn each_numeric_literals() -> Result<()> {
    verify_that!(json!([10, 20, 30]), j::each!(ge(10)))
}

#[test]
fn each_explain_match_includes_first_failure() -> Result<()> {
    let matcher = j::each!(gt(10));
    verify_that!(
        matcher.explain_match(&json!([20, 5, 30])),
        displays_as(contains_substring("element #1"))
    )
}

#[test]
fn each_mixed_types_owned_values() -> Result<()> {
    let arr = vec![json!("a"), json!("ax"), json!("aaa")];
    verify_that!(json!(arr), j::each!(starts_with("a")))
}

#[test]
fn each_mixed_types_borrowed_values() -> Result<()> {
    let a = json!("a");
    let b = json!("abc");
    let c = json!("ax");
    verify_that!(json!([&a, &b, &c]), j::each!(starts_with("a")))
}

#[test]
fn each_inline_owned_literals() -> Result<()> {
    verify_that!(json!([1, 2, 3]), j::each!(gt(0)))
}

#[test]
fn each_inline_borrowed_literals() -> Result<()> {
    verify_that!(json!([2, 3]), j::each!(ge(2)))
}

#[test]
fn each_variable_inside_matcher() -> Result<()> {
    let min = 5;
    verify_that!(json!([6, 7, 8]), j::each!(gt(min)))
}

#[test]
fn each_unmatch_variable_inside_matcher() -> Result<()> {
    let limit = 10;
    verify_that!(json!([12, 5]), not(j::each!(gt(limit))))
}

#[test]
fn each_mixed_owned_and_borrowed() -> Result<()> {
    let a = json!("aaa");
    verify_that!(json!([&a, json!("ax")]), j::each!(starts_with("a")))
}

#[test]
fn each_describe_messages() -> Result<()> {
    let matcher = j::each!(gt(10));
    verify_that!(
        matcher.describe(MatcherResult::Match),
        displays_as(contains_substring("JSON array where each element"))
    )?;
    verify_that!(
        matcher.describe(MatcherResult::NoMatch),
        displays_as(contains_substring("less than or equal"))
    )
}

#[test]
fn each_explain_failure_message() -> Result<()> {
    let matcher = j::each!(gt(5));
    let msg = matcher.explain_match(&json!([10, 3, 20]));
    verify_that!(msg, displays_as(contains_substring("element #1")))?;
    verify_that!(msg, displays_as(contains_substring("element #1 (3)")))?;
    Ok(())
}

#[test]
fn each_explains_all_elements_matched_for_not() -> Result<()> {
    let result = verify_that!(json!([1, 1]), not(j::each!(eq(1))));
    verify_that!(
        result,
        err(displays_as(contains_substring("all 2 elements matched")))
    )
}
