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
/// # Examples
///
/// This passes:
/// ```
/// # use googletest::prelude::*;
/// # use serde_json::json;
/// # use crate::googletest_serde_json::json;
/// let value = json!(["a", "b", "c"]);
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
/// This fails because the element `"x"` does not match any expected element:
/// ```should_panic
/// # use googletest::prelude::*;
/// # use serde_json::json;
/// # use crate::googletest_serde_json::json;
/// let value = json!(["a", "x", "c"]);
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
/// # use crate::googletest_serde_json::json;
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
/// # Alias
///
/// This macro is re-exported as [`json::unordered_elements_are!`](crate::json::unordered_elements_are).
#[macro_export]
#[doc(hidden)]
macro_rules! __json_unordered_elements_are {
    ($($matcher:expr),* $(,)?) => {
        $crate::matchers::__internal_unstable_do_not_depend_on_these::JsonUnorderedElementsAre::new(vec![
            $(Box::new($matcher) as Box<dyn for<'a> googletest::matcher::Matcher<&'a serde_json::Value>>),*
        ])
    };
}

#[doc(hidden)]
pub mod internal {
    use googletest::description::Description;
    use googletest::matcher::{Matcher, MatcherBase, MatcherResult};
    use serde_json::Value;

    /// Matches a JSON array whose elements, **in any order**, satisfy the provided
    /// list of matchers one-to-one.
    ///
    /// If lengths differ, or if no perfect assignment exists between expected
    /// matchers and actual elements, the matcher does not match.
    #[doc(hidden)]
    #[derive(MatcherBase)]
    pub struct JsonUnorderedElementsAre {
        elements: Vec<Box<dyn for<'a> Matcher<&'a Value>>>,
    }

    impl JsonUnorderedElementsAre {
        /// Create a new unordered-elements matcher from a vector of element matchers.
        /// Defaults to `Requirements::PerfectMatch` (1:1 mapping; equal sizes).
        pub fn new(elements: Vec<Box<dyn for<'a> Matcher<&'a Value>>>) -> Self {
            Self { elements }
        }
    }

    impl Matcher<&Value> for JsonUnorderedElementsAre {
        fn matches(&self, actual: &Value) -> MatcherResult {
            let Value::Array(arr) = actual else {
                return MatcherResult::NoMatch;
            };

            if arr.len() != self.elements.len() {
                return MatcherResult::NoMatch;
            }
            let n = arr.len();
            let mut used = vec![false; n];
            fn backtrack<'a>(
                i: usize,
                expected: &[Box<dyn for<'b> Matcher<&'b Value>>],
                actual: &'a [Value],
                used: &mut [bool],
            ) -> bool {
                if i == expected.len() {
                    return true;
                }
                for j in 0..actual.len() {
                    if used[j] {
                        continue;
                    }
                    if expected[i].matches(&actual[j]).is_match() {
                        used[j] = true;
                        if backtrack(i + 1, expected, actual, used) {
                            return true;
                        }
                        used[j] = false;
                    }
                }
                false
            }
            if backtrack(0, &self.elements, arr, &mut used) {
                MatcherResult::Match
            } else {
                MatcherResult::NoMatch
            }
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
                Value::Array(arr) => {
                    // Early size check: must match exactly.
                    if arr.len() != self.elements.len() {
                        return format!(
                            "which has size {} (expected {})",
                            arr.len(),
                            self.elements.len()
                        )
                        .into();
                    }
                    // For exact-size mismatches, prefer the detailed best-match explanation
                    // rather than returning feasibility shortcuts.
                    // Fall through to best-match block below.
                    // If we didn't return early, everything matched but backtracking failed.
                    // Insert best-match fallback explanation logic.
                    let best_match_explanation = |label: &str| -> Description {
                        // Greedy partial matching to build a readable explanation
                        let mut used = vec![false; self.elements.len()];
                        let mut lines: Vec<String> = Vec::new();
                        for (ai, av) in arr.iter().enumerate() {
                            let mut matched = None;
                            for (ei, em) in self.elements.iter().enumerate() {
                                if used[ei] {
                                    continue;
                                }
                                if em.matches(av).is_match() {
                                    used[ei] = true;
                                    matched = Some((ei, em));
                                    break;
                                }
                            }
                            if let Some((ei, em)) = matched {
                                lines.push(format!(
                                    "  Actual element {av:?} at index {ai} matched expected element `{}` at index {ei}.",
                                    em.describe(MatcherResult::Match).to_string().trim()
                                ));
                            } else {
                                lines.push(format!(
                                    "  Actual element {av:?} at index {ai} did not match any remaining expected element."
                                ));
                            }
                        }
                        for (ei, em) in self.elements.iter().enumerate() {
                            if !used[ei] {
                                lines.push(format!(
                                    "  Expected element `{}` at index {ei} did not match any remaining actual element.",
                                    em.describe(MatcherResult::Match).to_string().trim()
                                ));
                            }
                        }
                        let prefix = format!(
                            "which does not have a {label} match with the expected elements. The best match found was:\n"
                        );
                        format!("{}{}", prefix, lines.join("\n")).into()
                    };

                    best_match_explanation("perfect")
                }
                _ => "which is not a JSON array".into(),
            }
        }
    }
}
