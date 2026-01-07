use googletest::Result;
use googletest::prelude::*;
use googletest_json_serde::json;
use indoc::indoc;
use serde_json::json as j;

#[test]
fn pat_matches_nested_object_strict() -> Result<()> {
    let val = j!({
        "user": {
            "id": 1,
            "name": "Alice"
        },
        "active": true
    });

    verify_that!(
        val,
        json::pat!({
            "user": json::pat!({
                "id": eq(1),
                "name": starts_with("Alic"),
            }),
            "active": j!(true),
        })
    )
}

#[test]
fn pat_matches_nested_object_strict_implicit() -> Result<()> {
    let val = j!({
        "user": {
            "id": 1,
            "name": "Alice"
        },
        "active": true
    });

    verify_that!(
        val,
        json::pat!({
            "user": {
                "id": eq(1),
                "name": starts_with("Alic"),
            },
            "active": j!(true),
        })
    )
}

#[test]
fn pat_matches_nested_object_non_strict() -> Result<()> {
    let val = j!({
        "user": {
            "id": 1,
            "name": "Alice",
            "extra": "data"
        }
    });

    verify_that!(
        val,
        json::pat!({
            "user": json::pat!({
                "id": ge(1),
                ..
            })
        })
    )
}

#[test]
fn pat_matches_nested_object_non_strict_implicit() -> Result<()> {
    let val = j!({
        "user": {
            "id": 1,
            "name": "Alice",
            "extra": "data"
        }
    });

    verify_that!(
        val,
        json::pat!({
            "user": {
                "id": ge(1),
                ..
            }
        })
    )
}

#[test]
fn pat_matches_nested_object_mixed_strictness_outer_relaxed_inner_strict() -> Result<()> {
    let val = j!({
        "user": {
            "id": 1,
            "name": "Alice"
        },
        "active": true,
        "extra_top": "ok"
    });

    verify_that!(
        val,
        json::pat!({
            "user": {
                "id": eq(1),
                "name": starts_with("Alic"),
            },
            "active": eq(true),
            ..
        })
    )
}

#[test]
fn pat_matches_nested_object_mixed_strictness_outer_strict_inner_relaxed() -> Result<()> {
    let val = j!({
        "user": {
            "id": 1,
            "name": "Alice",
            "extra": "ok"
        },
        "active": true
    });

    verify_that!(
        val,
        json::pat!({
            "user": {
                "id": ge(1),
                ..
            },
            "active": eq(true),
        })
    )
}

#[test]
fn pat_matches_nested_object_non_strict_outer_and_inner() -> Result<()> {
    let val = j!({
        "user": {
            "id": 1,
            "name": "Alice",
            "extra": "ok"
        },
        "active": true,
        "extra_top": "ok"
    });

    verify_that!(
        val,
        json::pat!({
            "user": {
                "id": eq(1),
                ..
            },
            "active": eq(true),
            ..
        })
    )
}

#[test]
fn pat_rejects_nested_object_outer_relaxed_inner_strict_with_extra_inner_field() -> Result<()> {
    let val = j!({
        "user": {
            "id": 1,
            "name": "Alice",
            "extra": "nope"
        },
        "active": true,
        "extra_top": "ok"
    });

    verify_that!(
        val,
        not(json::pat!({
            "user": {
                "id": eq(1),
                "name": eq("Alice"),
            },
            "active": eq(true),
            ..
        }))
    )
}

#[test]
fn pat_rejects_nested_object_outer_strict_inner_relaxed_with_extra_outer_field() -> Result<()> {
    let val = j!({
        "user": {
            "id": 1,
            "name": "Alice",
            "extra": "ok"
        },
        "active": true,
        "extra_top": "nope"
    });

    verify_that!(
        val,
        not(json::pat!({
            "user": {
                "id": eq(1),
                ..
            },
            "active": eq(true),
        }))
    )
}

#[test]
fn pat_rejects_nested_object_strict_with_extra_inner_field() -> Result<()> {
    let val = j!({
        "user": {
            "id": 1,
            "name": "Alice",
            "extra": "nope"
        },
        "active": true
    });

    verify_that!(
        val,
        not(json::pat!({
            "user": {
                "id": eq(1),
                "name": eq("Alice"),
            },
            "active": eq(true),
        }))
    )
}

#[test]
fn pat_rejects_nested_object_strict_with_extra_outer_field() -> Result<()> {
    let val = j!({
        "user": {
            "id": 1,
            "name": "Alice"
        },
        "active": true,
        "extra_top": "nope"
    });

    verify_that!(
        val,
        not(json::pat!({
            "user": {
                "id": eq(1),
                "name": eq("Alice"),
            },
            "active": eq(true),
        }))
    )
}

#[test]
fn pat_rejects_enum_like_object_strict() -> Result<()> {
    let val = j!({
        "type": "Dog",
        "bark": true
    });

    verify_that!(
        val,
        not(json::pat!({
            "type": eq("Cat"),
            "meow": is_true()
        }))
    )
}

