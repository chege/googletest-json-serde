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
/// Example:
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
        $crate::matchers::__internal_unstable_do_not_depend_on_these::JsonIsContainedInMatcher::new(
                vec![
                    $( Box::new($matcher) as Box<dyn for<'a> googletest::matcher::Matcher<&'a serde_json::Value>> ),*
                ],
            )
    }};
    // Convenience: allow unbracketed list and forward to the bracketed arm.
    ($($matcher:expr),* $(,)?) => {{
        $crate::__json_is_contained_in!([$($matcher),*])
    }};
}

#[doc(hidden)]
pub mod internal {
    use googletest::description::Description;
    use googletest::matcher::{Matcher, MatcherBase, MatcherResult};
    use serde_json::Value;

    #[derive(MatcherBase)]
    pub struct JsonIsContainedInMatcher {
        elements: Vec<Box<dyn for<'a> Matcher<&'a Value>>>,
    }

    impl JsonIsContainedInMatcher {
        pub fn new(elements: Vec<Box<dyn for<'a> Matcher<&'a Value>>>) -> Self {
            Self { elements }
        }
    }

    impl Matcher<&Value> for JsonIsContainedInMatcher {
        fn matches(&self, actual: &Value) -> MatcherResult {
            let Value::Array(arr) = actual else {
                return MatcherResult::NoMatch;
            };

            let mut used = vec![false; self.elements.len()];
            for actual_elem in arr {
                let mut matched = false;
                for (i, matcher) in self.elements.iter().enumerate() {
                    if used[i] {
                        continue;
                    }
                    if matcher.matches(actual_elem).is_match() {
                        used[i] = true;
                        matched = true;
                        break;
                    }
                }
                if !matched {
                    return MatcherResult::NoMatch;
                }
            }
            MatcherResult::Match
        }

        fn describe(&self, result: MatcherResult) -> Description {
            let inner: Description = self
                .elements
                .iter()
                .map(|m| m.describe(MatcherResult::Match))
                .collect();
            let inner = inner.enumerate().indent();
            let header = if result.into() {
                "contains each of the following elements (in any order):"
            } else {
                "doesn't contain each of the following elements (in any order):"
            };
            format!("{header}\n{inner}").into()
        }

        fn explain_match(&self, actual: &Value) -> Description {
            match actual {
                Value::Array(arr) => {
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
                                "which has no element matching the expected element #{i}"
                            )
                            .into();
                        }
                    }
                    if arr.len() < self.elements.len() {
                        return format!(
                            "which has size {} (expected at least {})",
                            arr.len(),
                            self.elements.len()
                        )
                        .into();
                    }
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

                    format!("which does not have a subset match with the expected elements. The best match found was:\n{}", lines.join("\n")).into()
                }
                _ => "which is not a JSON array".into(),
            }
        }
    }
}
