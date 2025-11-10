use googletest::prelude::*;
use googletest_json_serde::json;
use serde_json::json as j;

#[test]
fn optional_matches_when_field_exists_and_matches_inner() -> Result<()> {
    let val = j!({"name": "bill"});
    verify_that!(
        val,
        json::pat!({"name": json::optional!(starts_with("bill"))})
    )
}

#[test]
fn optional_matches_when_field_missing() -> Result<()> {
    let val = j!({});
    verify_that!(
        val,
        json::pat!({"field": json::optional!(starts_with("value"))})
    )
}

#[test]
fn optional_unmatches_when_field_exists_but_fails_inner() -> Result<()> {
    let val = j!({"field": "wrong"});
    verify_that!(
        val,
        not(json::pat!({"field": json::optional!(starts_with("value"))}))
    )
}

#[test]
fn optional_works_in_nested_object() -> Result<()> {
    let val = j!({"user": {"id": 1}});
    verify_that!(
        val,
        json::pat!({
            "user": json::pat!({
                "id": eq(1),
                "nickname": json::optional!(starts_with("Bob"))
            })
        })
    )
}

#[test]
fn optional_explain_match_when_field_present_and_mismatch() -> Result<()> {
    let matcher = json::pat!({
        "user": json::pat!({
            "nickname": json::optional!(starts_with("Bob"))
        })
    });
    let val = j!({"user": {"nickname": "Alice"}});
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
    let val = j!({});
    verify_that!(
        val,
        json::pat!({
            "nickname": json::optional!(starts_with("Bob"))
        })
    )
}

#[test]
fn optional_allows_nested_pat_inside_optional() -> Result<()> {
    let val = j!({});
    verify_that!(
        val,
        json::pat!({
            "profile": json::optional!(json::pat!({
                "name": starts_with("Alice"),
                "age": eq(30)
            }))
        })
    )
}

#[test]
fn optional_nested_pat_matches_when_present_and_inner_matches() -> Result<()> {
    let val = j!({"profile": {"name": "Alice", "age": 30}});
    verify_that!(
        val,
        json::pat!({
            "profile": json::optional!(json::pat!({
                "name": starts_with("Alice"),
                "age": eq(30)
            }))
        })
    )
}

#[test]
fn optional_nested_pat_fails_when_inner_mismatch() -> Result<()> {
    let val = j!({"profile": {"name": "Bob", "age": 30}});
    verify_that!(
        val,
        not(json::pat!({
            "profile": json::optional!(json::pat!({
                "name": starts_with("Alice"),
                "age": eq(30)
            }))
        }))
    )
}

#[test]
fn pat_inside_optional_inside_pat_allows_missing_subobject() -> Result<()> {
    let val = j!({"user": {}});
    verify_that!(
        val,
        json::pat!({
            "user": json::pat!({
                "profile": json::optional!(
                    json::pat!({
                    "nickname": starts_with("Bob")
                }))
            })
        })
    )
}

#[test]
fn pat_inside_optional_inside_pat_matches_when_nested_object_present_and_correct() -> Result<()> {
    let val = j!({"user": {"profile": {"nickname": "Bob"}}});
    verify_that!(
        val,
        json::pat!({
            "user": json::pat!({
                "profile": json::optional!(json::pat!({
                    "nickname": starts_with("Bob")
                }))
            })
        })
    )
}

#[test]
fn optional_array_field_matches_when_present_and_all_elements_match() -> Result<()> {
    let val = j!({"items": ["apple", "banana", "carrot"]});
    verify_that!(
        val,
        json::pat!({
            "items": json::optional!(json::elements_are![starts_with("ap"), starts_with("ban"), starts_with("car")])
        })
    )
}

#[test]
fn optional_array_field_matches_when_missing() -> Result<()> {
    let val = j!({});
    verify_that!(
        val,
        json::pat!({
            "tags": json::optional!(json::elements_are![starts_with("x"), starts_with("y")])
        })
    )
}

#[test]
fn optional_array_field_fails_when_element_mismatch() -> Result<()> {
    let val = j!({"items": ["apple", "banana", "not_carrot"]});
    verify_that!(
        val,
        not(json::pat!({
            "items": json::optional!(json::elements_are![starts_with("ap"), starts_with("ban"), starts_with("car")])
        }))
    )
}

#[test]
fn optional_matches_when_field_is_null() -> Result<()> {
    let val = j!({"field": null});
    verify_that!(
        val,
        json::pat!({"field": json::optional!(starts_with("Bob"))})
    )
}

#[test]
fn optional_does_not_trigger_strict_mode_violation_when_missing() -> Result<()> {
    let val = j!({});
    verify_that!(
        val,
        json::pat!({
            "field": json::optional!(eq(1))
        })
    )
}

#[test]
fn deeply_nested_optional_inside_multiple_pats() -> Result<()> {
    let val = j!({"a": {"b": {"c": "Bob"}}});
    verify_that!(
        val,
        json::pat!({
            "a": json::pat!({
                "b": json::pat!({
                    "c": json::optional!(starts_with("Bob"))
                })
            })
        })
    )
}

#[test]
fn optional_with_primitive_matcher() -> Result<()> {
    let val = j!({"flag": true});
    verify_that!(
        val,
        json::pat!({"flag": json::optional!(json::primitive!(is_true()))})
    )
}

#[test]
fn optional_with_primitive_matcher_fails_when_value_mismatch() -> Result<()> {
    let val = j!({"flag": false});
    verify_that!(
        val,
        not(json::pat!({"flag": json::optional!(json::primitive!(is_true()))}))
    )
}

#[test]
fn optional_with_primitive_matcher_allows_missing_field() -> Result<()> {
    let val = j!({});
    verify_that!(
        val,
        json::pat!({"flag": json::optional!(json::primitive!(is_true()))})
    )
}
