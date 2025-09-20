use googletest::Result;
use googletest::prelude::*;
use googletest_serde_json::json;
use serde_json::json as j;

#[test]
fn contains_each_matches_one_to_one() -> Result<()> {
    verify_that!(
        j!(["a", "b", "c"]),
        json::contains_each![eq("a"), eq("b"), eq("c")]
    )
}

#[test]
fn contains_each_trailing_comma() -> Result<()> {
    verify_that!(
        j!(["a", "b", "c"]),
        json::contains_each![eq("a"), eq("b"), eq("c"),]
    )
}

#[test]
fn contains_each_empty_matchers() -> Result<()> {
    verify_that!(j!(["a", "b", "c"]), json::contains_each![])
}

#[test]
fn contains_each_empty_matchers_trailing_comma() -> Result<()> {
    verify_that!(j!(["a", "b", "c"]), json::contains_each![,])
}

#[test]
fn contains_each_empty_input_and_matchers() -> Result<()> {
    verify_that!(j!([]), json::contains_each![])
}

#[test]
fn contains_each_excess_elements() -> Result<()> {
    verify_that!(
        j!(["a", "b", "c", "d"]),
        json::contains_each![eq("b"), eq("c"), eq("d")]
    )
}

#[test]
fn contains_each_unmatched_fails() -> Result<()> {
    verify_that!(
        j!(["a", "b", "c"]),
        not(json::contains_each![eq("b"), eq("c"), eq("x")])
    )
}

#[test]
fn contains_each_mismatch_size_description() -> Result<()> {
    let matcher = json::contains_each![eq("x"), eq("y"), eq("z")];
    verify_that!(
        matcher.explain_match(&j!(["x", "z"])),
        displays_as(eq(
            "which has size 2 (expected at least 3) and no element matching the expected element #1"
        ))
    )
}

#[test]
fn contains_each_mismatch_missing_one() -> Result<()> {
    let matcher = json::contains_each![eq("x"), eq("y"), eq("z")];
    verify_that!(
        matcher.explain_match(&j!(["x", "z"])),
        displays_as(eq(
            "which has size 2 (expected at least 3) and no element matching the expected element #1"
        ))
    )
}
