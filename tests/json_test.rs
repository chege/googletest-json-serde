#![allow(deprecated)]

use googletest::Result;
use googletest::prelude::*;
use googletest_json_serde::json;
use indoc::indoc;
use serde_json::json;

#[test]
fn any_value_matches_non_null() -> Result<()> {
    verify_that!(json!("2112"), json::any_value())
}

#[test]
fn any_value_rejects_null() -> Result<()> {
    verify_that!(json!(null), not(json::any_value()))
}

#[test]
fn any_value_fails_and_includes_full_message() -> Result<()> {
    let result = verify_that!(json!(null), json::any_value());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!(null)
            Expected: any JSON value
            Actual: Null,
              which is a JSON null
            "#
        ))))
    )
}

#[test]
fn is_not_null_matches_strings() -> Result<()> {
    verify_that!(json!("2112"), json::is_not_null())
}
#[test]
fn is_not_null_rejects_null() -> Result<()> {
    verify_that!(json!(null), not(json::is_not_null()))
}
#[test]
fn is_not_null_fails_and_includes_full_message() -> Result<()> {
    let result = verify_that!(json!(null), json::is_not_null());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!(null)
            Expected: not JSON null
            Actual: Null,
              which is a JSON null
            "#
        ))))
    )
}

#[test]
fn is_null_matches_null() -> Result<()> {
    verify_that!(json!(null), json::is_null())
}
#[test]
fn is_null_rejects_non_null() -> Result<()> {
    verify_that!(json!("not null"), not(json::is_null()))
}
#[test]
fn is_null_fails_and_includes_full_message() -> Result<()> {
    let result = verify_that!(json!("not null"), json::is_null());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!("not null")
            Expected: JSON null
            Actual: String("not null"),
              which is a JSON string
            "#
        ))))
    )
}

#[test]
fn is_string_matches_string() -> Result<()> {
    verify_that!(json!("a string"), json::is_string())
}
#[test]
fn is_string_rejects_number() -> Result<()> {
    verify_that!(json!(123), not(json::is_string()))
}
#[test]
fn is_string_fails_and_includes_full_message() -> Result<()> {
    let result = verify_that!(json!(123), json::is_string());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!(123)
            Expected: a JSON string
            Actual: Number(123),
              which is a JSON number
            "#
        ))))
    )
}

#[test]
fn is_number_matches_number() -> Result<()> {
    verify_that!(json!(123), json::is_number())
}
#[test]
fn is_number_rejects_bool() -> Result<()> {
    verify_that!(json!(true), not(json::is_number()))
}
#[test]
fn is_number_fails_and_includes_full_message() -> Result<()> {
    let result = verify_that!(json!(true), json::is_number());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!(true)
            Expected: a JSON number
            Actual: Bool(true),
              which is a JSON boolean
            "#
        ))))
    )
}

#[test]
fn is_integer_matches_integer_number() -> Result<()> {
    verify_that!(json!(123), json::is_integer())
}

#[test]
fn is_integer_rejects_fractional_number() -> Result<()> {
    verify_that!(json!(3.5), not(json::is_integer()))
}

#[test]
fn is_integer_rejects_float_without_fraction_but_encoded_as_float() -> Result<()> {
    verify_that!(json!(2.0), not(json::is_integer()))
}

#[test]
fn is_integer_rejects_large_imprecise_float() -> Result<()> {
    verify_that!(json!(1e23), not(json::is_integer()))
}

#[test]
fn is_integer_rejects_non_number() -> Result<()> {
    verify_that!(json!("string"), not(json::is_integer()))
}

#[test]
fn is_integer_fails_and_includes_full_message_for_fractional_number() -> Result<()> {
    let result = verify_that!(json!(3.5), json::is_integer());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!(3.5)
            Expected: an integer JSON number
            Actual: Number(3.5),
              which is a non-integer JSON number
            "#
        ))))
    )
}

#[test]
fn is_integer_fails_and_includes_full_message_for_non_number() -> Result<()> {
    let result = verify_that!(json!("vampire"), json::is_integer());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!("vampire")
            Expected: an integer JSON number
            Actual: String("vampire"),
              which is a JSON string
            "#
        ))))
    )
}

