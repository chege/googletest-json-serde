use googletest::prelude::*;
use googletest_json_serde::json as j;
use indoc::indoc;
use serde_json::json;

#[test]
fn i64_wrong_type() -> Result<()> {
    let val = json!(true);
    let result = verify_that!(val, j::primitive!(gt(10)));
    verify_that!(
        result,
        err(displays_as(starts_with(indoc!(
            "Value of: val\n\
                 Expected: is greater than 10\n\
                 Actual: Bool(true),\n\
                 \x20\x20which is not a JSON number\n"
        ))))
    )
}

#[test]
fn f64_wrong_type() -> Result<()> {
    let val = json!("wat");
    let result = verify_that!(val, j::primitive!(gt(1.0)));
    verify_that!(
        result,
        err(displays_as(starts_with(indoc!(
            "Value of: val\n\
                 Expected: is greater than 1.0\n\
                 Actual: String(\"wat\"),\n\
                 \x20\x20which is not a JSON number\n"
        ))))
    )
}

#[test]
fn bool_wrong_type() -> Result<()> {
    let val = json!(123);
    let result = verify_that!(val, j::primitive!(is_true()));
    verify_that!(
        result,
        err(displays_as(starts_with(indoc!(
            "Value of: val\n\
                 Expected: is true\n\
                 Actual: Number(123),\n\
                 \x20\x20which is not a JSON boolean"
        ))))
    )
}

#[test]
fn f64_nan_not_match() -> Result<()> {
    let val = json!(f64::NAN);
    verify_that!(val, not(j::primitive!(eq(5.0))))
}

#[test]
fn string_type() -> Result<()> {
    let val = json!("hello");
    verify_that!(val, j::primitive!(eq("hello")))
}
#[test]
fn i64_type() -> Result<()> {
    let val = json!(99i64);
    verify_that!(val, j::primitive!(eq(99i64)))
}
#[test]
fn i8_type() -> Result<()> {
    let val = json!(123i8);
    verify_that!(val, j::primitive!(eq(123i8)))
}

#[test]
fn i8_out_of_range_fails() -> Result<()> {
    let val = json!(200);
    let result = verify_that!(val, j::primitive!(eq(123i8)));
    verify_that!(
        result,
        err(displays_as(starts_with(indoc!(
            "Value of: val\n\
                 Expected: is equal to 123\n\
                 Actual: Number(200),\n\
                 \x20\x20number out of i8 range: 200"
        ))))
    )
}

#[test]
fn i16_type() -> Result<()> {
    let val = json!(12345i16);
    verify_that!(val, j::primitive!(eq(12345i16)))
}

#[test]
fn i16_out_of_range_fails() -> Result<()> {
    let val = json!(40000);
    let result = verify_that!(val, j::primitive!(eq(12345i16)));
    verify_that!(
        result,
        err(displays_as(starts_with(indoc!(
            "Value of: val\n\
                 Expected: is equal to 12345\n\
                 Actual: Number(40000),\n\
                 \x20\x20number out of i16 range: 40000"
        ))))
    )
}

#[test]
fn u8_type() -> Result<()> {
    let val = json!(200u8);
    verify_that!(val, j::primitive!(eq(200u8)))
}

#[test]
fn u8_out_of_range_fails() -> Result<()> {
    let val = json!(300);
    let result = verify_that!(val, j::primitive!(eq(200u8)));
    verify_that!(
        result,
        err(displays_as(starts_with(indoc!(
            "Value of: val\n\
                 Expected: is equal to 200\n\
                 Actual: Number(300),\n\
                 \x20\x20number out of u8 range: 300"
        ))))
    )
}

#[test]
fn u16_type() -> Result<()> {
    let val = json!(60000u16);
    verify_that!(val, j::primitive!(eq(60000u16)))
}

#[test]
fn u16_out_of_range_fails() -> Result<()> {
    let val = json!(70000);
    let result = verify_that!(val, j::primitive!(eq(60000u16)));
    verify_that!(
        result,
        err(displays_as(starts_with(indoc!(
            "Value of: val\n\
                 Expected: is equal to 60000\n\
                 Actual: Number(70000),\n\
                 \x20\x20number out of u16 range: 70000"
        ))))
    )
}

#[test]
fn u32_type() -> Result<()> {
    let val = json!(4000000000u32);
    verify_that!(val, j::primitive!(eq(4000000000u32)))
}

#[test]
fn u32_out_of_range_fails() -> Result<()> {
    let val = json!(9000000000u64);
    let result = verify_that!(val, j::primitive!(eq(4000000000u32)));
    verify_that!(
        result,
        err(displays_as(starts_with(indoc!(
            "Value of: val\n\
                 Expected: is equal to 4000000000\n\
                 Actual: Number(9000000000),\n\
                 \x20\x20number out of u32 range: 9000000000"
        ))))
    )
}

#[test]
fn i32_type() -> Result<()> {
    let val = json!(1234567890i32);
    verify_that!(val, j::primitive!(eq(1234567890i32)))
}

#[test]
fn i32_out_of_range_fails() -> Result<()> {
    let val = json!(3000000000u64);
    let result = verify_that!(val, j::primitive!(eq(1234567890i32)));
    verify_that!(
        result,
        err(displays_as(starts_with(indoc!(
            "Value of: val\n\
                 Expected: is equal to 1234567890\n\
                 Actual: Number(3000000000),\n\
                 \x20\x20number out of i32 range: 3000000000"
        ))))
    )
}
#[test]
fn f64_type() -> Result<()> {
    let val = json!(99.3f64);
    verify_that!(val, j::primitive!(eq(99.3f64)))
}
#[test]
fn bool_type() -> Result<()> {
    let val = json!(false);
    verify_that!(val, j::primitive!(is_false()))
}

#[test]
fn primitive_produces_correct_failure_message() -> Result<()> {
    let result = verify_that!(json!(5), j::primitive!(gt(10)));
    verify_that!(
        result,
        err(displays_as(starts_with(indoc!(
            r#"
                Value of: json!(5)
                Expected: is greater than 10
                Actual: Number(5),
                  which is less than or equal to 10"#
        ))))
    )
}
