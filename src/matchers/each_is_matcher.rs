use crate::matchers::__internal_unstable_do_not_depend_on_these;
use crate::matchers::__internal_unstable_do_not_depend_on_these::JsonPredicateMatcher;
use googletest::description::Description;
use serde_json::Value;

fn describe_kind(value: &Value) -> &'static str {
    match value {
        Value::Null => "JSON null",
        Value::Bool(_) => "JSON boolean",
        Value::Number(_) => "JSON number",
        Value::String(_) => "JSON string",
        Value::Array(_) => "JSON array",
        Value::Object(_) => "JSON object",
    }
}

fn build_each_is_type(
    kind: &'static str,
    predicate: impl Fn(&Value) -> bool + Copy + 'static,
) -> JsonPredicateMatcher<impl Fn(&Value) -> bool, String, &'static str> {
    JsonPredicateMatcher::new(
        move |v| match v {
            Value::Array(a) => a.iter().all(predicate),
            _ => false,
        },
        format!("a JSON array whose elements are {}", kind),
        "which is not a JSON array",
    )
    .with_explain_fn(move |v| match v {
        Value::Array(a) => a
            .iter()
            .enumerate()
            .find(|(_, el)| !predicate(el))
            .map(|(idx, el)| {
                Description::new().text(format!(
                    "which contains a {} at index {}",
                    describe_kind(el),
                    idx
                ))
            })
            .unwrap_or_else(|| Description::new().text("which is an empty JSON array")),
        _ => __internal_unstable_do_not_depend_on_these::describe_json_type(v),
    })
}

/// Matches JSON arrays whose elements are all JSON strings.
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json;
/// # use serde_json::json as j;
/// assert_that!(j!( ["a", "b" ]), json::each_is_string());
/// assert_that!(j!([1, "b"]), not(json::each_is_string()));
/// ```
pub fn each_is_string() -> JsonPredicateMatcher<impl Fn(&Value) -> bool, String, &'static str> {
    build_each_is_type("JSON string", |v| v.is_string())
}

/// Matches JSON arrays whose elements are all JSON numbers.
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json;
/// # use serde_json::json as j;
/// assert_that!(j!([1, 2, 3]), json::each_is_number());
/// assert_that!(j!([1, "b"]), not(json::each_is_number()));
/// ```
pub fn each_is_number() -> JsonPredicateMatcher<impl Fn(&Value) -> bool, String, &'static str> {
    build_each_is_type("JSON number", |v| v.is_number())
}

/// Matches JSON arrays whose elements are all JSON booleans.
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json;
/// # use serde_json::json as j;
/// assert_that!(j!([true, false]), json::each_is_boolean());
/// assert_that!(j!([true, 1]), not(json::each_is_boolean()));
/// ```
pub fn each_is_boolean() -> JsonPredicateMatcher<impl Fn(&Value) -> bool, String, &'static str> {
    build_each_is_type("JSON boolean", |v| v.is_boolean())
}

/// Matches JSON arrays whose elements are all JSON null.
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json;
/// # use serde_json::json as j;
/// assert_that!(j!([null, null]), json::each_is_null());
/// assert_that!(j!([null, true]), not(json::each_is_null()));
/// ```
pub fn each_is_null() -> JsonPredicateMatcher<impl Fn(&Value) -> bool, String, &'static str> {
    build_each_is_type("JSON null", |v| v.is_null())
}

/// Matches JSON arrays whose elements are all JSON arrays.
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json;
/// # use serde_json::json as j;
/// assert_that!(j!([[1], [2]]), json::each_is_array());
/// assert_that!(j!([[1], {"a": 1}]), not(json::each_is_array()));
/// ```
pub fn each_is_array() -> JsonPredicateMatcher<impl Fn(&Value) -> bool, String, &'static str> {
    build_each_is_type("JSON array", |v| v.is_array())
}

/// Matches JSON arrays whose elements are all JSON objects.
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json;
/// # use serde_json::json as j;
/// assert_that!(j!([{ "a": 1 }, { "b": 2 }]), json::each_is_object());
/// assert_that!(j!([{ "a": 1 }, [1]]), not(json::each_is_object()));
/// ```
pub fn each_is_object() -> JsonPredicateMatcher<impl Fn(&Value) -> bool, String, &'static str> {
    build_each_is_type("JSON object", |v| v.is_object())
}
