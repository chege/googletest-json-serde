/// Matches a JSON array with elements that satisfy the given matchers, in order.
///
/// Each element of the JSON array is matched against a corresponding
/// [`Matcher`][googletest::matcher::Matcher]. The array must have the same length
/// as the list of matchers, and all matchers must succeed.
///
/// This macro supports two forms:
/// - Bracketed: `elements_are!([matcher1, matcher2, ...])`
/// - Unbracketed: `elements_are!(matcher1, matcher2, ...)`
///
/// Callers should prefer the public-facing [`json::elements_are!`](crate::json::elements_are!) macro.
///
/// # Examples
///
/// Basic usage:
/// ```
/// # use googletest::prelude::*;
/// # use serde_json::json as j;
/// # use crate::googletest_json_serde::json;
/// let value = j!(["alex", "bart", "cucumberbatch"]);
/// assert_that!(
///     value,
///     json::elements_are![
///         j!("alex"),
///         starts_with("b"),
///         char_count(eq(13))
///     ]
/// );
/// ```
///
/// Nested example:
/// ```
/// # use googletest::prelude::*;
/// # use serde_json::json as j;
/// # use crate::googletest_json_serde::json;
/// let value = j!([["x", "y"], ["z"]]);
/// assert_that!(
///     value,
///     json::elements_are![
///         json::elements_are![j!("x"), eq("y")],
///         json::elements_are![eq("z")]
///     ]
/// );
/// ```
///
/// # Notes
///
///  - Both JSON-aware and native GoogleTest matchers (such as `starts_with`, `contains_substring`) can be used directly.
///  - Wrapping with `json::primitive!` is no longer needed.
///  - Direct `serde_json::Value` inputs (e.g. `json!(...)`) are supported and compared by structural equality.
///  - On failure, the first mismatching index is reported.
#[macro_export]
#[doc(hidden)]
macro_rules! __json_elements_are {
    // Preferred bracketed form: __json_elements_are!([ m1, m2, ... ])
    ([$($matcher:expr),* $(,)?]) => {{
        $crate::matchers::__internal_unstable_do_not_depend_on_these::JsonElementsAre::new(vec![
            $(
                $crate::matchers::__internal_unstable_do_not_depend_on_these::IntoJsonMatcher::into_json_matcher($matcher)
            ),*
        ])
    }};
    // Convenience: allow unbracketed list and forward to the bracketed arm.
    ($($matcher:expr),* $(,)?) => {{
        $crate::__json_elements_are!([$($matcher),*])
    }};
}

#[doc(hidden)]
pub mod internal {
    use crate::matchers::json_matcher::internal::JsonMatcher;
    use googletest::description::Description;
    use googletest::matcher::{Matcher, MatcherBase, MatcherResult};
    use serde_json::Value;

    #[doc(hidden)]
    #[derive(MatcherBase)]
    pub struct JsonElementsAre {
        elements: Vec<Box<dyn JsonMatcher>>,
    }

    impl JsonMatcher for JsonElementsAre {}

    impl JsonElementsAre {
        pub fn new(elements: Vec<Box<dyn JsonMatcher>>) -> Self {
            Self { elements }
        }
    }

    impl Matcher<&Value> for JsonElementsAre {
        fn matches(&self, actual: &Value) -> MatcherResult {
            match actual {
                Value::Array(arr) => {
                    if arr.len() != self.elements.len() {
                        return MatcherResult::NoMatch;
                    }
                    for (item, matcher) in arr.iter().zip(&self.elements) {
                        if matcher.matches(item).is_no_match() {
                            return MatcherResult::NoMatch;
                        }
                    }
                    MatcherResult::Match
                }
                _ => MatcherResult::NoMatch,
            }
        }

        fn describe(&self, result: MatcherResult) -> Description {
            let verb = if result.into() { "has" } else { "doesn't have" };
            let inner = self
                .elements
                .iter()
                .map(|m| m.describe(MatcherResult::Match))
                .collect::<Description>()
                .enumerate()
                .indent();

            format!("{verb} JSON array elements:\n{inner}").into()
        }

        fn explain_match(&self, actual: &Value) -> Description {
            match actual {
                Value::Array(arr) => {
                    let mut mismatches = Vec::new();
                    let actual_len = arr.len();
                    let expected_len = self.elements.len();

                    for (index, (item, matcher)) in arr.iter().zip(&self.elements).enumerate() {
                        if matcher.matches(item).is_no_match() {
                            mismatches.push(format!(
                                "element #{index} is {item:?}, {}",
                                matcher.explain_match(item)
                            ));
                        }
                    }

                    if mismatches.is_empty() {
                        if actual_len == expected_len {
                            "whose elements all match".into()
                        } else {
                            format!("whose size is {}", actual_len).into()
                        }
                    } else if mismatches.len() == 1 {
                        let description = mismatches.into_iter().collect::<Description>();
                        format!("where {description}").into()
                    } else {
                        let description = mismatches.into_iter().collect::<Description>();
                        format!("where:\n{}", description.bullet_list().indent()).into()
                    }
                }
                _ => Description::new().text("where the type is not array".to_string()),
            }
        }
    }
}
