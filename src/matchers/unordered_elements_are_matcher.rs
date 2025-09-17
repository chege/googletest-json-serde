//! JSON matcher for arrays where element order does **not** matter.
//!
//! This mirrors the semantics of `googletest::matchers::unordered_elements_are`,
//! but works for `&serde_json::Value` arrays and delegates each element to
//! inner matchers you provide.

use googletest::description::Description;
use googletest::matcher::{Matcher, MatcherBase, MatcherResult};
use serde_json::Value;

// Add next to your other JSON array macros in unordered_elements_are_matcher.rs

/// Macro to build a `JsonUnorderedElementsAre` configured as **Superset**
/// (aka "contains each"): every provided matcher must match a **unique**
/// element of the actual JSON array, but the array may contain extra elements.
///
/// # Examples
/// ```
/// # use googletest::prelude::*;
/// # use serde_json::json;
/// # use crate::googletest_serde_json::json;
/// let value = json!(["a", "b", "c", "d"]);
/// assert_that!(
///     value,
///     json::contains_each![eq("a"), eq("c")]  // passes; "b","d" are extra
/// );
/// ```
#[macro_export]
macro_rules! __json_contains_each {
    ([$($matcher:expr),* $(,)?]) => {{
        $crate::matchers::__internal_unstable_do_not_depend_on_these::JsonUnorderedElementsAre::new_with_requirements(
            vec![
                $( Box::new($matcher) as Box<dyn for<'a> googletest::matcher::Matcher<&'a serde_json::Value>> ),*
            ],
            $crate::matchers::__internal_unstable_do_not_depend_on_these::Requirements::Superset,
        )
    }};
    // Convenience: allow unbracketed list and forward to the bracketed arm.
    ($($matcher:expr),* $(,)?) => {{
        $crate::__json_contains_each!([$($matcher),*])
    }};
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

/// Macro to build a [`JsonIsContainedIn`] from element matchers.
///
/// Each argument should be a matcher over `&serde_json::Value`.
#[macro_export]
macro_rules! __json_is_contained_in {
    ([$($matcher:expr),* $(,)?]) => {{
        $crate::matchers::__internal_unstable_do_not_depend_on_these::
            JsonUnorderedElementsAre::new_with_requirements(
                vec![
                    $( Box::new($matcher) as Box<dyn for<'a> googletest::matcher::Matcher<&'a serde_json::Value>> ),*
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
    use super::*;

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum Requirements {
        PerfectMatch,
        Subset,
        Superset,
    }

    /// Matches a JSON array whose elements, **in any order**, satisfy the provided
    /// list of matchers one-to-one.
    ///
    /// If lengths differ, or if no perfect assignment exists between expected
    /// matchers and actual elements, the matcher does not match.
    pub struct JsonUnorderedElementsAre {
        elements: Vec<Box<dyn for<'a> Matcher<&'a Value>>>,
        requirements: Requirements,
    }

    impl JsonUnorderedElementsAre {
        /// Create a new unordered-elements matcher from a vector of element matchers.
        /// Defaults to `Requirements::PerfectMatch` (1:1 mapping; equal sizes).
        pub fn new(elements: Vec<Box<dyn for<'a> Matcher<&'a Value>>>) -> Self {
            Self {
                elements,
                requirements: Requirements::PerfectMatch,
            }
        }

        /// Create with explicit matching requirements (`PerfectMatch`, `Subset`, or `Superset`).
        pub fn new_with_requirements(
            elements: Vec<Box<dyn for<'a> Matcher<&'a Value>>>,
            requirements: Requirements,
        ) -> Self {
            Self {
                elements,
                requirements,
            }
        }
    }

    impl MatcherBase for JsonUnorderedElementsAre {}

    impl Matcher<&Value> for JsonUnorderedElementsAre {
        fn matches(&self, actual: &Value) -> MatcherResult {
            let Value::Array(arr) = actual else {
                return MatcherResult::NoMatch;
            };

            match self.requirements {
                Requirements::PerfectMatch => {
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
                Requirements::Superset => {
                    if arr.len() < self.elements.len() {
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
                Requirements::Subset => {
                    if self.elements.len() < arr.len() {
                        return MatcherResult::NoMatch;
                    }
                    let m = self.elements.len();
                    let mut used = vec![false; m];
                    fn backtrack_subset<'a>(
                        i: usize,
                        actual: &'a [Value],
                        matchers: &[Box<dyn for<'b> Matcher<&'b Value>>],
                        used: &mut [bool],
                    ) -> bool {
                        if i == actual.len() {
                            return true;
                        }
                        for j in 0..matchers.len() {
                            if used[j] {
                                continue;
                            }
                            if matchers[j].matches(&actual[i]).is_match() {
                                used[j] = true;
                                if backtrack_subset(i + 1, actual, matchers, used) {
                                    return true;
                                }
                                used[j] = false;
                            }
                        }
                        false
                    }
                    if backtrack_subset(0, arr, &self.elements, &mut used) {
                        MatcherResult::Match
                    } else {
                        MatcherResult::NoMatch
                    }
                }
            }
        }

        fn describe(&self, result: MatcherResult) -> Description {
            let inner: Description = self
                .elements
                .iter()
                .map(|m| m.describe(MatcherResult::Match))
                .collect();
            let inner = inner.enumerate().indent();
            let header = match self.requirements {
                Requirements::PerfectMatch => {
                    if result.into() {
                        "has elements matching in any order:"
                    } else {
                        "doesn't have elements matching in any order:"
                    }
                }
                Requirements::Superset => {
                    if result.into() {
                        "contains each of the following elements (in any order):"
                    } else {
                        "doesn't contain each of the following elements (in any order):"
                    }
                }
                Requirements::Subset => {
                    if result.into() {
                        "is contained in the following element set:"
                    } else {
                        "is not contained in the following element set:"
                    }
                }
            };
            format!("{header}\n{inner}").into()
        }

        fn explain_match(&self, actual: &Value) -> Description {
            match actual {
                Value::Array(arr) => {
                    match self.requirements {
                        Requirements::PerfectMatch | Requirements::Superset => {
                            for (i, exp) in self.elements.iter().enumerate() {
                                let mut any = false;
                                for v in arr {
                                    if exp.matches(v).is_match() {
                                        any = true;
                                        break;
                                    }
                                }
                                if !any {
                                    return format!(
                                        "where no element matches expected #{i}: {}",
                                        exp.describe(MatcherResult::Match)
                                    )
                                    .into();
                                }
                            }
                        }
                        Requirements::Subset => {
                            for (i, v) in arr.iter().enumerate() {
                                let mut any = false;
                                for exp in &self.elements {
                                    if exp.matches(v).is_match() {
                                        any = true;
                                        break;
                                    }
                                }
                                if !any {
                                    return format!(
                                        "where element #{i} = {:?} had no candidate matcher: no candidate matcher accepted it",
                                        v
                                    ).into();
                                }
                            }
                        }
                    }
                    // If we didn't return early, everything matched but backtracking failed.
                    "whose elements all match".into()
                }
                _ => "which is not a JSON array".into(),
            }
        }
    }
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
    #[test]
    fn matches_when_all_actual_elems_have_a_candidate() {
        let val = json!(["x", "y"]);
        assert_that!(val, json::is_contained_in![eq("z"), eq("x"), eq("y"),]);
    }

    #[test]
    fn passes_with_duplicates_when_candidate_is_reused() {
        // In subset semantics there must be a 1\-to\-1 assignment from actual elements to matchers.
        // Here there are two actual elements but only one matcher, so it must FAIL.
        let val = json!(["x", "x"]);
        let result = verify_that!(&val, json::is_contained_in![eq("x")]);
        assert_that!(
            result,
            err(displays_as(contains_substring("whose elements all match")))
        );
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
                contains_substring("Expected: is contained in the following element set:"),
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
