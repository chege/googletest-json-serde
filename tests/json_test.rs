#![allow(deprecated)]

use googletest::Result;
use googletest::prelude::*;
use googletest_json_serde::json as j;
use indoc::indoc;
use serde_json::json;

#[test]
fn any_value_matches_non_null() -> Result<()> {
    verify_that!(json!("2112"), j::any_value())
}

#[test]
fn any_value_rejects_null() -> Result<()> {
    verify_that!(json!(null), not(j::any_value()))
}

#[test]
fn any_value_fails_and_includes_full_message() -> Result<()> {
    let result = verify_that!(json!(null), j::any_value());
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
    verify_that!(json!("2112"), j::is_not_null())
}
#[test]
fn is_not_null_rejects_null() -> Result<()> {
    verify_that!(json!(null), not(j::is_not_null()))
}
#[test]
fn is_not_null_fails_and_includes_full_message() -> Result<()> {
    let result = verify_that!(json!(null), j::is_not_null());
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
    verify_that!(json!(null), j::is_null())
}
#[test]
fn is_null_rejects_non_null() -> Result<()> {
    verify_that!(json!("not null"), not(j::is_null()))
}
#[test]
fn is_null_fails_and_includes_full_message() -> Result<()> {
    let result = verify_that!(json!("not null"), j::is_null());
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
    verify_that!(json!("a string"), j::is_string())
}
#[test]
fn is_string_rejects_number() -> Result<()> {
    verify_that!(json!(123), not(j::is_string()))
}
#[test]
fn is_string_fails_and_includes_full_message() -> Result<()> {
    let result = verify_that!(json!(123), j::is_string());
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
fn is_empty_string_matches_empty() -> Result<()> {
    verify_that!(json!(""), j::is_empty_string())
}

#[test]
fn is_empty_string_rejects_non_empty() -> Result<()> {
    verify_that!(json!("hi"), not(j::is_empty_string()))
}

#[test]
fn is_empty_string_fails_for_non_empty_string_and_includes_full_message() -> Result<()> {
    let result = verify_that!(json!("hi"), j::is_empty_string());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!("hi")
            Expected: an empty JSON string
            Actual: String("hi"),
              which is a non-empty JSON string
            "#
        ))))
    )
}

#[test]
fn is_empty_string_fails_for_number_and_includes_full_message() -> Result<()> {
    let result = verify_that!(json!(123), j::is_empty_string());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!(123)
            Expected: an empty JSON string
            Actual: Number(123),
              which is a JSON number
            "#
        ))))
    )
}

#[test]
fn is_non_empty_string_matches_non_empty() -> Result<()> {
    verify_that!(json!("hi"), j::is_non_empty_string())
}

#[test]
fn is_non_empty_string_rejects_empty() -> Result<()> {
    verify_that!(json!(""), not(j::is_non_empty_string()))
}

#[test]
fn is_non_empty_string_fails_for_empty_string_and_includes_full_message() -> Result<()> {
    let result = verify_that!(json!(""), j::is_non_empty_string());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!("")
            Expected: a non-empty JSON string
            Actual: String(""),
              which is an empty JSON string
            "#
        ))))
    )
}

#[test]
fn is_non_empty_string_fails_for_boolean_and_includes_full_message() -> Result<()> {
    let result = verify_that!(json!(true), j::is_non_empty_string());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!(true)
            Expected: a non-empty JSON string
            Actual: Bool(true),
              which is a JSON boolean
            "#
        ))))
    )
}

#[test]
fn is_number_matches_number() -> Result<()> {
    verify_that!(json!(123), j::is_number())
}
#[test]
fn is_number_rejects_bool() -> Result<()> {
    verify_that!(json!(true), not(j::is_number()))
}
#[test]
fn is_number_fails_and_includes_full_message() -> Result<()> {
    let result = verify_that!(json!(true), j::is_number());
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
    verify_that!(json!(123), j::is_integer())
}

#[test]
fn is_integer_rejects_fractional_number() -> Result<()> {
    verify_that!(json!(3.5), not(j::is_integer()))
}

#[test]
fn is_integer_rejects_float_without_fraction_but_encoded_as_float() -> Result<()> {
    verify_that!(json!(2.0), not(j::is_integer()))
}

#[test]
fn is_integer_rejects_large_imprecise_float() -> Result<()> {
    verify_that!(json!(1e23), not(j::is_integer()))
}

#[test]
fn is_integer_rejects_non_number() -> Result<()> {
    verify_that!(json!("string"), not(j::is_integer()))
}

#[test]
fn is_integer_fails_and_includes_full_message_for_fractional_number() -> Result<()> {
    let result = verify_that!(json!(3.5), j::is_integer());
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
    let result = verify_that!(json!("vampire"), j::is_integer());
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
    verify_that!(json!(123), j::is_whole_number())?;
    verify_that!(json!(123.0), j::is_whole_number())
}

