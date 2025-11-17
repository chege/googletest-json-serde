use googletest::matcher::MatcherResult;
use googletest::prelude::*;
use googletest_json_serde::json;
use serde_json::json as j;

#[test]
fn each_matches_all_elements() -> Result<()> {
    verify_that!(j!([1, 2, 3]), json::each!(gt(0)))
}

#[test]
fn each_unmatches_when_any_element_fails() -> Result<()> {
    verify_that!(j!([1, -2, 3]), not(json::each!(gt(0))))
}

#[test]
fn each_accepts_mixed_types_as_long_as_matcher_handles_them() -> Result<()> {
    verify_that!(j!(["abc", "ax", "aaa"]), json::each!(starts_with("a")))
}

#[test]
fn each_literal_number() -> Result<()> {
    verify_that!(j!([5, 5, 5]), json::each!(5))
}

#[test]
fn each_literal_string() -> Result<()> {
    verify_that!(j!(["x", "x"]), json::each!("x"))
}

#[test]
fn each_literal_bool() -> Result<()> {
    verify_that!(j!([true, true, true]), json::each!(true))
}

#[test]
fn each_literal_number_unmatch() -> Result<()> {
    verify_that!(j!([1, 2, 3]), not(json::each!(1)))
}

#[test]
fn each_literal_string_unmatch() -> Result<()> {
    verify_that!(j!(["a", "b"]), not(json::each!("a")))
}

#[test]
fn each_literal_bool_unmatch() -> Result<()> {
    verify_that!(j!([true, false]), not(json::each!(true)))
}

#[test]
fn each_fails_on_non_array() -> Result<()> {
    let result = verify_that!(j!(123), json::each!(gt(0)));
    verify_that!(
        result,
        err(displays_as(contains_substring("not a JSON array")))
    )
}

#[test]
fn each_fails_on_object() -> Result<()> {
    let result = verify_that!(j!({"a":1}), json::each!(gt(0)));
    verify_that!(
        result,
        err(displays_as(contains_substring("not a JSON array")))
    )
}

#[test]
fn each_fails_on_null() -> Result<()> {
    let result = verify_that!(j!(null), json::each!(json::is_string()));
    verify_that!(
        result,
        err(displays_as(contains_substring("not a JSON array")))
    )
}

#[test]
fn each_matches_empty_array() -> Result<()> {
    verify_that!(j!([]), json::each!(gt(0)))
}

#[test]
fn each_numeric_literals() -> Result<()> {
    verify_that!(j!([10, 20, 30]), json::each!(ge(10)))
}

#[test]
fn each_explain_match_includes_first_failure() -> Result<()> {
    let matcher = json::each!(gt(10));
    verify_that!(
        matcher.explain_match(&j!([20, 5, 30])),
        displays_as(contains_substring("element #1"))
    )
}

#[test]
fn each_mixed_types_owned_values() -> Result<()> {
    let arr = vec![j!("a"), j!("ax"), j!("aaa")];
    verify_that!(j!(arr), json::each!(starts_with("a")))
}

#[test]
fn each_mixed_types_borrowed_values() -> Result<()> {
    let a = j!("a");
    let b = j!("abc");
    let c = j!("ax");
    verify_that!(j!([&a, &b, &c]), json::each!(starts_with("a")))
}

#[test]
fn each_inline_owned_literals() -> Result<()> {
    verify_that!(j!([1, 2, 3]), json::each!(gt(0)))
}

#[test]
fn each_inline_borrowed_literals() -> Result<()> {
    verify_that!(j!([2, 3]), json::each!(ge(2)))
}

#[test]
fn each_variable_inside_matcher() -> Result<()> {
    let min = 5;
    verify_that!(j!([6, 7, 8]), json::each!(gt(min)))
}

#[test]
fn each_unmatch_variable_inside_matcher() -> Result<()> {
    let limit = 10;
    verify_that!(j!([12, 5]), not(json::each!(gt(limit))))
}

#[test]
fn each_mixed_owned_and_borrowed() -> Result<()> {
    let a = j!("aaa");
    verify_that!(j!([&a, j!("ax")]), json::each!(starts_with("a")))
}

#[test]
fn each_describe_messages() -> Result<()> {
    let matcher = json::each!(gt(10));
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
    let matcher = json::each!(gt(5));
    let msg = matcher.explain_match(&j!([10, 3, 20]));
    verify_that!(msg, displays_as(contains_substring("element #1")))?;
    verify_that!(msg, displays_as(contains_substring("element #1 (3)")))?;
    Ok(())
}
