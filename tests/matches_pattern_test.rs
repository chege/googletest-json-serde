use googletest::prelude::*;
use googletest_json_serde::json;
use indoc::indoc;
use serde_json::json as j;

#[test]
fn match_nested_object_strict() {
    let val = j!({
        "user": {
            "id": 1,
            "name": "Alice"
        },
        "active": true
    });

    assert_that!(
        val,
        json::pat!({
            "user": json::pat!({
                "id": eq(1),
                "name": starts_with("Alic"),
            }),
            "active": j!(true),
        })
    );
}

#[test]
fn match_nested_object_non_strict() {
    let val = j!({
        "user": {
            "id": 1,
            "name": "Alice",
            "extra": "data"
        }
    });

    assert_that!(
        val,
        json::pat!({
            "user": json::pat!({
                "id": ge(1),
                ..
            })
        })
    );
}

#[test]
fn match_enum_like_object_strict_mismatch() {
    let val = j!({
        "type": "Dog",
        "bark": true
    });

    assert_that!(
        val,
        not(json::pat!({
            "type": eq("Cat"),
            "meow": is_true()
        }))
    );
}

#[test]
fn match_option_nested_mixed_matchers() {
    let val = Some(j!({
        "type": "Dog",
        "props": {
            "bark": true,
            "age": 5
        }
    }));

    assert_that!(
        val,
        json::pat!({
            "type": eq("Dog"),
            "props": json::pat!({
                "bark": eq(true),
                "age": gt(2),
            }),
        })
    );
}

#[test]
fn fail_on_unexpected_fields_strict() {
    let val = j!({
        "a": 1,
        "b": 2,
        "unexpected": 3
    });

    assert_that!(
        val,
        not(json::pat!({
            "a": eq(1),
            "b": eq(2),
        }))
    );
}
#[test]
fn match_object_with_any_value_field() {
    let val = j!({"field": "value", "unexpected": 123});
    assert_that!(
        val,
        json::pat!({
            "field": eq("value"),
            "unexpected": json::is_not_null()
        })
    );
}

#[test]
fn match_option_none() {
    let val: Option<serde_json::Value> = None;
    assert_that!(
        val,
        not(json::pat!({
            "field": eq("value")
        }))
    );
}

#[test]
fn match_object_with_wrong_field() {
    let val = j!({"field": "other"});
    assert_that!(
        val,
        not(json::pat!({
            "field": eq("value")
        }))
    );
}

#[test]
fn explain_mismatch_nested_object() {
    let val = j!({
        "field": {
            "subfield": 123,
            "flag": false
        },
        "extra": "hello"
    });

    if let Err(err) = verify_that!(
        val,
        json::pat!({
            "field": json::pat!({
                "subfield": eq(999),
                "flag": eq(true)
            }),
            "extra": eq("world")
        })
    ) {
        assert_that!(
            err.description,
            all![
                contains_substring("field 'field': had 2 field mismatches"),
                contains_substring("field 'subfield': which isn't equal to 999"),
                contains_substring("field 'flag': which isn't equal to true"),
                contains_substring("field 'extra': which isn't equal to \"world\""),
            ]
        );
    } else {
        panic!("expected failure but matcher reported success");
    }
}

#[test]
fn explain_single_field_mismatch() {
    let val = j!({"foo": 1});
    if let Err(err) = verify_that!(
        val,
        json::pat!({
            "foo": eq(2)
        })
    ) {
        assert_that!(
            err.description,
            contains_substring("field 'foo': which isn't equal to 2")
        );
    } else {
        panic!("expected failure but matcher reported success");
    }
}

#[test]
fn explain_wrong_type() {
    let val = j!(123);
    if let Err(err) = verify_that!(
        val,
        json::pat!({
            "foo": eq(1)
        })
    ) {
        assert_that!(
            err.description,
            contains_substring("was 123 (expected object)")
        );
    } else {
        panic!("expected failure but matcher reported success");
    }
}

#[test]
fn explain_option_none() {
    let val: Option<serde_json::Value> = None;
    if let Err(err) = verify_that!(
        val,
        json::pat!({
            "foo": eq(1)
        })
    ) {
        assert_that!(err.description, contains_substring("was None"));
    } else {
        panic!("expected failure but matcher reported success");
    }
}

