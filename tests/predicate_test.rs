use googletest::Result;
use googletest::description::Description;
use googletest::prelude::*;
use googletest_json_serde::json;
use indoc::indoc;
use serde_json::json as j;

#[test]
fn predicate_matches_when_predicate_returns_true() -> Result<()> {
    let matcher = json::predicate(|v| v.is_number());
    verify_that!(j!(123), matcher)
}

#[test]
fn predicate_unmatches_when_predicate_returns_false() -> Result<()> {
    let matcher = json::predicate(|v| v.is_number());
    verify_that!(j!("string"), not(matcher))
}

#[test]
fn predicate_default_descriptions_are_used_when_no_description_provided() -> Result<()> {
    let matcher = json::predicate(|v| v.is_null());
    let result = verify_that!(j!("x"), matcher);
    verify_that!(
        result,
        err(displays_as(contains_substring(
            "which does not match the predicate"
        )))
    )
}

#[test]
fn predicate_with_description_overrides_default() -> Result<()> {
    let matcher = json::predicate(|v| v.is_string()).with_description("a string", "not a string");
    let result = verify_that!(j!(1), matcher);
    verify_that!(
        result,
        err(displays_as(contains_substring(
            "which does not match the predicate"
        )))
    )
}

#[test]
fn predicate_explains_json_type() -> Result<()> {
    let matcher = json::predicate(|v| v.is_boolean());
    verify_that!(
        matcher.explain_match(&j!(true)),
        displays_as(contains_substring("which does not match the predicate"))
    )
}

#[test]
fn predicate_supports_closures_with_captures() -> Result<()> {
    let threshold = 10;
    let matcher = json::predicate(move |v| v.as_i64().unwrap_or_default() > threshold)
        .with_description("is above threshold", "is below or equal to threshold");
    verify_that!(j!(20), matcher)
}

#[test]
fn predicate_handles_various_json_types() -> Result<()> {
    let matcher = json::predicate(|v| !v.is_null());
    verify_that!(j!(123), &matcher)?;
    verify_that!(j!("hello"), &matcher)?;
    verify_that!(j!(true), &matcher)?;
    verify_that!(j!(null), not(&matcher))
}

#[test]
fn predicate_with_description_and_explain_match_overrides_both() -> Result<()> {
    let matcher = json::predicate(|v| v.is_number())
        .with_description("a number", "not a number")
        .with_explain_fn(|v| Description::new().text(format!("which is actually {:?}", v)));

    let result = verify_that!(j!("str"), matcher);
    verify_that!(
        result,
        err(displays_as(starts_with(indoc!(
            r#"
            Value of: j!("str")
            Expected: a number
            Actual: String("str"),
              which is actually String("str")
            "#
        ))))
    )
}
#[test]
fn predicate_without_description_but_with_explain_match_uses_explain_fn() -> Result<()> {
    let matcher = json::predicate(|v| v.is_number())
        .with_explain_fn(|v| Description::new().text(format!("which is actually {:?}", v)));

    let result = verify_that!(j!(true), matcher);
    verify_that!(
        result,
        err(displays_as(starts_with(indoc!(
            r#"
            Value of: j!(true)
            Expected: matches predicate
            Actual: Bool(true),
              which is actually Bool(true)
            "#
        ))))
    )
}

#[test]
fn predicate_with_explain_fn_uses_captured_value_in_message() -> Result<()> {
    let expected_type = "number".to_string();
    let matcher = json::predicate(|v| v.is_number()).with_explain_fn(move |v| {
        Description::new().text(format!(
            "which was {:?}, expected a JSON {}",
            v, expected_type
        ))
    });

    let result = verify_that!(j!(true), matcher);
    verify_that!(
        result,
        err(displays_as(starts_with(indoc!(
            r#"
            Value of: j!(true)
            Expected: matches predicate
            Actual: Bool(true),
              which was Bool(true), expected a JSON number
            "#
        ))))
    )
}
