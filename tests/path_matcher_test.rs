use googletest::Result;
use googletest::prelude::*;
use googletest_json_serde::json as j;
use serde_json::json;

#[test]
fn has_paths_matches_superset() -> Result<()> {
    verify_that!(
        json!({"id": 1, "name": "Alice", "extra": true}),
        j::has_paths(&["id", "name"])
    )
}

#[test]
fn has_paths_rejects_missing_path() -> Result<()> {
    let result = verify_that!(json!({"id": 1}), j::has_paths(&["id", "name"]));
    verify_that!(
        result,
        err(displays_as(contains_substring("missing paths [\"name\"]")))
    )
}

#[test]
fn has_paths_rejects_non_object() -> Result<()> {
    verify_that!(json!(null), not(j::has_paths(&["id"])))
}

#[test]
fn has_paths_supports_nested_paths() -> Result<()> {
    verify_that!(
        json!({"user": {"id": 1, "profile": {"active": true}}}),
        j::has_paths(&["user.id", "user.profile.active"])
    )
}

#[test]
fn has_paths_supports_array_indices() -> Result<()> {
    verify_that!(
        json!({"items": [{"id": 1}, {"id": 2}]}),
        j::has_paths(&["items.0.id", "items.1.id"])
    )
}

#[test]
fn has_only_paths_matches_exact_set() -> Result<()> {
    verify_that!(
        json!({"id": 1, "name": "Alice"}),
        j::has_only_paths(&["name", "id"])
    )
}

#[test]
fn has_only_paths_rejects_missing_path() -> Result<()> {
    let result = verify_that!(json!({"id": 1}), j::has_only_paths(&["id", "name"]));
    verify_that!(
        result,
        err(displays_as(contains_substring("missing paths [\"name\"]")))
    )
}

#[test]
fn has_only_paths_rejects_extra_path() -> Result<()> {
    let result = verify_that!(
        json!({"id": 1, "name": "Alice", "extra": true}),
        j::has_only_paths(&["id", "name"])
    );
    verify_that!(
        result,
        err(displays_as(contains_substring("extra paths [\"extra\"]")))
    )
}

#[test]
fn has_only_paths_supports_nested_paths() -> Result<()> {
    verify_that!(
        json!({"user": {"id": 1}}),
        j::has_only_paths(&["user", "user.id"])
    )
}

#[test]
fn has_only_paths_rejects_missing_nested_leaf() -> Result<()> {
    let result = verify_that!(
        json!({"user": {"id": 1}}),
        j::has_only_paths(&["user", "user.id", "user.name"])
    );
    verify_that!(
        result,
        err(displays_as(contains_substring(
            "missing paths [\"user.name\"]"
        )))
    )
}
