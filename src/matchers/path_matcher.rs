use crate::matcher_support::path::{ParsedPaths, collect_paths, format_path, parse_expected_paths};
use crate::matchers::__internal_unstable_do_not_depend_on_these;
use crate::matchers::__internal_unstable_do_not_depend_on_these::JsonPredicateMatcher;
use googletest::description::Description;
use serde_json::Value;
use std::collections::BTreeSet;

/// Matches a JSON leaf at the given path against the provided matcher.
///
/// The path uses the same dot-and-escape rules as [`has_paths`]. The matcher can be a literal,
/// a `serde_json::Value`, or any native googletest matcher.
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json as j;
/// # use serde_json::json;
/// let value = json!({"user": {"id": 7, "name": "Ada"}});
/// assert_that!(
///     value,
///     j::has_path_with!("user.name", "Ada")
///         .and(j::has_path_with!("user.id", json!(7)))
///         .and(j::has_path_with!("user.name", starts_with("A")))
/// );
/// ```
///
/// # Supported Inputs
/// - Literal JSON-compatible values
/// - Direct `serde_json::Value`
/// - Native googletest matchers
///
/// # Errors
/// Fails when the path is invalid, missing, the value is not an object, or the leaf does not
/// satisfy the matcher.
#[macro_export]
#[doc(hidden)]
macro_rules! __json_has_path_with {
    ($path:expr, $matcher:expr) => {{
        $crate::matchers::__internal_unstable_do_not_depend_on_these::JsonPathWithMatcher::new(
            $path,
            $crate::matchers::__internal_unstable_do_not_depend_on_these::IntoJsonMatcher::into_json_matcher($matcher),
        )
    }};
}

/// Matches a JSON object that contains all specified paths (order-agnostic, extras allowed).
///
/// Paths use dot notation; escape dots inside field names with `\`.
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json as j;
/// # use serde_json::json;
/// let value = json!({"user": {"id": 7, "name": "Ada"}});
/// assert_that!(value, j::has_paths(&["user.id", "user.name"]));
/// ```
///
/// # Errors
///
/// Fails when any path is invalid, when the value is not a JSON object, or when required paths are missing.
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
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json as j;
/// # use serde_json::json;
/// let value = json!({"ids": [1, 2], "ok": true});
/// assert_that!(value, j::has_only_paths(&["ids", "ids.0", "ids.1", "ok"]));
/// ```
///
/// # Errors
///
/// Fails when any path is invalid, when the value is not a JSON object, or when the set of paths differs.
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

#[doc(hidden)]
pub mod internal {
    use crate::matcher_support::path::{PathSegment, format_path, parse_expected_paths};
    use crate::matchers::__internal_unstable_do_not_depend_on_these::describe_json_type;
    use crate::matchers::json_matcher::internal::JsonMatcher;
    use googletest::description::Description;
    use googletest::matcher::{Matcher, MatcherBase, MatcherResult};
    use serde_json::Value;

    #[derive(MatcherBase)]
    pub struct JsonPathWithMatcher {
        raw: String,
        segments: Vec<PathSegment>,
        matcher: Box<dyn JsonMatcher>,
        parse_error: Option<String>,
    }

    fn parse_single_path(path: &str) -> (Vec<PathSegment>, Option<String>) {
        let parsed_paths = parse_expected_paths(&[path]);
        match (parsed_paths.parsed.first(), parsed_paths.errors.first()) {
            (_, Some(err)) => (Vec::new(), Some(err.clone())),
            (Some(p), None) => (p.segments.clone(), None),
            _ => (Vec::new(), Some("empty path".to_string())),
        }
    }

    impl JsonPathWithMatcher {
        pub fn new(path: &str, matcher: Box<dyn JsonMatcher>) -> Self {
            let (segments, parse_error) = parse_single_path(path);
            Self {
                raw: path.to_string(),
                segments,
                matcher,
                parse_error,
            }
        }

        fn find_leaf<'a>(&self, value: &'a Value) -> Option<&'a Value> {
            let mut current = value;
            for seg in &self.segments {
                match (seg, current) {
                    (PathSegment::Field(name), Value::Object(map)) => {
                        current = map.get(name)?;
                    }
                    (PathSegment::Index(idx), Value::Array(arr)) => {
                        current = arr.get(*idx)?;
                    }
                    _ => return None,
                }
            }
            Some(current)
        }
    }

    impl Matcher<&Value> for JsonPathWithMatcher {
        fn matches(&self, value: &Value) -> MatcherResult {
            if self.parse_error.is_some() {
                return MatcherResult::NoMatch;
            }
            let Some(leaf) = self.find_leaf(value) else {
                return MatcherResult::NoMatch;
            };
            self.matcher.matches(leaf)
        }

        fn describe(&self, result: MatcherResult) -> Description {
            let path = &self.raw;
            match result {
                MatcherResult::Match => format!("has path `{path}` whose value matches").into(),
                MatcherResult::NoMatch => {
                    format!("has path `{path}` whose value does not match").into()
                }
            }
        }

        fn explain_match(&self, value: &Value) -> Description {
            if let Some(err) = &self.parse_error {
                return Description::new().text(format!("invalid path {err}"));
            }
            let Some(leaf) = self.find_leaf(value) else {
                return match value {
                    Value::Object(_) => Description::new()
                        .text(format!("missing path `{}`", format_path(&self.segments))),
                    _ => describe_json_type(value),
                };
            };
            self.matcher.explain_match(leaf)
        }
    }
}
