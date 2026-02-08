use googletest::Result;
use googletest::prelude::*;
use googletest_json_serde::matchers::__internal_unstable_do_not_depend_on_these::{
    IntoJsonMatcher, JsonUnorderedElementsAreMatcher, Literal, Requirements,
};
use serde_json::Value;

fn as_json_array(values: &[i64]) -> Value {
    Value::Array(values.iter().copied().map(Value::from).collect())
}

fn make_unordered_eq_matcher(
    expected: &[i64],
    requirements: Requirements,
) -> JsonUnorderedElementsAreMatcher {
    let elements = expected
        .iter()
        .copied()
        .map(IntoJsonMatcher::<Literal>::into_json_matcher)
        .collect();
    JsonUnorderedElementsAreMatcher::new(elements, requirements)
}

#[test]
fn perfect_match_handles_hundreds_of_elements_in_reverse_order() -> Result<()> {
    let actual: Vec<i64> = (0..512).map(i64::from).collect();
    let expected: Vec<i64> = actual.iter().rev().copied().collect();

    let matcher = make_unordered_eq_matcher(&expected, Requirements::PerfectMatch);
    verify_that!(as_json_array(&actual), matcher)
}

#[test]
fn perfect_match_respects_duplicate_cardinality() -> Result<()> {
    let actual: Vec<i64> = [vec![7; 128], vec![3; 128]].concat();
    let expected: Vec<i64> = [vec![7; 127], vec![3; 129]].concat();

    let matcher = make_unordered_eq_matcher(&expected, Requirements::PerfectMatch);
    verify_that!(as_json_array(&actual), not(matcher))
}

#[test]
fn superset_mode_matches_when_actual_contains_all_expected() -> Result<()> {
    let actual: Vec<i64> = (0..300).map(i64::from).collect();
    let expected: Vec<i64> = (120..180).map(i64::from).collect();

    let matcher = make_unordered_eq_matcher(&expected, Requirements::Superset);
    verify_that!(as_json_array(&actual), matcher)
}

#[test]
fn subset_mode_matches_when_actual_is_contained_in_expected() -> Result<()> {
    let actual: Vec<i64> = (0..120).map(i64::from).collect();
    let expected: Vec<i64> = (0..250).map(i64::from).collect();

    let matcher = make_unordered_eq_matcher(&expected, Requirements::Subset);
    verify_that!(as_json_array(&actual), matcher)
}

#[test]
fn explain_match_reports_unmatched_actual_and_expected_elements() -> Result<()> {
    let actual = as_json_array(&[10, 20, 30, 99]);
    let matcher = make_unordered_eq_matcher(&[10, 20, 30, 77], Requirements::PerfectMatch);

    verify_that!(
        matcher.explain_match(&actual),
        displays_as(all!(
            contains_substring("does not match any expected elements"),
            contains_substring("no elements match the expected element")
        ))
    )
}

#[test]
#[ignore = "slow perf guard for matrix-heavy matcher changes"]
fn perfect_match_large_perf_guard() -> Result<()> {
    let actual: Vec<i64> = (0..4000).map(i64::from).collect();
    let expected: Vec<i64> = actual.iter().rev().copied().collect();

    let matcher = make_unordered_eq_matcher(&expected, Requirements::PerfectMatch);
    verify_that!(as_json_array(&actual), matcher)
}

#[test]
fn explanation_branch_no_actual_but_multiple_expected_unmatchable() -> Result<()> {
    // Equal lengths avoids size-short-circuit and forces unmatchable explanation path.
    let matcher = make_unordered_eq_matcher(&[1, 2, 3], Requirements::PerfectMatch);
    verify_that!(
        matcher.explain_match(&as_json_array(&[1, 1, 1])),
        displays_as(eq(
            "which has no elements matching the expected elements #1, #2"
        ))
    )
}

#[test]
fn explanation_branch_many_actual_one_expected_unmatchable() -> Result<()> {
    let matcher = make_unordered_eq_matcher(&[1, 1, 4], Requirements::PerfectMatch);
    verify_that!(
        matcher.explain_match(&as_json_array(&[1, 2, 3])),
        displays_as(eq(
            "whose elements #1, #2 do not match any expected elements and no elements match the expected element #2"
        ))
    )
}

#[test]
fn explanation_branch_one_actual_many_expected_unmatchable() -> Result<()> {
    let matcher = make_unordered_eq_matcher(&[1, 3, 4], Requirements::PerfectMatch);
    verify_that!(
        matcher.explain_match(&as_json_array(&[1, 1, 2])),
        displays_as(eq(
            "whose element #2 does not match any expected elements and no elements match the expected elements #1, #2"
        ))
    )
}

#[test]
#[ignore = "slow perf guard for matrix-heavy matcher changes"]
fn contains_each_semantics_large_perf_guard() -> Result<()> {
    let actual: Vec<i64> = (0..4500).map(i64::from).collect();
    let expected: Vec<i64> = (900..1900).map(i64::from).collect();

    let matcher = make_unordered_eq_matcher(&expected, Requirements::Superset);
    verify_that!(as_json_array(&actual), matcher)
}

#[test]
#[ignore = "slow perf guard for matrix-heavy matcher changes"]
fn is_contained_in_semantics_large_perf_guard() -> Result<()> {
    let actual: Vec<i64> = (0..1000).map(i64::from).collect();
    let expected: Vec<i64> = (0..5000).map(i64::from).collect();

    let matcher = make_unordered_eq_matcher(&expected, Requirements::Subset);
    verify_that!(as_json_array(&actual), matcher)
}
