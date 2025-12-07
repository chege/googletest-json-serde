use googletest::prelude::*;
use googletest_json_serde::json;
use indoc::indoc;
use serde_json::json;

#[test]
fn each_is_string_matches_uniform_array() -> Result<()> {
    verify_that!(json!(["a", "b"]), json::each_is_string())
}

#[test]
fn each_is_number_rejects_mixed_array_and_reports_index() -> Result<()> {
    let result = verify_that!(json!([1, "b"]), json::each_is_number());
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
    verify_that!(json!(null), not(json::each_is_number()))
}

#[test]
fn each_is_boolean_matches_uniform_array() -> Result<()> {
    verify_that!(json!([true, false]), json::each_is_boolean())
}

#[test]
fn each_is_boolean_rejects_mixed_array_and_reports_index() -> Result<()> {
    let result = verify_that!(json!([true, 1]), json::each_is_boolean());
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
    verify_that!(json!([null, null]), json::each_is_null())
}

#[test]
fn each_is_null_rejects_mixed_array_and_reports_index() -> Result<()> {
    let result = verify_that!(json!([null, true]), json::each_is_null());
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
    verify_that!(json!([[1], [2]]), json::each_is_array())
}

#[test]
fn each_is_array_rejects_mixed_array_and_reports_index() -> Result<()> {
    let result = verify_that!(json!([[1], {"a": 1}]), json::each_is_array());
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
    verify_that!(json!([{ "a": 1 }, { "b": 2 }]), json::each_is_object())
}

#[test]
fn each_is_object_rejects_mixed_array_and_reports_index() -> Result<()> {
    let result = verify_that!(json!([{ "a": 1 }, [1]]), json::each_is_object());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            r#"
            Value of: json!([{ "a": 1 }, [1]])
            Expected: a JSON array whose elements are JSON object
            Actual: Array [Object {"a": Number(1)}, Array [Number(1)]],
              which contains a JSON array at index 1
            "#
        ))))
    )
}