#[test]
fn explain_option_some_mismatch() {
    let val = Some(j!({"foo": 1}));
    if let Err(err) = verify_that!(
        val,
        json::pat!({
            "foo": eq(2)
        })
    ) {
        assert_that!(
            err.description,
            contains_substring("field 'foo': which isn't equal to 2")
        );
    } else {
        panic!("expected failure but matcher reported success");
    }
}

#[test]
fn matches_pattern_produces_correct_failure_message() -> Result<()> {
    let result = verify_that!(
        j!({
            "user": { "id": 1, "name": "Alice" },
            "active": true
        }),
        json::pat!({
            "user": json::pat!({
                "id": eq(2),
                "name": eq("Bob"),
            }),
            "active": eq(false),
        })
    );
    verify_that!(
        result,
        err(displays_as(starts_with(indoc!(
            r#"
                Value of: j!({ "user": { "id": 1, "name": "Alice" }, "active": true })
                Expected: has JSON object with expected fields
                Actual: Object {
                    "active": Bool(true),
                    "user": Object {
                        "id": Number(1),
                        "name": String("Alice"),
                    },
                },
                  had 2 field mismatches:
                    field 'user': had 2 field mismatches:
                    field 'id': which isn't equal to 2
                    field 'name': which isn't equal to "Bob"
                    field 'active': which isn't equal to false"#
        ))))
    )
}
#[test]
fn pat_matches_mixed_types_with_owned_values() -> Result<()> {
    let value = j!({"a": "x", "b": 1, "c": true});
    let a = j!("x");
    let b = j!(1);
    let c = j!(true);
    verify_that!(value, json::pat!({"a": a, "b": b, "c": c}))
}

#[test]
fn pat_matches_mixed_types_with_borrowed_values() -> Result<()> {
    let value = j!({"a": "x", "b": 1, "c": true});
    let a = j!("x");
    let b = j!(1);
    let c = j!(true);
    verify_that!(value, json::pat!({"a": &a, "b": &b, "c": &c}))
}

#[test]
fn pat_matches_mixed_types_with_inline_borrowed_literals() -> Result<()> {
    verify_that!(
        j!({"a": "x", "b": 1, "c": true}),
        json::pat!({"a": &j!("x"), "b": &j!(1), "c": &j!(true)})
    )
}

#[test]
fn pat_matches_mixed_types_with_inline_owned_literals() -> Result<()> {
    verify_that!(
        j!({"a": "x", "b": 1, "c": true}),
        json::pat!({"a": j!("x"), "b": j!(1), "c": j!(true)})
    )
}

#[test]
fn pat_matches_mixed_types_with_mixed_owned_and_borrowed() -> Result<()> {
    let value = j!({"a": "x", "b": 1, "c": true});
    let a = j!("x");
    verify_that!(value, json::pat!({"a": a, "b": j!(1), "c": &j!(true)}))
}

#[test]
fn pat_unmatch_with_owned_values() -> Result<()> {
    let value = j!({"a": "x", "b": 1, "c": false});
    let a = j!("x");
    let b = j!(1);
    let c = j!(true);
    verify_that!(value, not(json::pat!({"a": a, "b": b, "c": c})))
}

#[test]
fn pat_unmatch_with_borrowed_values() -> Result<()> {
    let value = j!({"a": "x", "b": 1, "c": false});
    let a = j!("x");
    let b = j!(1);
    let c = j!(true);
    verify_that!(value, not(json::pat!({"a": &a, "b": &b, "c": &c})))
}

#[test]
fn pat_unmatch_with_inline_borrowed_literals() -> Result<()> {
    verify_that!(
        j!({"a": "x", "b": 1, "c": false}),
        not(json::pat!({"a": &j!("x"), "b": &j!(1), "c": &j!(true)}))
    )
}

#[test]
fn pat_unmatch_with_inline_owned_literals() -> Result<()> {
    verify_that!(
        j!({"a": "x", "b": 1, "c": false}),
        not(json::pat!({"a": j!("x"), "b": j!(1), "c": j!(true)}))
    )
}

#[test]
fn pat_unmatch_with_mixed_owned_and_borrowed() -> Result<()> {
    let value = j!({"a": "x", "b": 1, "c": false});
    let a = j!("x");
    verify_that!(value, not(json::pat!({"a": a, "b": j!(1), "c": &j!(true)})))
}
