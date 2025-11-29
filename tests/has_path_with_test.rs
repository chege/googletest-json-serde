use googletest::prelude::*;
use googletest_json_serde::json;
use serde_json::json as j;

#[test]
fn has_path_with_matches_literal() -> Result<()> {
    let value = j!({"user": {"id": 7}});
    verify_that!(value, json::has_path_with!("user.id", eq(7)))
}

#[test]
fn has_path_with_matches_value() -> Result<()> {
    let value = j!({"user": {"meta": j!({"active": true})}});
    verify_that!(
        value,
        json::has_path_with!("user.meta", j!({"active": true}))
    )
}

#[test]
fn has_path_with_matches_with_matcher() -> Result<()> {
    let value = j!({"user": {"name": "Ada"}});
    verify_that!(value, json::has_path_with!("user.name", starts_with("A")))
}

#[test]
fn has_path_with_matches_with_borrowed_value() -> Result<()> {
    let value = j!({"user": {"id": 7}});
    let expected = j!(7);
    verify_that!(value, json::has_path_with!("user.id", &expected))
}

#[test]
fn has_path_with_matches_with_inline_borrowed_literal() -> Result<()> {
    verify_that!(
        j!({"user": {"active": true}}),
        json::has_path_with!("user.active", &j!(true))
    )
}

#[test]
fn has_path_with_matches_with_borrowed_path_and_value() -> Result<()> {
    let path = String::from("user.name");
    let name = j!("Ada");
    verify_that!(
        j!({"user": {"name": "Ada"}}),
        json::has_path_with!(&path, &name)
    )
}

#[test]
fn has_path_with_fails_on_missing_path() -> Result<()> {
    let value = j!({"user": {"name": "Ada"}});
    verify_that!(value, not(json::has_path_with!("user.id", eq(1))))
}

#[test]
fn has_path_with_fails_on_mismatch() -> Result<()> {
    let value = j!({"user": {"id": 7}});
    verify_that!(value, not(json::has_path_with!("user.id", eq("seven"))))
}

#[test]
fn has_path_with_fails_on_mismatch_with_borrowed_value() -> Result<()> {
    let value = j!({"user": {"id": 7}});
    let wrong = j!(8);
    verify_that!(value, not(json::has_path_with!("user.id", &wrong)))
}

#[test]
fn has_path_with_supports_escaped_dot_paths() -> Result<()> {
    let value = j!({"user.name": {"id": 1}});
    verify_that!(value, json::has_path_with!(r"user\.name.id", eq(1)))
}

#[test]
fn has_path_with_handles_nested_array_indices() -> Result<()> {
    let value = j!({"items": [ {"id": 1}, {"id": 2, "name": "two"} ]});
    verify_that!(value, json::has_path_with!("items.1.name", eq("two")))
}
