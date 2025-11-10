//! JSON matcher for arrays where element order does **not** matter.
//!
//! This mirrors the semantics of `googletest::matchers::unordered_elements_are`,
//! but works for `&serde_json::Value` arrays and delegates each element to
//! inner matchers you provide.

/// Matches a JSON array whose elements, in any order, have a 1:1 correspondence
/// with the provided matchers.
///
/// Each element in the input array must match exactly one of the given matchers,
/// and vice versa. Matching fails if the input is not an array, if the number of
/// elements and matchers differ, or if no perfect one-to-one mapping can be found.
///
/// # Example
///
/// This passes:
/// ```
/// # use googletest::prelude::*;
/// # use serde_json::json as j;
/// # use crate::googletest_json_serde::json;
/// let value = j!(["a", "b", "c"]);
/// assert_that!(
///     value,
///     json::unordered_elements_are![
///         j!("c"),
///         eq("a"),
///         starts_with("b"),
///     ]
/// );
/// ```
///
/// This fails because the element `"x"` does not match any expected element:
/// ```should_panic
/// # use googletest::prelude::*;
/// # use serde_json::json as j;
/// # use crate::googletest_json_serde::json;
/// let value = j!(["a", "x", "c"]);
/// assert_that!(
///     value,
///     json::unordered_elements_are![
///         eq("c"),
///         eq("a"),
///         eq("b"),
///     ]
/// );
/// ```
///
/// This fails because the input is not an array:
/// ```should_panic
/// # use googletest::prelude::*;
/// # use serde_json::json;
/// # use crate::googletest_json_serde::json;
/// let value = json!("not an array");
/// assert_that!(
///     value,
///     json::unordered_elements_are![
///         eq("a"),
///         eq("b"),
///     ]
/// );
/// ```
///
///
/// # Notes
///
///  - Both JSON-aware and native GoogleTest matchers (such as `starts_with`, `contains_substring`) can be used directly.
///  - Wrapping with `json::primitive!` is no longer needed.
///  - Direct `serde_json::Value` inputs (e.g. `json!(...)`) are supported and compared by structural equality.
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

/// Matches a JSON array that contains elements matched by the given matchers, in any order.
///
/// To match, each provided matcher must have a **distinct** corresponding element in the array.
/// There may be **additional** elements in the array that do not correspond to any matcher.
///
/// Put another way, `json::contains_each![...]` succeeds if there is a subset of the actual JSON
/// array that `json::unordered_elements_are![...]` would match.
///
/// The actual value must be a JSON array (`serde_json::Value::Array`). If the value is not an array,
/// or if any matcher has no unique matching element, the match fails.
///
/// # Examples
///
/// ```
/// # use googletest::prelude::*;
/// # use serde_json::json;
/// # use crate::googletest_json_serde::json;
/// # fn should_pass() -> Result<()> {
/// verify_that!(json!(["c", "b", "a"]), json::contains_each![eq("a"), eq("b")])?;   // Passes
/// verify_that!(json!(["x", "y", "y"]), json::contains_each![eq("y"), eq("x")])?;   // Passes
/// #     Ok(())
/// # }
/// # fn should_fail_1() -> Result<()> {
/// verify_that!(json!(["a"]), json::contains_each![eq("a"), eq("b")])?;             // Fails: array too small
/// #     Ok(())
/// # }
/// # fn should_fail_2() -> Result<()> {
/// verify_that!(json!(["a", "b", "c"]), json::contains_each![eq("a"), eq("z")])?;    // Fails: second matcher unmatched
/// #     Ok(())
/// # }
/// # fn should_fail_3() -> Result<()> {
/// verify_that!(json!(["x", "x"]), json::contains_each![eq("x"), eq("x"), eq("x")])?; // Fails: no 1-1 mapping
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail_1().unwrap_err();
/// # should_fail_2().unwrap_err();
/// # should_fail_3().unwrap_err();
/// ```
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
/// Matches a JSON array where every element matches one of the provided matchers.
///
/// This macro succeeds if:
/// - the input is a JSON array
/// - every element in the array matches exactly one matcher
/// - matchers are not reused
/// - extra matchers may be provided and left unmatched
/// - order does not matter
///
/// This macro fails if:
/// - the input is not a JSON array
/// - any element in the array fails to match all matchers
///
/// Accepts both bracketed (`json::is_contained_in!([ ... ])`) and unbracketed (`json::is_contained_in!(...)`) forms.
///
/// # Notes
///
///  - Both JSON-aware and native GoogleTest matchers (such as `starts_with`, `contains_substring`) can be used directly.
///  - Wrapping with `json::primitive!` is no longer needed.
///  - Direct `serde_json::Value` inputs (e.g. `json!(...)`) are supported and compared by structural equality.
///
/// # Example
/// ```
/// # use googletest::prelude::*;
/// # use serde_json::json as j;
/// # use crate::googletest_json_serde::json;
/// let value = j!(["a", "b", "c"]);
/// assert_that!(
///     value,
///     json::is_contained_in![eq("a"), eq("b"), eq("c"), eq("d")]
/// );
/// ```
///
/// # How it works
///
/// - Each matcher can match at most one element
/// - Extra matchers may remain unused
/// - Every element in the array must be matched
///
/// # Alias
///
/// This macro is re-exported as [`json::is_contained_in!`](crate::json::is_contained_in).
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
