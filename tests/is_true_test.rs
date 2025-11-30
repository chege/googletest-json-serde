use googletest::Result;
use googletest::prelude::*;
use googletest_json_serde::json;
use indoc::indoc;
use serde_json::json;

#[test]
fn is_true_matches_true() -> Result<()> {
    verify_that!(json!(true), json::is_true())
}

#[test]
fn is_true_rejects_false() -> Result<()> {
    verify_that!(json!(false), not(json::is_true()))
}

#[test]
fn is_true_fails_and_includes_full_message_for_false() -> Result<()> {
    let result = verify_that!(json!(false), json::is_true());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!(false)
            Expected: JSON true
            Actual: Bool(false),
              which is JSON false
            "#
        ))))
    )
}

#[test]
fn is_true_fails_and_includes_full_message_for_non_bool() -> Result<()> {
    let result = verify_that!(json!(0), json::is_true());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!(0)
            Expected: JSON true
            Actual: Number(0),
              which is a JSON number
            "#
        ))))
    )
}
