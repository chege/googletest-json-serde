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
/// # use crate::googletest_serde_json::json;
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
        $crate::matchers::__internal_unstable_do_not_depend_on_these::JsonContainsEachMatcher::new(
            vec![
                $( Box::new($matcher) as Box<dyn for<'a> googletest::matcher::Matcher<&'a serde_json::Value>> ),*
            ],
        )
    }};
    // Convenience: allow unbracketed list and forward to the bracketed arm.
    ($($matcher:expr),* $(,)?) => {{
        $crate::__json_contains_each!([$($matcher),*])
    }};
}

#[doc(hidden)]
pub mod internal {
    use googletest::description::Description;
    use googletest::matcher::{Matcher, MatcherBase, MatcherResult};
    use serde_json::Value;

    /// Matches JSON arrays where all expected matchers match unique elements, regardless of order.
    ///
    /// This matcher implements a "contains each" semantic — each matcher must match exactly
    /// one element in the array, but the array may include additional unmatched elements.
    ///
    /// The actual value must be a JSON array. If not, the match fails.
    ///
    /// Matching is performed via backtracking to ensure one-to-one mapping.
    #[doc(hidden)]
    #[derive(MatcherBase)]
    pub struct JsonContainsEachMatcher {
        elements: Vec<Box<dyn for<'a> Matcher<&'a Value>>>,
    }

    impl JsonContainsEachMatcher {
        /// Constructs a new `JsonContainsEachMatcher` from the given list of matchers.
        ///
        /// Each matcher must match a unique element in the actual JSON array.
        /// The matcher allows extra elements in the actual array.
        pub fn new(elements: Vec<Box<dyn for<'a> Matcher<&'a Value>>>) -> Self {
            Self { elements }
        }
    }

    impl Matcher<&Value> for JsonContainsEachMatcher {
        fn matches(&self, actual: &Value) -> MatcherResult {
            let Value::Array(actual_array) = actual else {
                return MatcherResult::NoMatch;
            };

            if actual_array.len() < self.elements.len() {
                return MatcherResult::NoMatch;
            }

            let mut used_actual_elements = vec![false; actual_array.len()];

            fn find_matching_assignment<'a>(
                current_expected_index: usize,
                expected_matchers: &[Box<dyn for<'b> Matcher<&'b Value>>],
                actual_array: &'a [Value],
                used_actual_elements: &mut [bool],
            ) -> bool {
                if current_expected_index == expected_matchers.len() {
                    return true;
                }
                for actual_element_index in 0..actual_array.len() {
                    if used_actual_elements[actual_element_index] {
                        continue;
                    }
                    let current_matcher = &expected_matchers[current_expected_index];
                    let current_actual_element = &actual_array[actual_element_index];

                    if current_matcher.matches(current_actual_element).is_match() {
                        used_actual_elements[actual_element_index] = true;
                        if find_matching_assignment(
                            current_expected_index + 1,
                            expected_matchers,
                            actual_array,
                            used_actual_elements,
                        ) {
                            return true;
                        }
                        used_actual_elements[actual_element_index] = false;
                    }
                }
                false
            }

            if find_matching_assignment(0, &self.elements, actual_array, &mut used_actual_elements)
            {
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
            let header = {
                if result.into() {
                    "contains each of the following elements (in any order):"
                } else {
                    "doesn't contain each of the following elements (in any order):"
                }
            };
            format!("{header}\n{inner}").into()
        }

        fn explain_match(&self, actual: &Value) -> Description {
            match actual {
                Value::Array(actual_array) => {
                    let is_size_insufficient = actual_array.len() < self.elements.len();

                    // Ensure every expected matcher has at least one candidate in the actual array.
                    for (expected_index, expected_matcher) in self.elements.iter().enumerate() {
                        let mut has_matching_element = false;
                        for actual_value in actual_array {
                            if expected_matcher.matches(actual_value).is_match() {
                                has_matching_element = true;
                                break;
                            }
                        }
                        if !has_matching_element {
                            if is_size_insufficient {
                                return format!(
                                    "which has size {} (expected at least {}) and no element matching the expected element #{}",
                                    actual_array.len(),
                                    self.elements.len(),
                                    expected_index
                                )
                                    .into();
                            }
                            return format!(
                                "which has no element matching the expected element #{}",
                                expected_index
                            )
                            .into();
                        }
                    }
                    // Only after feasibility, check minimal size.
                    if is_size_insufficient {
                        return format!(
                            "which has size {} (expected at least {})",
                            actual_array.len(),
                            self.elements.len()
                        )
                        .into();
                    }

                    let mut used_expected_elements = vec![false; self.elements.len()];
                    let mut match_description_lines = Vec::new();
                    for (actual_element_index, actual_element_value) in
                        actual_array.iter().enumerate()
                    {
                        let mut matched_expected = None;
                        for (expected_element_index, expected_element_matcher) in
                            self.elements.iter().enumerate()
                        {
                            if used_expected_elements[expected_element_index] {
                                continue;
                            }
                            if expected_element_matcher
                                .matches(actual_element_value)
                                .is_match()
                            {
                                used_expected_elements[expected_element_index] = true;
                                matched_expected =
                                    Some((expected_element_index, expected_element_matcher));
                                break;
                            }
                        }
                        if let Some((matched_expected_index, matched_expected_matcher)) =
                            matched_expected
                        {
                            match_description_lines.push(format!(
                                "  Actual element {actual_element_value:?} at index {actual_element_index} matched expected element `{}` at index {matched_expected_index}.",
                                matched_expected_matcher.describe(MatcherResult::Match).to_string().trim()
                            ));
                        } else {
                            match_description_lines.push(format!(
                                "  Actual element {actual_element_value:?} at index {actual_element_index} did not match any remaining expected element."
                            ));
                        }
                    }
                    for (unmatched_expected_index, unmatched_expected_matcher) in
                        self.elements.iter().enumerate()
                    {
                        if !used_expected_elements[unmatched_expected_index] {
                            match_description_lines.push(format!(
                                "  Expected element `{}` at index {unmatched_expected_index} did not match any remaining actual element.",
                                unmatched_expected_matcher.describe(MatcherResult::Match).to_string().trim()
                            ));
                        }
                    }
                    format!(
                        "which does not have a superset match with the expected elements. The best match found was:\n{}",
                        match_description_lines.join("\n")
                    )
                    .into()
                }
                _ => "which is not a JSON array".into(),
            }
        }
    }
}
