use googletest::Result;
use googletest::prelude::*;
use googletest_json_serde::json as j;
use serde_json::json;

#[test]
fn composed_matchers_nested_object_array_optional_smoke_test() -> Result<()> {
    let value = json!({
        "user": {
            "name": "Alice",
            "age": 30,
            "nickname": null,
            "roles": ["admin", "staff"]
        },
        "items": [
            {"id": 1, "kind": "book"},
            {"id": 2, "kind": "pen"}
        ]
    });

    verify_that!(
        value,
        j::pat!({
            "user": j::pat!({
                "name": j::as_string(starts_with("Ali")),
                "age": j::as_i64(ge(18)),
                "nickname": j::optional!(starts_with("A")),
                "roles": j::elements_are![starts_with("ad"), starts_with("st")]
            }),
            "items": j::unordered_elements_are![
                j::pat!({"id": eq(2), "kind": eq("pen")}),
                j::pat!({"id": eq(1), "kind": eq("book")})
            ]
        })
    )
}

#[test]
fn pat_with_each_embedded_pat_matches() -> Result<()> {
    let value = json!({
        "items": [
            {"id": 1, "kind": "book"},
            {"id": 2, "kind": "pen"}
        ]
    });

    verify_that!(
        value,
        j::pat!({
            "items": j::each!(j::pat!({
                "id": j::as_i64(gt(0)),
                "kind": j::as_string(anything())
            }))
        })
    )
}

#[test]
fn pat_with_each_embedded_pat_unmatches() -> Result<()> {
    let value = json!({
        "items": [
            {"id": 1, "kind": "book"},
            {"id": -2, "kind": "pen"}
        ]
    });

    verify_that!(
        value,
        not(j::pat!({
            "items": j::each!(j::pat!({
                "id": j::as_i64(gt(0)),
                "kind": j::as_string(anything())
            }))
        }))
    )
}

#[test]
fn pat_with_contains_each_embedded_pat_matches() -> Result<()> {
    let value = json!({
        "items": [
            {"id": 1, "kind": "book"},
            {"id": 2, "kind": "pen"},
            {"id": 3, "kind": "notebook"}
        ]
    });

    verify_that!(
        value,
        j::pat!({
            "items": j::contains_each![
                j::pat!({"kind": eq("book"), ..}),
                j::pat!({"kind": starts_with("note"), ..})
            ]
        })
    )
}

#[test]
fn pat_with_is_contained_in_embedded_pat_matches() -> Result<()> {
    let value = json!({
        "items": [
            {"id": 1, "kind": "book"},
            {"id": 2, "kind": "pen"}
        ]
    });

    verify_that!(
        value,
        j::pat!({
            "items": j::is_contained_in![
                j::pat!({"kind": eq("book"), ..}),
                j::pat!({"kind": eq("pen"), ..}),
                j::pat!({"kind": eq("eraser"), ..})
            ]
        })
    )
}

#[test]
fn pat_with_elements_are_embedded_pat_matches() -> Result<()> {
    let value = json!({
        "items": [
            {"id": 1, "kind": "book"},
            {"id": 2, "kind": "pen"}
        ]
    });

    verify_that!(
        value,
        j::pat!({
            "items": j::elements_are![
                j::pat!({"id": eq(1), "kind": eq("book")}),
                j::pat!({"id": eq(2), "kind": eq("pen")})
            ]
        })
    )
}

#[test]
fn pat_with_as_adapters_matches() -> Result<()> {
    let value = json!({
        "name": "Alice",
        "active": true,
        "user": {"id": "123"},
        "tags": ["admin", "staff"]
    });

    verify_that!(
        value,
        j::pat!({
            "name": j::as_string(starts_with("A")),
            "active": j::as_bool(eq(true)),
            "user": j::as_object(len(eq(1))),
            "tags": j::as_array(contains(j::as_string(eq("admin"))))
        })
    )
}

#[test]
fn unordered_elements_are_with_embedded_pat_matches() -> Result<()> {
    let value = json!([
        {"id": 1, "kind": "book"},
        {"id": 2, "kind": "pen"}
    ]);

    verify_that!(
        value,
        j::unordered_elements_are![
            j::pat!({"kind": eq("pen"), ..}),
            j::pat!({"kind": eq("book"), ..})
        ]
    )
}

#[test]
fn contains_each_with_embedded_pat_unmatches() -> Result<()> {
    let value = json!([
        {"id": 1, "kind": "book"},
        {"id": 2, "kind": "pen"}
    ]);

    verify_that!(
        value,
        not(j::contains_each![
            j::pat!({"kind": eq("book")}),
            j::pat!({"kind": eq("not-there")})
        ])
    )
}

#[test]
fn is_contained_in_with_embedded_pat_unmatches() -> Result<()> {
    let value = json!([
        {"id": 1, "kind": "book"},
        {"id": 2, "kind": "pen"},
        {"id": 3, "kind": "eraser"}
    ]);

    verify_that!(
        value,
        not(j::is_contained_in![
            j::pat!({"kind": eq("book")}),
            j::pat!({"kind": eq("pen")})
        ])
    )
}

#[test]
fn has_path_with_embedded_pat_matches() -> Result<()> {
    let value = json!({
        "user": {
            "profile": {
                "name": "Alice",
                "active": true
            }
        }
    });

    verify_that!(
        value,
        j::has_path_with!(
            "user.profile",
            j::pat!({
                "name": starts_with("Ali"),
                "active": eq(true)
            })
        )
    )
}
