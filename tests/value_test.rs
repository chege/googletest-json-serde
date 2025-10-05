use googletest::prelude::*;
use googletest_json_serde::json;
use indoc::indoc;
use serde_json::json;

#[test]
fn i64_wrong_type() -> Result<()> {
    let val = json!(true);
    let result = verify_that!(val, json::primitive!(gt(10)));
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
    let result = verify_that!(val, json::primitive!(gt(1.0)));
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
    let result = verify_that!(val, json::primitive!(is_true()));
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
    verify_that!(val, not(json::primitive!(eq(5.0))))
}

#[test]
fn string_type() -> Result<()> {
    let val = json!("hello");
    verify_that!(val, json::primitive!(eq("hello")))
}
#[test]
fn i64_type() -> Result<()> {
    let val = json!(99i64);
    verify_that!(val, json::primitive!(eq(99i64)))
}
#[test]
fn f64_type() -> Result<()> {
    let val = json!(99.3f64);
    verify_that!(val, json::primitive!(eq(99.3f64)))
}
#[test]
fn bool_type() -> Result<()> {
    let val = json!(false);
    verify_that!(val, json::primitive!(is_false()))
}
