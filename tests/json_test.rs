use googletest::prelude::*;
use googletest_json_serde::json;
use indoc::indoc;
use serde_json::json;
#[test]
fn any_value_match() {
    assert_that!(json!("2112"), json::any_value());
}
#[test]
fn any_value_unmatch() {
    assert_that!(json!(null), not(json::any_value()));
}

#[test]
fn any_value_fails_and_includes_full_message() -> Result<()> {
    let result = verify_that!(json!(null), json::any_value());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!(null)
            Expected: any JSON Value
            Actual: Null,
              which is JSON Null
            "#
        ))))
    )
}

#[test]
fn is_null_match() {
    assert_that!(json!(null), json::is_null());
}
#[test]
fn is_null_unmatch() {
    assert_that!(json!("not null"), not(json::is_null()));
}
#[test]
fn is_null_fails_and_includes_full_message() -> Result<()> {
    let result = verify_that!(json!("not null"), json::is_null());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!("not null")
            Expected: JSON Null
            Actual: String("not null"),
              which is a JSON String
            "#
        ))))
    )
}

#[test]
fn is_string_match() {
    assert_that!(json!("a string"), json::is_string());
}
#[test]
fn is_string_unmatch() {
    assert_that!(json!(123), not(json::is_string()));
}
#[test]
fn is_string_fails_and_includes_full_message() -> Result<()> {
    let result = verify_that!(json!(123), json::is_string());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!(123)
            Expected: a JSON String
            Actual: Number(123),
              which is a JSON Number
            "#
        ))))
    )
}

#[test]
fn is_number_match() {
    assert_that!(json!(123), json::is_number());
}
#[test]
fn is_number_unmatch() {
    assert_that!(json!(true), not(json::is_number()));
}
#[test]
fn is_number_fails_and_includes_full_message() -> Result<()> {
    let result = verify_that!(json!(true), json::is_number());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!(true)
            Expected: a JSON Number
            Actual: Bool(true),
              which is a JSON Boolean
            "#
        ))))
    )
}

#[test]
fn is_boolean_match() {
    assert_that!(json!(true), json::is_boolean());
}
#[test]
fn is_boolean_unmatch() {
    assert_that!(json!([1, 2, 3]), not(json::is_boolean()));
}
#[test]
fn is_boolean_fails_and_includes_full_message() -> Result<()> {
    let result = verify_that!(json!([1, 2, 3]), json::is_boolean());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!([1, 2, 3])
            Expected: a JSON Boolean
            Actual: Array [Number(1), Number(2), Number(3)],
              which is a JSON Array
            "#
        ))))
    )
}

#[test]
fn is_array_match() {
    assert_that!(json!([1, 2, 3]), json::is_array());
}
#[test]
fn is_array_unmatch() {
    assert_that!(json!({"key": "value"}), not(json::is_array()));
}
#[test]
fn is_array_fails_and_includes_full_message() -> Result<()> {
    let result = verify_that!(json!({"key": "value"}), json::is_array());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!({"key": "value"})
            Expected: a JSON Array
            Actual: Object {"key": String("value")},
              which is a JSON Object
            "#
        ))))
    )
}

#[test]
fn is_object_match() {
    assert_that!(json!({"key": "value"}), json::is_object());
}
#[test]
fn is_object_unmatch() {
    assert_that!(json!(false), not(json::is_object()));
}
#[test]
fn is_object_fails_and_includes_full_message() -> Result<()> {
    let result = verify_that!(json!(false), json::is_object());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!(false)
            Expected: a JSON Object
            Actual: Bool(false),
              which is a JSON Boolean
            "#
        ))))
    )
}
