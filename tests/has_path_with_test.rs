use googletest::prelude::*;
use googletest_json_serde::json as j;
use serde_json::json;

#[test]
fn has_path_with_matches_literal() -> Result<()> {
    let value = json!({"user": {"id": 7}});
    verify_that!(value, j::has_path_with!("user.id", eq(7)))
}

#[test]
fn has_path_with_matches_value() -> Result<()> {
    let value = json!({"user": {"meta": json!({"active": true})}});
    verify_that!(
        value,
        j::has_path_with!("user.meta", json!({"active": true}))
    )
}

#[test]
fn has_path_with_matches_with_matcher() -> Result<()> {
    let value = json!({"user": {"name": "Ada"}});
    verify_that!(value, j::has_path_with!("user.name", starts_with("A")))
}

#[test]
fn has_path_with_matches_with_borrowed_value() -> Result<()> {
    let value = json!({"user": {"id": 7}});
    let expected = json!(7);
    verify_that!(value, j::has_path_with!("user.id", &expected))
}

#[test]
fn has_path_with_matches_with_inline_borrowed_literal() -> Result<()> {
    verify_that!(
        json!({"user": {"active": true}}),
        j::has_path_with!("user.active", &json!(true))
    )
}

#[test]
fn has_path_with_matches_with_borrowed_path_and_value() -> Result<()> {
    let path = String::from("user.name");
    let name = json!("Ada");
    verify_that!(
        json!({"user": {"name": "Ada"}}),
        j::has_path_with!(&path, &name)
    )
}

#[test]
fn has_path_with_fails_on_missing_path() -> Result<()> {
    let value = json!({"user": {"name": "Ada"}});
    verify_that!(value, not(j::has_path_with!("user.id", eq(1))))
}

#[test]
fn has_path_with_fails_on_mismatch() -> Result<()> {
    let value = json!({"user": {"id": 7}});
    verify_that!(value, not(j::has_path_with!("user.id", eq("seven"))))
}

#[test]
fn has_path_with_fails_on_mismatch_with_borrowed_value() -> Result<()> {
    let value = json!({"user": {"id": 7}});
    let wrong = json!(8);
    verify_that!(value, not(j::has_path_with!("user.id", &wrong)))
}

#[test]
fn has_path_with_supports_escaped_dot_paths() -> Result<()> {
    let value = json!({"user.name": {"id": 1}});
    verify_that!(value, j::has_path_with!(r"user\.name.id", eq(1)))
}

#[test]
fn has_path_with_handles_nested_array_indices() -> Result<()> {
    let value = json!({"items": [ {"id": 1}, {"id": 2, "name": "two"} ]});
    verify_that!(value, j::has_path_with!("items.1.name", eq("two")))
}
