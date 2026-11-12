/// Matches a JSON array when every element is equal to an expected value.
///
/// Like googletest's `subset_of`, this ignores order and duplicates on both
/// sides and compares values using `serde_json::Value` equality. It accepts
/// JSON-serializable values, not matchers. In contrast,
/// [`is_contained_in!`](crate::json::is_contained_in) accepts values or
/// matchers and requires a distinct expected entry for each actual element.
/// Non-arrays do not match. Comparison takes up to the product of the array
/// and expected lengths in value equality checks.
///
/// Values are converted using `serde_json::json!`; use `json!(null)` for null.
/// Conversion panics if a value cannot be serialized as JSON; non-finite
/// floats become JSON null, as with `serde_json::json!`.
///
/// ```
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json as j;
/// # use serde_json::json;
/// assert_that!(json!([1, 1, "x"]), j::subset_of![1, "x"]);
/// ```
///
/// ```compile_fail
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json as j;
/// let _ = j::subset_of![gt(1)];
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! __json_subset_of {
    ($($value:expr),* $(,)?) => {{
        $crate::matchers::__internal_unstable_do_not_depend_on_these::JsonSetMatcher::new(
            vec![$($crate::matchers::__internal_unstable_do_not_depend_on_these::json!($value)),*],
            $crate::matchers::__internal_unstable_do_not_depend_on_these::JsonSetRelation::Subset,
        )
    }};
}

/// Matches a JSON array when it contains every expected value.
///
/// Like googletest's `superset_of`, this ignores order and duplicates on both
/// sides and compares values using `serde_json::Value` equality. It accepts
/// JSON-serializable values, not matchers. In contrast,
/// [`contains_each!`](crate::json::contains_each) accepts values or matchers
/// and requires a distinct actual element for each expected entry.
/// Non-arrays do not match. Comparison takes up to the product of the array
/// and expected lengths in value equality checks.
///
/// Values are converted using `serde_json::json!`; use `json!(null)` for null.
/// Conversion panics if a value cannot be serialized as JSON; non-finite
/// floats become JSON null, as with `serde_json::json!`.
///
/// ```
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json as j;
/// # use serde_json::json;
/// assert_that!(json!([1, "x"]), j::superset_of![1, 1]);
/// ```
///
/// ```compile_fail
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json as j;
/// let _ = j::superset_of![gt(1)];
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! __json_superset_of {
    ($($value:expr),* $(,)?) => {{
        $crate::matchers::__internal_unstable_do_not_depend_on_these::JsonSetMatcher::new(
            vec![$($crate::matchers::__internal_unstable_do_not_depend_on_these::json!($value)),*],
            $crate::matchers::__internal_unstable_do_not_depend_on_these::JsonSetRelation::Superset,
        )
    }};
}

#[doc(hidden)]
pub mod internal {
    use crate::matchers::json_matcher::internal::{JsonMatcher, describe_json_type};
    use googletest::description::Description;
    use googletest::matcher::{Matcher, MatcherBase, MatcherResult};
    use serde_json::Value;

    pub enum Relation {
        Subset,
        Superset,
    }

    #[derive(MatcherBase)]
    pub struct JsonSetMatcher {
        expected: Vec<Value>,
        relation: Relation,
    }

    impl JsonSetMatcher {
        pub fn new(expected: Vec<Value>, relation: Relation) -> Self {
            Self { expected, relation }
        }
    }

    impl JsonMatcher for JsonSetMatcher {}

    impl Matcher<&Value> for JsonSetMatcher {
        fn matches(&self, actual: &Value) -> MatcherResult {
            let Some(actual) = actual.as_array() else {
                return MatcherResult::NoMatch;
            };
            match self.relation {
                Relation::Subset => actual.iter().all(|value| self.expected.contains(value)),
                Relation::Superset => self.expected.iter().all(|value| actual.contains(value)),
            }
            .into()
        }

        fn describe(&self, result: MatcherResult) -> Description {
            let relation = match self.relation {
                Relation::Subset => "subset",
                Relation::Superset => "superset",
            };
            let verb = if result.is_match() { "is" } else { "isn't" };
            format!(
                "{verb} a JSON array that is a {relation} of {:?}",
                self.expected
            )
            .into()
        }

        fn explain_match(&self, actual: &Value) -> Description {
            let Some(actual) = actual.as_array() else {
                return describe_json_type(actual);
            };
            match self.relation {
                Relation::Subset => {
                    let unexpected: Vec<_> = actual
                        .iter()
                        .enumerate()
                        .filter(|(_, value)| !self.expected.contains(value))
                        .map(|(index, value)| format!("{value:?} at #{index}"))
                        .collect();
                    if unexpected.is_empty() {
                        "whose elements are all expected".into()
                    } else {
                        format!("whose unexpected elements are {}", unexpected.join(", ")).into()
                    }
                }
                Relation::Superset => {
                    let missing: Vec<_> = self
                        .expected
                        .iter()
                        .filter(|value| !actual.contains(value))
                        .map(|value| format!("{value:?}"))
                        .collect();
                    if missing.is_empty() {
                        "whose expected elements are all present".into()
                    } else {
                        format!("whose missing elements are {}", missing.join(", ")).into()
                    }
                }
            }
        }
    }
}