#[test]
fn is_whole_number_matches_integer_and_float_with_zero_fraction() -> Result<()> {
    verify_that!(json!(123), json::is_whole_number())?;
    verify_that!(json!(123.0), json::is_whole_number())
}

#[test]
fn is_whole_number_rejects_fractional_number() -> Result<()> {
    verify_that!(json!(3.5), not(json::is_whole_number()))
}

#[test]
fn is_whole_number_rejects_non_number() -> Result<()> {
    verify_that!(json!("string"), not(json::is_whole_number()))
}

#[test]
fn is_whole_number_fails_and_includes_full_message_for_fractional_number() -> Result<()> {
    let result = verify_that!(json!(3.5), json::is_whole_number());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!(3.5)
            Expected: a JSON number with no fractional part
            Actual: Number(3.5),
              which is a JSON number with a fractional part
            "#
        ))))
    )
}

#[test]
fn is_whole_number_fails_and_includes_full_message_for_non_number() -> Result<()> {
    let result = verify_that!(json!("vampire"), json::is_whole_number());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!("vampire")
            Expected: a JSON number with no fractional part
            Actual: String("vampire"),
              which is a JSON string
            "#
        ))))
    )
}

#[test]
fn is_boolean_matches_bool() -> Result<()> {
    verify_that!(json!(true), json::is_boolean())
}
#[test]
fn is_boolean_rejects_array() -> Result<()> {
    verify_that!(json!([1, 2, 3]), not(json::is_boolean()))
}
#[test]
fn is_boolean_fails_and_includes_full_message() -> Result<()> {
    let result = verify_that!(json!([1, 2, 3]), json::is_boolean());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!([1, 2, 3])
            Expected: a JSON boolean
            Actual: Array [Number(1), Number(2), Number(3)],
              which is a JSON array
            "#
        ))))
    )
}

#[test]
fn is_array_matches_array() -> Result<()> {
    verify_that!(json!([1, 2, 3]), json::is_array())
}
#[test]
fn is_array_rejects_object() -> Result<()> {
    verify_that!(json!({"key": "value"}), not(json::is_array()))
}
#[test]

fn is_array_fails_and_includes_full_message() -> Result<()> {
    let result = verify_that!(json!({"key": "value"}), json::is_array());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!({"key": "value"})
            Expected: a JSON array
            Actual: Object {"key": String("value")},
              which is a JSON object
            "#
        ))))
    )
}

#[test]
fn is_empty_array_matches_empty() -> Result<()> {
    verify_that!(json!([]), json::is_empty_array())
}

#[test]
fn is_empty_array_rejects_non_empty() -> Result<()> {
    verify_that!(json!([1]), not(json::is_empty_array()))
}

#[test]
fn is_empty_array_fails_for_non_empty_array_and_includes_full_message() -> Result<()> {
    let result = verify_that!(json!([1]), json::is_empty_array());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!([1])
            Expected: an empty JSON array
            Actual: Array [Number(1)],
              which is a non-empty JSON array
            "#
        ))))
    )
}

#[test]
fn is_empty_array_fails_for_boolean_and_includes_full_message() -> Result<()> {
    let result = verify_that!(json!(true), json::is_empty_array());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!(true)
            Expected: an empty JSON array
            Actual: Bool(true),
              which is a JSON boolean
            "#
        ))))
    )
}

#[test]
fn is_object_matches_object() -> Result<()> {
    verify_that!(json!({"key": "value"}), json::is_object())
}
#[test]
fn is_object_rejects_boolean() -> Result<()> {
    verify_that!(json!(false), not(json::is_object()))
}

#[test]
fn is_empty_object_matches_empty() -> Result<()> {
    verify_that!(json!({}), json::is_empty_object())
}

#[test]
fn is_empty_object_rejects_non_empty() -> Result<()> {
    verify_that!(json!({"a": 1}), not(json::is_empty_object()))
}

#[test]
fn is_empty_object_rejects_non_object() -> Result<()> {
    verify_that!(json!(["a"]), not(json::is_empty_object()))
}
#[test]
fn is_object_fails_and_includes_full_message() -> Result<()> {
    let result = verify_that!(json!(false), json::is_object());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!(false)
            Expected: a JSON object
            Actual: Bool(false),
              which is a JSON boolean
            "#
        ))))
    )
}
