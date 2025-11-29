//! JSON matchers that ignore element order in arrays.
//!
//! # Examples
//! ```rust
//! # use googletest::prelude::*;
//! # use googletest_json_serde::json;
//! # use serde_json::json as j;
//! assert_that!(
//!     j!(["b", "a", j!("c")]),
//!     json::unordered_elements_are!["a", j!("c"), starts_with("b")]
//! );
//! ```

/// Matches a JSON array whose elements pair one-to-one with the provided matchers, ignoring order.
///
/// The array length must equal the matcher count.
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json;
/// # use serde_json::json as j;
/// assert_that!(
///     j!(["a", "b", j!("c")]),
///     json::unordered_elements_are!["a", j!("c"), starts_with("b")]
/// );
/// ```
///
/// ```rust,should_panic
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json;
/// # use serde_json::json as j;
/// assert_that!(
///     j!(["a", "x", "c"]),
///     json::unordered_elements_are![eq("c"), eq("a"), eq("b")]
/// );
/// ```
///
/// # Supported Inputs
/// - Literal JSON-compatible values
/// - Direct `serde_json::Value`
/// - Native googletest matchers
#[macro_export]
#[doc(hidden)]
macro_rules! __json_unordered_elements_are {
    ($(,)?) => {{
        $crate::matchers::__internal_unstable_do_not_depend_on_these::
        JsonUnorderedElementsAreMatcher::new(
            vec![],
            $crate::matchers::__internal_unstable_do_not_depend_on_these::Requirements::PerfectMatch,
        )
    }};

    ($($matcher:expr),* $(,)?) => {{
        $crate::matchers::__internal_unstable_do_not_depend_on_these::
        JsonUnorderedElementsAreMatcher::new(
            vec![
                $(
                    $crate::matchers::__internal_unstable_do_not_depend_on_these::IntoJsonMatcher::into_json_matcher($matcher)
                ),*
            ],
            $crate::matchers::__internal_unstable_do_not_depend_on_these::Requirements::PerfectMatch,
        )
    }};
}

/// Matches a JSON array that contains distinct matches for each provided matcher, ignoring order.
///
/// Extra array elements are allowed.
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json;
/// # use serde_json::json as j;
/// verify_that!(
///     j!(["alpha", "bingo", "c"]),
///     json::contains_each!["c", j!("alpha"), starts_with("b")]
/// )
/// .unwrap();
/// ```
///
/// ```rust,should_panic
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json;
/// # use serde_json::json;
/// verify_that!(json!(["a"]), json::contains_each![eq("a"), eq("b")]).unwrap();
/// ```
///
/// # Supported Inputs
/// - Literal JSON-compatible values
/// - Direct `serde_json::Value`
/// - Native googletest matchers
#[macro_export]
#[doc(hidden)]
macro_rules! __json_contains_each {
    ([$($matcher:expr),* $(,)?]) => {{
        $crate::matchers::__internal_unstable_do_not_depend_on_these::
        JsonUnorderedElementsAreMatcher::new(
            vec![
                $(
                    $crate::matchers::__internal_unstable_do_not_depend_on_these::IntoJsonMatcher::into_json_matcher($matcher)
                ),*
            ],
            $crate::matchers::__internal_unstable_do_not_depend_on_these::Requirements::Superset,
        )
    }};
    // Convenience: allow unbracketed list and forward to the bracketed arm.
    ($($matcher:expr),* $(,)?) => {{
        $crate::__json_contains_each!([$($matcher),*])
    }};
}
/// Matches a JSON array where every element satisfies one of the provided matchers without reuse.
///
/// Matchers may remain unused; order is irrelevant.
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json;
/// # use serde_json::json as j;
/// assert_that!(
///     j!(["a", "b", j!("c")]),
///     json::is_contained_in!["a", j!("c"), starts_with("b"), eq("d")]
/// );
/// ```
///
/// ```rust,should_panic
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json;
/// # use serde_json::json as j;
/// assert_that!(
///     j!(["a", "x"]),
///     json::is_contained_in![eq("a"), eq("b")]
/// );
/// ```
///
/// # Supported Inputs
/// - Literal JSON-compatible values
/// - Direct `serde_json::Value`
/// - Native googletest matchers
#[macro_export]
#[doc(hidden)]
macro_rules! __json_is_contained_in {
    ([$($matcher:expr),* $(,)?]) => {{
        $crate::matchers::__internal_unstable_do_not_depend_on_these::JsonUnorderedElementsAreMatcher::new(
            vec![
                $(
                    $crate::matchers::__internal_unstable_do_not_depend_on_these::IntoJsonMatcher::into_json_matcher($matcher)
                ),*
            ],
            $crate::matchers::__internal_unstable_do_not_depend_on_these::Requirements::Subset,
        )
    }};
    // Convenience: allow unbracketed list and forward to the bracketed arm.
    ($($matcher:expr),* $(,)?) => {{
        $crate::__json_is_contained_in!([$($matcher),*])
    }};
}
#[doc(hidden)]
pub mod internal {
    use crate::matcher_support::match_matrix::internal::{MatchMatrix, Requirements};
    use crate::matchers::json_matcher::internal::JsonMatcher;
    use googletest::description::Description;
    use googletest::matcher::{Matcher, MatcherBase, MatcherResult};
    use serde_json::Value;

    #[doc(hidden)]
    #[derive(MatcherBase)]
    pub struct JsonUnorderedElementsAreMatcher {
        elements: Vec<Box<dyn JsonMatcher>>,
        requirements: Requirements,
    }
    impl JsonMatcher for JsonUnorderedElementsAreMatcher {}

    impl JsonUnorderedElementsAreMatcher {
        pub fn new(elements: Vec<Box<dyn JsonMatcher>>, requirements: Requirements) -> Self {
            Self {
                elements,
                requirements,
            }
        }
    }

    impl Matcher<&Value> for JsonUnorderedElementsAreMatcher {
        fn matches(&self, actual: &Value) -> MatcherResult {
            let Value::Array(actual_array) = actual else {
                return MatcherResult::NoMatch;
            };
            let matrix = MatchMatrix::generate(actual_array, &self.elements);
            matrix.is_match_for(self.requirements).into()
        }

        fn describe(&self, result: MatcherResult) -> Description {
            let inner: Description = self
                .elements
                .iter()
                .map(|m| m.describe(MatcherResult::Match))
                .collect();
            let inner = inner.enumerate().indent();
            let header = if result.into() {
                "contains JSON array elements matching in any order:"
            } else {
                "doesn't contain JSON array elements matching in any order:"
            };
            format!("{header}\n{inner}").into()
        }

        fn explain_match(&self, actual: &Value) -> Description {
            match actual {
                Value::Array(actual_array) => {
                    if let Some(size_msg) = self
                        .requirements
                        .explain_size_mismatch(actual_array, self.elements.len())
                    {
                        return size_msg;
                    }
                    let matrix = MatchMatrix::generate(actual_array, &self.elements);
                    if let Some(unmatchable) = matrix.explain_unmatchable(self.requirements) {
                        return unmatchable;
                    }
                    let best = matrix.find_best_match();
                    best.get_explanation(actual_array, &self.elements, self.requirements)
                        .unwrap_or("whose elements all match".into())
                }
                _ => "which is not a JSON array".into(),
            }
        }
    }
}
