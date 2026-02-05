use googletest::prelude::*;
use googletest_json_serde::json as j;
use serde_json::json;

#[test]
fn as_string_matches_string_value() {
    assert_that!(json!("hello"), j::as_string(starts_with("h")));
}

#[test]
fn as_string_does_not_match_non_string_value() {
    assert_that!(json!(42), not(j::as_string(anything())));
}

#[test]
fn as_bool_matches_bool_value() {
    assert_that!(json!(true), j::as_bool(eq(true)));
    assert_that!(json!(false), j::as_bool(eq(false)));
}

#[test]
fn as_bool_does_not_match_non_bool_value() {
    assert_that!(json!(null), not(j::as_bool(anything())));
}

#[test]
fn as_i64_matches_i64_value() {
    assert_that!(json!(42), j::as_i64(eq(42)));
    assert_that!(json!(-10), j::as_i64(lt(0)));
}

#[test]
fn as_u64_matches_u64_value() {
    assert_that!(json!(100), j::as_u64(gt(50)));
}

#[test]
fn as_f64_matches_f64_value() {
    assert_that!(json!(3.14), j::as_f64(near(3.1, 0.1)));
}

#[test]
fn as_i32_matches_i32_value() {
    assert_that!(json!(123), j::as_i32(eq(123)));
}

#[test]
fn as_i32_fails_on_overflow() {
    assert_that!(json!(i64::MAX), not(j::as_i32(anything())));
}

#[test]
fn as_array_matches_array_value() {
    assert_that!(json!([1, 2, 3]), j::as_array(len(eq(3))));
}

#[test]
fn as_array_does_not_match_non_array_value() {
    assert_that!(json!({"a": 1}), not(j::as_array(anything())));
}

#[test]
fn as_object_matches_object_value() {
    assert_that!(json!({"a": 1}), j::as_object(len(eq(1))));
}

#[test]
fn as_object_does_not_match_non_object_value() {
    assert_that!(json!([]), not(j::as_object(anything())));
}

#[test]
fn composition_with_pat() {
    assert_that!(
        json!({
            "name": "Alice",
            "age": 30,
            "active": true,
            "tags": ["admin", "staff"]
        }),
        j::pat!({
            "name": j::as_string(starts_with("A")),
            "age": j::as_i64(ge(18)),
            "active": j::as_bool(eq(true)),
            "tags": j::as_array(contains(j::as_string(eq("admin"))))
        })
    );
}

#[test]
fn handles_anything_ambiguity() {
    // This would fail to compile with primitive!(anything()) or just anything()
    assert_that!(json!(42), j::as_i64(anything()));
    assert_that!(json!("hi"), j::as_string(anything()));
}

#[test]
fn nested_as_object() {
    assert_that!(
        json!({"user": {"id": "123"}}),
        j::pat!({
            "user": j::as_object(len(eq(1)))
        })
    );
}

#[test]
fn test_all_int_types() {
    assert_that!(json!(1), j::as_i8(eq(1)));
    assert_that!(json!(1), j::as_u8(eq(1)));
    assert_that!(json!(1), j::as_i16(eq(1)));
    assert_that!(json!(1), j::as_u16(eq(1)));
    assert_that!(json!(1), j::as_i32(eq(1)));
    assert_that!(json!(1), j::as_u32(eq(1)));
    assert_that!(json!(1), j::as_usize(eq(1)));
}
