use googletest::prelude::*;
use googletest_json_serde::json as j;
use serde_json::json;

#[test]
fn subset_of_ignores_duplicates_in_actual_and_expected() -> Result<()> {
    verify_that!(json!([1, 1, 2]), j::subset_of![1, 2])?;
    verify_that!(json!([1, 2]), j::subset_of![1, 1, 2])
}

#[test]
fn superset_of_ignores_duplicates_in_actual_and_expected() -> Result<()> {
    verify_that!(json!([1, 2]), j::superset_of![1, 1, 2])?;
    verify_that!(json!([1, 1, 2]), j::superset_of![1, 2])
}

#[test]
fn existing_array_matchers_require_distinct_matches() -> Result<()> {
    verify_that!(json!([1, 1]), not(j::is_contained_in![1]))?;
    verify_that!(json!([1]), not(j::contains_each![1, 1]))
}

#[test]
fn set_matchers_match_mixed_types() -> Result<()> {
    let object = json!({"a": [1]});
    let actual = json!([{"a": [1]}, "1", null, true, true]);
    verify_that!(actual, j::subset_of![json!(null), true, "1", &object])?;
    verify_that!(actual, j::superset_of![json!(null), true, "1", object])
}

#[test]
fn set_matchers_do_not_match_different_values() -> Result<()> {
    verify_that!(json!([1]), not(j::subset_of!["1"]))?;
    verify_that!(json!([1]), not(j::superset_of![true]))?;
    verify_that!(json!([{"a": [2]}]), not(j::subset_of![json!({"a": [1]})]))
}

#[test]
fn subset_of_handles_empty_arrays() -> Result<()> {
    verify_that!(json!([]), j::subset_of![])?;
    verify_that!(json!([]), j::subset_of![1])?;
    verify_that!(json!([1]), not(j::subset_of![]))
}

#[test]
fn superset_of_handles_empty_arrays() -> Result<()> {
    verify_that!(json!([]), j::superset_of![])?;
    verify_that!(json!([1]), j::superset_of![])?;
    verify_that!(json!([]), not(j::superset_of![1]))
}

#[test]
fn set_matchers_do_not_match_non_arrays() -> Result<()> {
    for value in [json!(null), json!(1), json!("x"), json!({})] {
        verify_that!(value, not(j::subset_of![]))?;
        verify_that!(value, not(j::superset_of![]))?;
    }
    Ok(())
}

#[test]
fn subset_of_explains_wrong_type() -> Result<()> {
    let result = verify_that!(json!({}), j::subset_of![]);
    verify_that!(
        result,
        err(displays_as(contains_substring("which is a JSON object")))
    )
}

#[test]
fn subset_of_explains_unexpected_element() -> Result<()> {
    let matcher = j::subset_of![1];
    verify_that!(
        matcher.explain_match(&json!([1, 2])),
        displays_as(contains_substring("at #1"))
    )
}

#[test]
fn superset_of_explains_missing_element() -> Result<()> {
    let matcher = j::superset_of![2];
    verify_that!(
        matcher.explain_match(&json!([1])),
        displays_as(contains_substring("missing"))
    )
}

#[test]
fn set_matchers_compose_with_object_patterns() -> Result<()> {
    let expected = json!("x");
    verify_that!(
        json!({"subset": ["x", "x"], "superset": [1, "x", null]}),
        j::pat!({
            "subset": j::subset_of![&expected, 1],
            "superset": j::superset_of![expected, 1],
        })
    )
}

#[test]
fn superset_of_composes_with_each() -> Result<()> {
    verify_that!(json!([[1, 1], [1, 2]]), j::each!(j::superset_of![1]))
}

#[test]
fn set_matchers_support_trailing_commas_and_parentheses() -> Result<()> {
    verify_that!(json!([1, "x"]), j::subset_of!(1, "x",))?;
    verify_that!(json!([1, "x"]), j::superset_of![1, "x",])?;
    verify_that!(json!([]), j::subset_of![,])?;
    verify_that!(json!([]), j::superset_of![,])
}

#[test]
fn subset_of_treats_inner_array_as_one_value() -> Result<()> {
    verify_that!(json!([[1, 2]]), j::subset_of![[1, 2]])?;
    verify_that!(json!([[1, 2]]), j::subset_of!([1, 2]))?;
    verify_that!(json!([1, 2]), not(j::subset_of![[1, 2]]))
}

#[test]
fn superset_of_treats_inner_array_as_one_value() -> Result<()> {
    verify_that!(json!([[1, 2], 3]), j::superset_of![[1, 2]])?;
    verify_that!(json!([[1, 2], 3]), j::superset_of!([1, 2]))?;
    verify_that!(json!([1, 2]), not(j::superset_of![[1, 2]]))
}

#[test]
fn set_matcher_evaluates_each_expression_once() -> Result<()> {
    let mut calls = 0;
    let matcher = j::superset_of![
        {
            calls += 1;
            1
        },
        {
            calls += 1;
            "x"
        }
    ];
    verify_that!(calls, eq(2))?;
    verify_that!(json!([1, "x"]), matcher)
}

#[test]
fn set_matchers_convert_non_finite_numbers_to_null() -> Result<()> {
    verify_that!(json!([null]), j::subset_of![f64::NAN])?;
    verify_that!(json!([null]), j::superset_of![f64::INFINITY])
}

#[test]
#[should_panic(expected = "key must be a string")]
fn set_matchers_reject_values_that_cannot_be_serialized_as_json() {
    let value = std::collections::BTreeMap::from([(vec![1, 2], true)]);
    let _ = j::subset_of![value];
}

#[test]
fn native_set_matchers_work_after_extracting_an_array() -> Result<()> {
    let actual = json!([1, 1, "x"]);
    let one = json!(1);
    let text = json!("x");
    let array = actual.as_array().unwrap();
    verify_that!(array, subset_of([&one, &text]))?;
    verify_that!(array, superset_of([&one, &one, &text]))
}
