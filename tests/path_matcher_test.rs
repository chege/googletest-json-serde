use googletest::Result;
use googletest::prelude::*;
use googletest_json_serde::json;
use serde_json::json as j;

#[test]
fn has_paths_matches_superset() -> Result<()> {
    verify_that!(
        j!({"id": 1, "name": "Alice", "extra": true}),
        json::has_paths(&["id", "name"])
    )
}

#[test]
fn has_paths_rejects_missing_path() -> Result<()> {
    let result = verify_that!(j!({"id": 1}), json::has_paths(&["id", "name"]));
    verify_that!(
        result,
        err(displays_as(contains_substring("missing paths [\"name\"]")))
    )
}

#[test]
fn has_paths_rejects_non_object() -> Result<()> {
    verify_that!(j!(null), not(json::has_paths(&["id"])))
}

#[test]
fn has_paths_supports_nested_paths() -> Result<()> {
    verify_that!(
        j!({"user": {"id": 1, "profile": {"active": true}}}),
        json::has_paths(&["user.id", "user.profile.active"])
    )
}

#[test]
fn has_paths_supports_array_indices() -> Result<()> {
    verify_that!(
        j!({"items": [{"id": 1}, {"id": 2}]}),
        json::has_paths(&["items.0.id", "items.1.id"])
    )
}

#[test]
fn has_only_paths_matches_exact_set() -> Result<()> {
    verify_that!(
        j!({"id": 1, "name": "Alice"}),
        json::has_only_paths(&["name", "id"])
    )
}

#[test]
fn has_only_paths_rejects_missing_path() -> Result<()> {
    let result = verify_that!(j!({"id": 1}), json::has_only_paths(&["id", "name"]));
    verify_that!(
        result,
        err(displays_as(contains_substring("missing paths [\"name\"]")))
    )
}

#[test]
fn has_only_paths_rejects_extra_path() -> Result<()> {
    let result = verify_that!(
        j!({"id": 1, "name": "Alice", "extra": true}),
        json::has_only_paths(&["id", "name"])
    );
    verify_that!(
        result,
        err(displays_as(contains_substring("extra paths [\"extra\"]")))
    )
}

#[test]
fn has_only_paths_supports_nested_paths() -> Result<()> {
    verify_that!(
        j!({"user": {"id": 1}}),
        json::has_only_paths(&["user", "user.id"])
    )
}

#[test]
fn has_only_paths_rejects_missing_nested_leaf() -> Result<()> {
    let result = verify_that!(
        j!({"user": {"id": 1}}),
        json::has_only_paths(&["user", "user.id", "user.name"])
    );
    verify_that!(
        result,
        err(displays_as(contains_substring(
            "missing paths [\"user.name\"]"
        )))
    )
}