#[test]
fn pat_matches_option_nested_mixed_matchers() -> Result<()> {
    let val = Some(j!({
        "type": "Dog",
        "props": {
            "bark": true,
            "age": 5
        }
    }));

    verify_that!(
        val,
        json::pat!({
            "type": eq("Dog"),
            "props": json::pat!({
                "bark": eq(true),
                "age": gt(2),
            }),
        })
    )
}

#[test]
fn pat_fails_on_unexpected_fields_strict() -> Result<()> {
    let val = j!({
        "a": 1,
        "b": 2,
        "unexpected": 3
    });

    verify_that!(
        val,
        not(json::pat!({
            "a": eq(1),
            "b": eq(2),
        }))
    )
}
#[test]
fn pat_matches_object_with_any_value_field() -> Result<()> {
    let val = j!({"field": "value", "unexpected": 123});
    verify_that!(
        val,
        json::pat!({
            "field": eq("value"),
            "unexpected": json::is_not_null()
        })
    )
}

#[test]
fn pat_rejects_option_none() -> Result<()> {
    let val: Option<serde_json::Value> = None;
    verify_that!(val, not(json::pat!({ "field": eq("value") })))
}

#[test]
fn pat_rejects_object_with_wrong_field() -> Result<()> {
    let val = j!({"field": "other"});
    verify_that!(
        val,
        not(json::pat!({
            "field": eq("value")
        }))
    )
}

#[test]
fn pat_explains_mismatch_nested_object() -> Result<()> {
    let val = j!({
        "field": {
            "subfield": 123,
            "flag": false
        },
        "extra": "hello"
    });

    let result = verify_that!(
        val,
        json::pat!({
            "field": json::pat!({
                "subfield": eq(999),
                "flag": eq(true)
            }),
            "extra": eq("world")
        })
    );
    verify_that!(
        result,
        err(displays_as(all![
            contains_substring("field 'field': had 2 field mismatches"),
            contains_substring("field 'subfield': which isn't equal to 999"),
            contains_substring("field 'flag': which isn't equal to true"),
            contains_substring("field 'extra': which isn't equal to \"world\""),
        ]))
    )
}

#[test]
fn pat_explains_single_field_mismatch() -> Result<()> {
    let result = verify_that!(
        j!({"foo": 1}),
        json::pat!({
            "foo": eq(2)
        })
    );
    verify_that!(
        result,
        err(displays_as(contains_substring(
            "field 'foo': which isn't equal to 2"
        )))
    )
}

#[test]
fn pat_explains_wrong_type() -> Result<()> {
    let result = verify_that!(
        j!(123),
        json::pat!({
            "foo": eq(1)
        })
    );
    verify_that!(
        result,
        err(displays_as(contains_substring("was 123 (expected object)")))
    )
}

#[test]
fn pat_explains_option_none() -> Result<()> {
    let result = verify_that!(
        None::<serde_json::Value>,
        json::pat!({
            "foo": eq(1)
        })
    );
    verify_that!(result, err(displays_as(contains_substring("was None"))))
}

#[test]
fn pat_explains_option_some_mismatch() -> Result<()> {
    let result = verify_that!(
        Some(j!({"foo": 1})),
        json::pat!({
            "foo": eq(2)
        })
    );
    verify_that!(
        result,
        err(displays_as(contains_substring(
            "field 'foo': which isn't equal to 2"
        )))
    )
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

#[test]
fn pat_matches_all_numeric_types_flat_object() -> Result<()> {
    let value = j!({
        "i8_val": 12i8,
        "i16_val": 32000i16,
        "i32_val": 123456i32,
        "i64_val": 9999999999i64,
        "u8_val": 200u8,
        "u16_val": 65000u16,
        "u32_val": 4000000000u32,
        "u64_val": 9000000000u64,
        "f64_val": 9.14159f64,
        "bool_val": true
    });

    verify_that!(
        value,
        json::pat!({
            "i8_val": eq(12i8),
            "i16_val": eq(32000i16),
            "i32_val": eq(123456i32),
            "i64_val": eq(9999999999i64),
            "u8_val": eq(200u8),
            "u16_val": eq(65000u16),
            "u32_val": eq(4000000000u32),
            "u64_val": eq(9000000000u64),
            "f64_val": eq(9.14159f64),
            "bool_val": eq(true),
        })
    )
}

#[test]
fn pat_matches_with_primitive_literals() -> Result<()> {
    let value = j!({"a": "x", "b": 1, "c": true});
    verify_that!(
        value,
        json::pat!({
            "a": "x",   // literal string
            "b": 1i64,     // literal number
            ..
        })
    )
}

#[test]
fn pat_unmatch_with_primitive_literals() -> Result<()> {
    let value = j!({"a": "x", "b": 1, "c": false});
    verify_that!(
        value,
        not(json::pat!({
            "a": "x",
            "b": 2i64,
            "c": true
        }))
    )
}
