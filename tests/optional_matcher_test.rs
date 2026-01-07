use googletest::prelude::*;
use googletest_json_serde::json as j;
use serde_json::json;

#[test]
fn optional_matches_when_field_exists_and_matches_inner() -> Result<()> {
    let val = json!({"name": "bill"});
    verify_that!(val, j::pat!({"name": j::optional!(starts_with("bill"))}))
}

#[test]
fn optional_matches_when_field_missing() -> Result<()> {
    let val = json!({});
    verify_that!(val, j::pat!({"field": j::optional!(starts_with("value"))}))
}

#[test]
fn optional_unmatches_when_field_exists_but_fails_inner() -> Result<()> {
    let val = json!({"field": "wrong"});
    verify_that!(
        val,
        not(j::pat!({"field": j::optional!(starts_with("value"))}))
    )
}

#[test]
fn optional_works_in_nested_object() -> Result<()> {
    let val = json!({"user": {"id": 1}});
    verify_that!(
        val,
        j::pat!({
            "user": j::pat!({
                "id": eq(1),
                "nickname": j::optional!(starts_with("Bob"))
            })
        })
    )
}

#[test]
fn optional_explain_match_when_field_present_and_mismatch() -> Result<()> {
    let matcher = j::pat!({
        "user": j::pat!({
            "nickname": j::optional!(starts_with("Bob"))
        })
    });
    let val = json!({"user": {"nickname": "Alice"}});
    let result = verify_that!(val, matcher);
    verify_that!(
        result,
        err(displays_as(contains_substring(
            "field 'nickname': which does not start with \"Bob\""
        )))
    )
}

#[test]
fn optional_explain_match_when_field_missing_should_not_error() -> Result<()> {
    let val = json!({});
    verify_that!(
        val,
        j::pat!({
            "nickname": j::optional!(starts_with("Bob"))
        })
    )
}

#[test]
fn optional_allows_nested_pat_inside_optional() -> Result<()> {
    let val = json!({});
    verify_that!(
        val,
        j::pat!({
            "profile": j::optional!(j::pat!({
                "name": starts_with("Alice"),
                "age": eq(30)
            }))
        })
    )
}

#[test]
fn optional_nested_pat_matches_when_present_and_inner_matches() -> Result<()> {
    let val = json!({"profile": {"name": "Alice", "age": 30}});
    verify_that!(
        val,
        j::pat!({
            "profile": j::optional!(j::pat!({
                "name": starts_with("Alice"),
                "age": eq(30)
            }))
        })
    )
}

#[test]
fn optional_nested_pat_fails_when_inner_mismatch() -> Result<()> {
    let val = json!({"profile": {"name": "Bob", "age": 30}});
    verify_that!(
        val,
        not(j::pat!({
            "profile": j::optional!(j::pat!({
                "name": starts_with("Alice"),
                "age": eq(30)
            }))
        }))
    )
}

#[test]
fn pat_inside_optional_inside_pat_allows_missing_subobject() -> Result<()> {
    let val = json!({"user": {}});
    verify_that!(
        val,
        j::pat!({
            "user": j::pat!({
                "profile": j::optional!(
                    j::pat!({
                    "nickname": starts_with("Bob")
                }))
            })
        })
    )
}

#[test]
fn pat_inside_optional_inside_pat_matches_when_nested_object_present_and_correct() -> Result<()> {
    let val = json!({"user": {"profile": {"nickname": "Bob"}}});
    verify_that!(
        val,
        j::pat!({
            "user": j::pat!({
                "profile": j::optional!(j::pat!({
                    "nickname": starts_with("Bob")
                }))
            })
        })
    )
}

#[test]
fn optional_array_field_matches_when_present_and_all_elements_match() -> Result<()> {
    let val = json!({"items": ["apple", "banana", "carrot"]});
    verify_that!(
        val,
        j::pat!({
            "items": j::optional!(j::elements_are![starts_with("ap"), starts_with("ban"), starts_with("car")])
        })
    )
}

#[test]
fn optional_array_field_matches_when_missing() -> Result<()> {
    let val = json!({});
    verify_that!(
        val,
        j::pat!({
            "tags": j::optional!(j::elements_are![starts_with("x"), starts_with("y")])
        })
    )
}

#[test]
fn optional_array_field_fails_when_element_mismatch() -> Result<()> {
    let val = json!({"items": ["apple", "banana", "not_carrot"]});
    verify_that!(
        val,
        not(j::pat!({
            "items": j::optional!(j::elements_are![starts_with("ap"), starts_with("ban"), starts_with("car")])
        }))
    )
}

#[test]
fn optional_matches_when_field_is_null() -> Result<()> {
    let val = json!({"field": null});
    verify_that!(val, j::pat!({"field": j::optional!(starts_with("Bob"))}))
}

#[test]
fn optional_does_not_trigger_strict_mode_violation_when_missing() -> Result<()> {
    let val = json!({});
    verify_that!(
        val,
        j::pat!({
            "field": j::optional!(eq(1))
        })
    )
}

#[test]
fn deeply_nested_optional_inside_multiple_pats() -> Result<()> {
    let val = json!({"a": {"b": {"c": "Bob"}}});
    verify_that!(
        val,
        j::pat!({
            "a": j::pat!({
                "b": j::pat!({
                    "c": j::optional!(starts_with("Bob"))
                })
            })
        })
    )
}

#[test]
fn optional_with_primitive_matcher() -> Result<()> {
    let val = json!({"flag": true});
    verify_that!(
        val,
        j::pat!({"flag": j::optional!(j::primitive!(is_true()))})
    )
}

#[test]
fn optional_with_primitive_matcher_fails_when_value_mismatch() -> Result<()> {
    let val = json!({"flag": false});
    verify_that!(
        val,
        not(j::pat!({"flag": j::optional!(j::primitive!(is_true()))}))
    )
}

#[test]
fn optional_with_primitive_matcher_allows_missing_field() -> Result<()> {
    let val = json!({});
    verify_that!(
        val,
        j::pat!({"flag": j::optional!(j::primitive!(is_true()))})
    )
}
