use googletest::prelude::*;
use googletest_json_serde::json;
use serde_json::json;

#[test]
fn match_nested_object_strict() {
    let val = json!({
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
            "active": eq(true),
        })
    );
}

#[test]
fn match_nested_object_non_strict() {
    let val = json!({
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
                "id": eq(1),
                ..
            })
        })
    );
}

#[test]
fn match_enum_like_object_strict_mismatch() {
    let val = json!({
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
    let val = Some(json!({
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
    let val = json!({
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
    let val = json!({"field": "value", "unexpected": 123});
    assert_that!(
        val,
        json::pat!({
            "field": eq("value"),
            "unexpected": json::any_value()
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
    let val = json!({"field": "other"});
    assert_that!(
        val,
        not(json::pat!({
            "field": eq("value")
        }))
    );
}

#[test]
fn explain_mismatch_nested_object() {
    let val = json!({
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
    let val = json!({"foo": 1});
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
    let val = json!(123);
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
    let val = Some(json!({"foo": 1}));
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