#[test]
fn is_whole_number_rejects_fractional_number() -> Result<()> {
    verify_that!(json!(3.5), not(j::is_whole_number()))
}

#[test]
fn is_whole_number_rejects_non_number() -> Result<()> {
    verify_that!(json!("string"), not(j::is_whole_number()))
}

#[test]
fn is_whole_number_fails_and_includes_full_message_for_fractional_number() -> Result<()> {
    let result = verify_that!(json!(3.5), j::is_whole_number());
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
    let result = verify_that!(json!("vampire"), j::is_whole_number());
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
fn is_fractional_number_matches_fractional() -> Result<()> {
    verify_that!(json!(3.5), j::is_fractional_number())
}

#[test]
fn is_fractional_number_rejects_integer_and_zero_fraction_float() -> Result<()> {
    verify_that!(json!(3), not(j::is_fractional_number()))?;
    verify_that!(json!(3.0), not(j::is_fractional_number()))
}

#[test]
fn is_fractional_number_rejects_non_number() -> Result<()> {
    verify_that!(json!(true), not(j::is_fractional_number()))
}

#[test]
fn is_fractional_number_fails_and_includes_full_message_for_integer() -> Result<()> {
    let result = verify_that!(json!(5), j::is_fractional_number());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!(5)
            Expected: a JSON number with a fractional part
            Actual: Number(5),
              which is a JSON number without a fractional part
            "#
        ))))
    )
}

#[test]
fn is_fractional_number_fails_and_includes_full_message_for_non_number() -> Result<()> {
    let result = verify_that!(json!("vampire"), j::is_fractional_number());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!("vampire")
            Expected: a JSON number with a fractional part
            Actual: String("vampire"),
              which is a JSON string
            "#
        ))))
    )
}

#[test]
fn is_boolean_matches_bool() -> Result<()> {
    verify_that!(json!(true), j::is_boolean())
}
#[test]
fn is_boolean_rejects_array() -> Result<()> {
    verify_that!(json!([1, 2, 3]), not(j::is_boolean()))
}
#[test]
fn is_boolean_fails_and_includes_full_message() -> Result<()> {
    let result = verify_that!(json!([1, 2, 3]), j::is_boolean());
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
    verify_that!(json!([1, 2, 3]), j::is_array())
}
#[test]
fn is_array_rejects_object() -> Result<()> {
    verify_that!(json!({"key": "value"}), not(j::is_array()))
}
#[test]

fn is_array_fails_and_includes_full_message() -> Result<()> {
    let result = verify_that!(json!({"key": "value"}), j::is_array());
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
    verify_that!(json!([]), j::is_empty_array())
}

#[test]
fn is_empty_array_rejects_non_empty() -> Result<()> {
    verify_that!(json!([1]), not(j::is_empty_array()))
}

#[test]
fn is_empty_array_fails_for_non_empty_array_and_includes_full_message() -> Result<()> {
    let result = verify_that!(json!([1]), j::is_empty_array());
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
    let result = verify_that!(json!(true), j::is_empty_array());
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
fn is_non_empty_array_matches_non_empty() -> Result<()> {
    verify_that!(json!([1]), j::is_non_empty_array())
}

#[test]
fn is_non_empty_array_rejects_empty() -> Result<()> {
    verify_that!(json!([]), not(j::is_non_empty_array()))
}

#[test]
fn is_non_empty_array_fails_for_empty_array_and_includes_full_message() -> Result<()> {
    let result = verify_that!(json!([]), j::is_non_empty_array());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!([])
            Expected: a non-empty JSON array
            Actual: Array [],
              which is an empty JSON array
            "#
        ))))
    )
}

#[test]
fn is_non_empty_array_fails_for_boolean_and_includes_full_message() -> Result<()> {
    let result = verify_that!(json!(true), j::is_non_empty_array());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!(true)
            Expected: a non-empty JSON array
            Actual: Bool(true),
              which is a JSON boolean
            "#
        ))))
    )
}

#[test]
fn each_is_string_matches_uniform_array() -> Result<()> {
    verify_that!(json!(["a", "b"]), j::each_is_string())
}

#[test]
fn each_is_number_rejects_mixed_array_and_reports_index() -> Result<()> {
    let result = verify_that!(json!([1, "b"]), j::each_is_number());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!([1, "b"])
            Expected: a JSON array whose elements are JSON number
            Actual: Array [Number(1), String("b")],
              which contains a JSON string at index 1
            "#
        ))))
    )
}

#[test]
fn each_is_number_rejects_non_array() -> Result<()> {
    verify_that!(json!(null), not(j::each_is_number()))
}

