use googletest::matcher::MatcherResult;
use googletest::prelude::*;
use googletest_json_serde::json as j;
use serde_json::json;

#[test]
fn each_key_matches_top_level_object_keys() -> Result<()> {
    verify_that!(
        json!({"usr_id": 1, "usr_name": "Nadja"}),
        j::each_key(starts_with("usr_"))
    )
}

#[test]
fn each_key_unmatches_when_a_key_fails() -> Result<()> {
    verify_that!(
        json!({"usr_id": 1, "name": "Nadja"}),
        not(j::each_key(starts_with("usr_")))
    )
}

#[test]
fn each_key_explains_failed_key() -> Result<()> {
    let result = verify_that!(
        json!({"usr_id": 1, "name": "Nadja"}),
        j::each_key(starts_with("usr_"))
    );
    verify_that!(
        result,
        err(displays_as(all![
            contains_substring("name"),
            contains_substring("starts with")
        ]))
    )
}

#[test]
fn each_value_matches_top_level_object_values() -> Result<()> {
    verify_that!(json!({"hp": 10, "mp": 20}), j::each_value(j::as_i64(gt(0))))
}

#[test]
fn each_value_unmatches_when_a_value_fails() -> Result<()> {
    verify_that!(
        json!({"hp": 10, "mp": -1}),
        not(j::each_value(j::as_i64(gt(0))))
    )
}

#[test]
fn each_value_explains_failed_value() -> Result<()> {
    let result = verify_that!(json!({"hp": 10, "mp": -1}), j::each_value(j::as_i64(gt(0))));
    verify_that!(
        result,
        err(displays_as(all![
            contains_substring("-1"),
            contains_substring("greater than")
        ]))
    )
}

#[test]
fn each_key_describes_match_and_no_match() -> Result<()> {
    let matcher = j::each_key(starts_with("usr_"));
    verify_that!(
        matcher.describe(MatcherResult::Match),
        displays_as(contains_substring(
            "is a JSON object whose every top-level key"
        ))
    )?;
    verify_that!(
        matcher.describe(MatcherResult::NoMatch),
        displays_as(contains_substring(
            "is not a JSON object or has a top-level key"
        ))
    )
}

#[test]
fn each_value_describes_match_and_no_match() -> Result<()> {
    let matcher = j::each_value(j::as_i64(gt(0)));
    verify_that!(
        matcher.describe(MatcherResult::Match),
        displays_as(contains_substring(
            "is a JSON object whose every top-level value"
        ))
    )?;
    verify_that!(
        matcher.describe(MatcherResult::NoMatch),
        displays_as(contains_substring(
            "is not a JSON object or has a top-level value"
        ))
    )
}

#[test]
fn each_key_fails_cleanly_on_non_object() -> Result<()> {
    let result = verify_that!(json!(["usr_id"]), j::each_key(starts_with("usr_")));
    verify_that!(
        result,
        err(displays_as(all![
            contains_substring("expected JSON object"),
            contains_substring("JSON array")
        ]))
    )
}

#[test]
fn each_key_reports_null_boolean_and_number_type_mismatches() -> Result<()> {
    for (value, type_name) in [
        (json!(null), "JSON null"),
        (json!(true), "JSON boolean"),
        (json!(1), "JSON number"),
    ] {
        let result = verify_that!(value, j::each_key(starts_with("usr_")));
        verify_that!(
            result,
            err(displays_as(all![
                contains_substring("expected JSON object"),
                contains_substring(type_name)
            ]))
        )?;
    }
    Ok(())
}

#[test]
fn each_value_fails_cleanly_on_non_object() -> Result<()> {
    let result = verify_that!(json!("value"), j::each_value(j::is_string()));
    verify_that!(
        result,
        err(displays_as(all![
            contains_substring("expected JSON object"),
            contains_substring("JSON string")
        ]))
    )
}
