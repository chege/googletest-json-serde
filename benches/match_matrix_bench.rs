use criterion::{Criterion, black_box, criterion_group, criterion_main};
use googletest::matcher::Matcher;
use googletest_json_serde::matchers::__internal_unstable_do_not_depend_on_these::{
    IntoJsonMatcher, JsonUnorderedElementsAreMatcher, Literal, Requirements,
};
use serde_json::Value;
use std::time::Duration;

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

fn bench_match_paths(c: &mut Criterion) {
    let mut group = c.benchmark_group("match_matrix/matches");

    for &size in &[1_000usize, 5_000, 10_000] {
        let actual: Vec<i64> = (0..size as i64).collect();

        let expected_perfect: Vec<i64> = actual.iter().rev().copied().collect();
        let perfect = make_unordered_eq_matcher(&expected_perfect, Requirements::PerfectMatch);
        let actual_value = as_json_array(&actual);
        group.bench_function(format!("perfect/n={size}"), |b| {
            b.iter(|| black_box(perfect.matches(black_box(&actual_value))))
        });

        let expected_superset: Vec<i64> = ((size / 4) as i64..(size / 2) as i64).collect();
        let superset = make_unordered_eq_matcher(&expected_superset, Requirements::Superset);
        group.bench_function(format!("superset/n={size}"), |b| {
            b.iter(|| black_box(superset.matches(black_box(&actual_value))))
        });

        let expected_subset: Vec<i64> = (0..(size as i64 + size as i64 / 2)).collect();
        let subset = make_unordered_eq_matcher(&expected_subset, Requirements::Subset);
        group.bench_function(format!("subset/n={size}"), |b| {
            b.iter(|| black_box(subset.matches(black_box(&actual_value))))
        });
    }

    group.finish();
}

fn bench_explain_paths(c: &mut Criterion) {
    let mut group = c.benchmark_group("match_matrix/explain_match");

    for &size in &[1_000usize, 5_000] {
        let mut actual: Vec<i64> = (0..size as i64).collect();
        let expected: Vec<i64> = actual.iter().rev().copied().collect();
        actual[size - 1] = -1;

        let matcher = make_unordered_eq_matcher(&expected, Requirements::PerfectMatch);
        let value = as_json_array(&actual);
        group.bench_function(format!("perfect_mismatch/n={size}"), |b| {
            b.iter(|| black_box(matcher.explain_match(black_box(&value))))
        });
    }

    group.finish();
}

fn criterion_config() -> Criterion {
    Criterion::default()
        .sample_size(10)
        .measurement_time(Duration::from_secs(4))
        .warm_up_time(Duration::from_secs(1))
}

criterion_group! {
    name = benches;
    config = criterion_config();
    targets = bench_match_paths, bench_explain_paths
}
criterion_main!(benches);
