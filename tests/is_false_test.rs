use googletest::Result;
use googletest::prelude::*;
use googletest_json_serde::json;
use indoc::indoc;
use serde_json::json;

#[test]
fn is_false_matches_false() -> Result<()> {
    verify_that!(json!(false), json::is_false())
}

#[test]
fn is_false_rejects_true() -> Result<()> {
    verify_that!(json!(true), not(json::is_false()))
}

#[test]
fn is_false_fails_and_includes_full_message_for_true() -> Result<()> {
    let result = verify_that!(json!(true), json::is_false());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!(true)
            Expected: JSON false
            Actual: Bool(true),
              which is JSON true
            "#
        ))))
    )
}

#[test]
fn is_false_fails_and_includes_full_message_for_non_bool() -> Result<()> {
    let result = verify_that!(json!(0), json::is_false());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!(0)
            Expected: JSON false
            Actual: Number(0),
              which is a JSON number
            "#
        ))))
    )
}
