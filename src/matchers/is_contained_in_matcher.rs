//! JSON matcher: `json_is_contained_in![...]`
//!
//! This provides a serde_json-aware adapter similar to googletest's
//! [`is_contained_in!`](https://docs.rs/googletest/latest/googletest/matchers/macro.is_contained_in.html)
//! but operating on `&serde_json::Value` arrays.
//!
//! # Semantics
//! For an **actual** `serde_json::Value::Array`, the matcher succeeds when **each**
//! actual array element matches **at least one** of the provided candidate matchers.
//! Extra candidates are allowed and ignored. Order does *not* matter.
//!
//! This variant does **not** enforce multiplicity: multiple actual elements may
//! match the same candidate matcher.
//!
//! # Examples
//! ```rust
//! use googletest::prelude::*;
//! use serde_json::json;
//! use googletest_serde_json::json::is_contained_in;
//!
//! let val = json!(["x","y"]);
//! assert_that!(
//!     val,
//!     is_contained_in![eq("y"), eq("x"), eq("z"), ]
//! );
//! ```
//!

use googletest::description::Description;
use googletest::matcher::{Matcher, MatcherBase, MatcherResult};
use serde_json::Value;

/// Macro to build a [`JsonIsContainedIn`] from element matchers.
///
/// Each argument should be a matcher over `&serde_json::Value`.
#[macro_export]
macro_rules! __json_is_contained_in {
    ([$($matcher:expr),* $(,)?]) => {{
        $crate::matchers::__internal_unstable_do_not_depend_on_these::JsonIsContainedIn::new(vec![
            $( Box::new($matcher) as Box<dyn for<'a> googletest::matcher::Matcher<&'a serde_json::Value>> ),*
        ])
    }};
    // Convenience: allow unbracketed list and forward to the bracketed arm.
    ($($matcher:expr),* $(,)?) => {{
        $crate::__json_is_contained_in!([$($matcher),*])
    }};
}

#[doc(hidden)]
pub mod internal {
    use super::*;

    /// Concrete matcher for `json_is_contained_in![...]` (hidden: use the macro).
    #[derive(MatcherBase)]
    pub struct JsonIsContainedIn {
        pub(crate) candidates: Vec<Box<dyn for<'a> Matcher<&'a Value>>>,
    }

    impl JsonIsContainedIn {
        /// Factory used by the `json_is_contained_in!` macro.
        pub fn new(candidates: Vec<Box<dyn for<'a> Matcher<&'a Value>>>) -> Self {
            Self { candidates }
        }
    }

    impl Matcher<&Value> for JsonIsContainedIn {
        fn matches(&self, actual: &Value) -> MatcherResult {
            match actual {
                Value::Array(arr) => {
                    for item in arr {
                        if !self
                            .candidates
                            .iter()
                            .any(|cand| cand.matches(item).is_match())
                        {
                            return MatcherResult::NoMatch;
                        }
                    }
                    MatcherResult::Match
                }
                _ => MatcherResult::NoMatch,
            }
        }

        fn explain_match(&self, actual: &Value) -> Description {
            match actual {
                Value::Array(arr) => {
                    // Find the first element that has no matching candidate and explain why.
                    for (idx, item) in arr.iter().enumerate() {
                        if !self
                            .candidates
                            .iter()
                            .any(|cand| cand.matches(item).is_match())
                        {
                            return format!(
                                "where element #{idx} is {item:?}, no candidate matcher accepted it"
                            )
                            .into();
                        }
                    }
                    // If we got here, size matched and all items matched at least one candidate.
                    "whose elements all match".into()
                }
                _ => "which is not a JSON array".into(),
            }
        }

        fn describe(&self, result: MatcherResult) -> Description {
            let inner: Description = self
                .candidates
                .iter()
                .map(|m| m.describe(MatcherResult::Match))
                .collect();

            let header = if result.is_match() {
                "is contained in the following JSON element set:"
            } else {
                "is not contained in the following JSON element set:"
            };

            format!("{header}\n{}", inner.enumerate().indent()).into()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::json;
    use googletest::prelude::*;
    use googletest::verify_that;
    use serde_json::json;

    #[test]
    fn matches_when_all_actual_elems_have_a_candidate() {
        let val = json!(["x", "y"]);
        assert_that!(val, json::is_contained_in![eq("z"), eq("x"), eq("y"),]);
    }

    #[test]
    fn passes_with_duplicates_when_candidate_is_reused() {
        // Multiplicity is not enforced; both "x" elements may match the same candidate.
        let val = json!(["x", "x"]);
        assert_that!(val, json::is_contained_in![eq("x")]);
    }

    #[test]
    fn fails_when_an_element_has_no_candidate() {
        let val = json!(["x", "y"]);
        let result = verify_that!(&val, json::is_contained_in![eq("y"),],);

        // Assert we get a readable error.
        assert_that!(
            result,
            err(displays_as(all![
                contains_substring("Value of: &val"),
                contains_substring("Expected: is contained in the following JSON element set:"),
                contains_substring(r#"0. is equal to "y""#),
                contains_substring("no candidate matcher accepted it"),
            ]))
        );
    }

    #[test]
    fn fails_when_not_array() {
        let val = json!({"x": 1});
        let result = verify_that!(&val, json::is_contained_in![eq(1)],);
        assert_that!(
            result,
            err(displays_as(contains_substring("which is not a JSON array")))
        );
    }
}
