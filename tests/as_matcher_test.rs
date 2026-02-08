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
    assert_that!(
        json!(std::f64::consts::PI),
        j::as_f64(near(std::f64::consts::PI, 0.1))
    );
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
fn handles_anything_ambiguity() {
    // This would fail to compile with primitive!(anything()) or just anything()
    assert_that!(json!(42), j::as_i64(anything()));
    assert_that!(json!("hi"), j::as_string(anything()));
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

#[test]
fn as_i8_fails_on_overflow() {
    assert_that!(json!(128), not(j::as_i8(anything())));
    assert_that!(json!(-129), not(j::as_i8(anything())));
}

#[test]
fn as_u8_fails_on_overflow_or_underflow() {
    assert_that!(json!(256), not(j::as_u8(anything())));
    assert_that!(json!(-1), not(j::as_u8(anything())));
}

#[test]
fn as_i16_fails_on_overflow() {
    assert_that!(json!(32768), not(j::as_i16(anything())));
    assert_that!(json!(-32769), not(j::as_i16(anything())));
}

#[test]
fn as_u16_fails_on_overflow_or_underflow() {
    assert_that!(json!(65536), not(j::as_u16(anything())));
    assert_that!(json!(-1), not(j::as_u16(anything())));
}

#[test]
fn as_i32_fails_on_underflow() {
    assert_that!(json!(i64::MIN), not(j::as_i32(anything())));
}

#[test]
fn as_u32_fails_on_overflow_or_underflow() {
    assert_that!(json!(u64::MAX), not(j::as_u32(anything())));
    assert_that!(json!(-1), not(j::as_u32(anything())));
}

#[test]
fn as_usize_fails_on_overflow_or_underflow() {
    // Testing underflow for usize, which always fails for negative numbers.
    assert_that!(json!(-1), not(j::as_usize(anything())));
}

#[test]
fn as_f64_handles_numbers_losing_precision_gracefully() {
    let large_integer_json = json!(9007199254740993u64); // 2^53 + 1, loses precision in f64
    let expected_f64 = 9007199254740992.0; // The actual value f64 will hold due to precision loss
    assert_that!(large_integer_json, j::as_f64(eq(expected_f64)));
    // Test a non-f64 number that might fail as_f64()
    // For serde_json, as_f64 usually only fails if the number is out of f64 range
    // which is hard to construct from a simple json!(...).
    // So for now, we only test the precision loss.
}

#[test]
fn as_string_explains_non_string() -> Result<()> {
    let result = verify_that!(json!(42), j::as_string(eq("hi")));
    verify_that!(
        result,
        err(displays_as(contains_substring("which is a JSON number")))
    )
}

#[test]
fn as_bool_explains_non_bool() -> Result<()> {
    let result = verify_that!(json!("true"), j::as_bool(eq(true)));
    verify_that!(
        result,
        err(displays_as(contains_substring("which is a JSON string")))
    )
}

#[test]
fn as_i64_explains_out_of_range() -> Result<()> {
    let result = verify_that!(json!(u64::MAX), j::as_i64(eq(1)));
    verify_that!(
        result,
        err(displays_as(contains_substring("out of i64 range")))
    )
}

#[test]
fn as_u64_explains_out_of_range() -> Result<()> {
    let result = verify_that!(json!(-1), j::as_u64(eq(1)));
    verify_that!(
        result,
        err(displays_as(contains_substring("out of u64 range")))
    )
}

#[test]
fn as_i32_explains_not_valid_number() -> Result<()> {
    let result = verify_that!(json!(1.2), j::as_i32(eq(1)));
    verify_that!(
        result,
        err(displays_as(contains_substring("not a valid i32 number")))
    )
}

#[test]
fn as_i32_explains_out_of_range() -> Result<()> {
    let result = verify_that!(json!(i64::MAX), j::as_i32(eq(1)));
    verify_that!(
        result,
        err(displays_as(contains_substring("out of i32 range")))
    )
}

#[test]
fn as_array_explains_non_array() -> Result<()> {
    let result = verify_that!(json!({"a": 1}), j::as_array(len(eq(1))));
    verify_that!(
        result,
        err(displays_as(contains_substring("which is a JSON object")))
    )
}

#[test]
fn as_object_explains_non_object() -> Result<()> {
    let result = verify_that!(json!([1, 2]), j::as_object(len(eq(1))));
    verify_that!(
        result,
        err(displays_as(contains_substring("which is a JSON array")))
    )
}
