//! JSON matcher for arrays where element order does **not** matter.
//!
//! This mirrors the semantics of `googletest::matchers::unordered_elements_are`,
//! but works for `&serde_json::Value` arrays and delegates each element to
//! inner matchers you provide.

use googletest::description::Description;
use googletest::matcher::{Matcher, MatcherBase, MatcherResult};
use serde_json::Value;

#[doc(hidden)]
pub mod internal {
    use super::*;

    /// Matches a JSON array whose elements, **in any order**, satisfy the provided
    /// list of matchers one-to-one.
    ///
    /// If lengths differ, or if no perfect assignment exists between expected
    /// matchers and actual elements, the matcher does not match.
    pub struct JsonUnorderedElementsAre {
        elements: Vec<Box<dyn for<'a> Matcher<&'a Value>>>,
    }

    impl JsonUnorderedElementsAre {
        /// Create a new unordered-elements matcher from a vector of element matchers.
        pub fn new(elements: Vec<Box<dyn for<'a> Matcher<&'a Value>>>) -> Self {
            Self { elements }
        }
    }

    impl MatcherBase for JsonUnorderedElementsAre {}

    impl Matcher<&Value> for JsonUnorderedElementsAre {
        fn matches(&self, actual: &Value) -> MatcherResult {
            let Value::Array(arr) = actual else {
                return MatcherResult::NoMatch;
            };

            if arr.len() != self.elements.len() {
                return MatcherResult::NoMatch;
            }

            // Backtracking assignment: expected[i] -> some unique actual[j]
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
            format!(
                "{} JSON array elements in any order:\n{}",
                if result.into() { "has" } else { "doesn't have" },
                inner
            )
            .into()
        }

        fn explain_match(&self, actual: &Value) -> Description {
            match actual {
                Value::Array(arr) => {
                    // Build a simple per-actual mask of which expected matchers can match it.
                    // This is only for explanation; the matching above is definitive.
                    let mut lines = String::new();
                    use std::fmt::Write;
                    write!(&mut lines, "Actual: {:?},", actual).ok();

                    // Try to find one failing expected matcher and show why.
                    for (i, exp) in self.elements.iter().enumerate() {
                        let mut any = false;
                        for v in arr {
                            if exp.matches(v).is_match() {
                                any = true;
                                break;
                            }
                        }
                        if !any {
                            // Nothing in `arr` matched expected[i]; print its positive description.
                            write!(
                                &mut lines,
                                "\n  where no element matches expected #{i}: {}",
                                exp.describe(MatcherResult::Match)
                            )
                            .ok();
                            break;
                        }
                    }

                    lines.into()
                }
                _ => "Actual: not a JSON array".into(),
            }
        }
    }
}

/// Macro to build an unordered JSON array matcher from element matchers.
///
/// # Examples
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
#[macro_export]
macro_rules! __json_unordered_elements_are {
    ($($matcher:expr),* $(,)?) => {
        $crate::matchers::__internal_unstable_do_not_depend_on_these::JsonUnorderedElementsAre::new(vec![
            $(Box::new($matcher) as Box<dyn for<'a> googletest::matcher::Matcher<&'a serde_json::Value>>),*
        ])
    };
}

#[cfg(test)]
mod tests {
    use crate::json;
    use googletest::prelude::*;
    use serde_json::json;

    #[test]
    fn matches_same_multiset_different_order() {
        let val = json!(["x", "y", "z"]);
        assert_that!(
            val,
            json::unordered_elements_are![eq("z"), eq("x"), eq("y"),]
        );
    }

    #[test]
    fn fails_on_length_mismatch() {
        let val = json!(["x", "y"]);
        let result = verify_that!(
            &val,
            json::unordered_elements_are![eq("x"), eq("y"), eq("z"),],
        );
        assert_that!(
            result,
            err(displays_as(contains_substring(
                "where no element matches expected #2"
            )))
        );
    }

    #[test]
    fn nested_arrays_work() {
        let val = json!([["x", "y"], ["z"]]);
        assert_that!(
            val,
            json::unordered_elements_are![
                json::unordered_elements_are![eq("z")],
                json::unordered_elements_are![eq("y"), eq("x")],
            ]
        );
    }

    #[test]
    fn nested_arrays_unmatch_explains() {
        let val = json!([["x", "y"], ["z"]]);
        let res = verify_that!(
            &val,
            json::unordered_elements_are![
                json::unordered_elements_are![eq("x"), eq("z")],
                json::unordered_elements_are![eq("z")],
            ],
        );
        let err = res.unwrap_err();
        assert_that!(err.description, contains_substring("Actual: Array"));
        assert_that!(err.description, contains_substring("in any order"));
    }
}