#[test]
fn each_is_boolean_matches_uniform_array() -> Result<()> {
    verify_that!(json!([true, false]), j::each_is_boolean())
}

#[test]
fn each_is_boolean_rejects_mixed_array_and_reports_index() -> Result<()> {
    let result = verify_that!(json!([true, 1]), j::each_is_boolean());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!([true, 1])
            Expected: a JSON array whose elements are JSON boolean
            Actual: Array [Bool(true), Number(1)],
              which contains a JSON number at index 1
            "#
        ))))
    )
}

#[test]
fn each_is_null_matches_uniform_array() -> Result<()> {
    verify_that!(json!([null, null]), j::each_is_null())
}

#[test]
fn each_is_null_rejects_mixed_array_and_reports_index() -> Result<()> {
    let result = verify_that!(json!([null, true]), j::each_is_null());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!([null, true])
            Expected: a JSON array whose elements are JSON null
            Actual: Array [Null, Bool(true)],
              which contains a JSON boolean at index 1
            "#
        ))))
    )
}

#[test]
fn each_is_array_matches_uniform_array() -> Result<()> {
    verify_that!(json!([[1], [2]]), j::each_is_array())
}

#[test]
fn each_is_array_rejects_mixed_array_and_reports_index() -> Result<()> {
    let result = verify_that!(json!([[1], {"a": 1}]), j::each_is_array());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!([[1], {"a": 1}])
            Expected: a JSON array whose elements are JSON array
            Actual: Array [Array [Number(1)], Object {"a": Number(1)}],
              which contains a JSON object at index 1
            "#
        ))))
    )
}

#[test]
fn each_is_object_matches_uniform_array() -> Result<()> {
    verify_that!(json!([{"a": 1}, {"b": 2}]), j::each_is_object())
}

#[test]
fn each_is_object_rejects_mixed_array_and_reports_index() -> Result<()> {
    let result = verify_that!(json!([{"a": 1}, [1]]), j::each_is_object());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!([{"a": 1}, [1]])
            Expected: a JSON array whose elements are JSON object
            Actual: Array [Object {"a": Number(1)}, Array [Number(1)]],
              which contains a JSON array at index 1
            "#
        ))))
    )
}

#[test]
fn is_object_matches_object() -> Result<()> {
    verify_that!(json!({"key": "value"}), j::is_object())
}
#[test]
fn is_object_rejects_boolean() -> Result<()> {
    verify_that!(json!(false), not(j::is_object()))
}

#[test]
fn is_empty_object_matches_empty() -> Result<()> {
    verify_that!(json!({}), j::is_empty_object())
}

#[test]
fn is_empty_object_rejects_non_empty() -> Result<()> {
    verify_that!(json!({"a": 1}), not(j::is_empty_object()))
}

#[test]
fn is_empty_object_rejects_non_object() -> Result<()> {
    verify_that!(json!(["a"]), not(j::is_empty_object()))
}
#[test]
fn is_object_fails_and_includes_full_message() -> Result<()> {
    let result = verify_that!(json!(false), j::is_object());
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

#[test]
fn is_non_empty_object_matches_non_empty() -> Result<()> {
    verify_that!(json!({"a": 1}), j::is_non_empty_object())
}

#[test]
fn is_non_empty_object_rejects_empty() -> Result<()> {
    verify_that!(json!({}), not(j::is_non_empty_object()))
}

#[test]
fn is_non_empty_object_fails_for_empty_object_and_includes_full_message() -> Result<()> {
    let result = verify_that!(json!({}), j::is_non_empty_object());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!({})
            Expected: a non-empty JSON object
            Actual: Object {},
              which is an empty JSON object
            "#
        ))))
    )
}

#[test]
fn is_non_empty_object_fails_for_array_and_includes_full_message() -> Result<()> {
    let result = verify_that!(json!([]), j::is_non_empty_object());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!([])
            Expected: a non-empty JSON object
            Actual: Array [],
              which is a JSON array
            "#
        ))))
    )
}

#[test]
fn as_string_matches_and_explains() -> Result<()> {
    verify_that!(json!("hello"), j::as_string(starts_with("h")))?;
    let result = verify_that!(json!(42), j::as_string(anything()));
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!(42)
            Expected: is a JSON string which is anything
            Actual: Number(42),
              which is a JSON number
            "#
        ))))
    )
}

#[test]
fn as_i64_matches_and_explains() -> Result<()> {
    verify_that!(json!(42), j::as_i64(gt(40)))?;
    let result = verify_that!(json!("not a number"), j::as_i64(anything()));
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!("not a number")
            Expected: is a JSON number (i64) which is anything
            Actual: String("not a number"),
              which is a JSON string
            "#
        ))))
    )
}
