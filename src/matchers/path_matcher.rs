use crate::matcher_support::path::{ParsedPaths, collect_paths, format_path, parse_expected_paths};
use crate::matchers::__internal_unstable_do_not_depend_on_these;
use crate::matchers::__internal_unstable_do_not_depend_on_these::JsonPredicateMatcher;
use googletest::description::Description;
use serde_json::Value;
use std::collections::BTreeSet;

/// Matches a JSON object that contains all the specified paths (order-agnostic, extras allowed).
pub fn has_paths(paths: &[&str]) -> JsonPredicateMatcher<impl Fn(&Value) -> bool, String, String> {
    let ParsedPaths { parsed, errors } = parse_expected_paths(paths);
    let expected_set: BTreeSet<_> = parsed.iter().map(|p| p.segments.clone()).collect();
    let errors_for_explain = errors.clone();
    let expected_desc = format!(
        "a JSON object containing paths {:?}",
        parsed.iter().map(|p| &p.raw).collect::<Vec<_>>()
    );
    let negative_desc = format!(
        "which is missing one of {:?}",
        parsed.iter().map(|p| &p.raw).collect::<Vec<_>>()
    );

    JsonPredicateMatcher::new(
        {
            let expected_set = expected_set.clone();
            let errors = errors.clone();
            move |v| {
                if !errors.is_empty() || !v.is_object() {
                    return false;
                }
                let actual = collect_paths(v);
                expected_set.iter().all(|p| actual.contains(p))
            }
        },
        expected_desc,
        negative_desc,
    )
    .with_explain_fn(move |v| {
        if !errors_for_explain.is_empty() {
            return Description::new().text(format!(
                "invalid paths {:?}",
                errors_for_explain
                    .iter()
                    .map(|e| e.as_str())
                    .collect::<Vec<_>>()
            ));
        }
        if !v.is_object() {
            return __internal_unstable_do_not_depend_on_these::describe_json_type(v);
        }
        let actual = collect_paths(v);
        let missing: BTreeSet<_> = expected_set.difference(&actual).cloned().collect();
        if missing.is_empty() {
            Description::new()
        } else {
            Description::new().text(format!(
                "missing paths {:?}",
                missing.iter().map(|p| format_path(p)).collect::<Vec<_>>()
            ))
        }
    })
}

/// Matches a JSON object whose paths are exactly the provided set (no extras or missing).
pub fn has_only_paths(
    paths: &[&str],
) -> JsonPredicateMatcher<impl Fn(&Value) -> bool, String, String> {
    let ParsedPaths { parsed, errors } = parse_expected_paths(paths);
    let expected_set: BTreeSet<_> = parsed.iter().map(|p| p.segments.clone()).collect();
    let errors_for_explain = errors.clone();
    let expected_desc = format!(
        "a JSON object with exactly paths {:?}",
        parsed.iter().map(|p| &p.raw).collect::<Vec<_>>()
    );
    let negative_desc = format!(
        "which does not have exactly paths {:?}",
        parsed.iter().map(|p| &p.raw).collect::<Vec<_>>()
    );

    JsonPredicateMatcher::new(
        {
            let expected_set = expected_set.clone();
            let errors = errors.clone();
            move |v| {
                if !errors.is_empty() || !v.is_object() {
                    return false;
                }
                let actual = collect_paths(v);
                actual == expected_set
            }
        },
        expected_desc,
        negative_desc,
    )
    .with_explain_fn(move |v| {
        if !errors_for_explain.is_empty() {
            return Description::new().text(format!(
                "invalid paths {:?}",
                errors_for_explain
                    .iter()
                    .map(|e| e.as_str())
                    .collect::<Vec<_>>()
            ));
        }
        if !v.is_object() {
            return __internal_unstable_do_not_depend_on_these::describe_json_type(v);
        }
        let actual = collect_paths(v);
        let missing: BTreeSet<_> = expected_set.difference(&actual).cloned().collect();
        let extra: BTreeSet<_> = actual.difference(&expected_set).cloned().collect();
        match (!missing.is_empty(), !extra.is_empty()) {
            (true, true) => Description::new()
                .text(format!(
                    "missing paths {:?}",
                    missing.iter().map(|p| format_path(p)).collect::<Vec<_>>()
                ))
                .text(format!(
                    ", extra paths {:?}",
                    extra.iter().map(|p| format_path(p)).collect::<Vec<_>>()
                )),
            (true, false) => Description::new().text(format!(
                "missing paths {:?}",
                missing.iter().map(|p| format_path(p)).collect::<Vec<_>>()
            )),
            (false, true) => Description::new().text(format!(
                "extra paths {:?}",
                extra.iter().map(|p| format_path(p)).collect::<Vec<_>>()
            )),
            (false, false) => Description::new(),
        }
    })
}
