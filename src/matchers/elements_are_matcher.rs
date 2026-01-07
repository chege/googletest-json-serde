/// Matches a JSON array against a list of matchers in order.
///
/// The array length must equal the matcher count.
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use serde_json::json;
/// # use crate::googletest_json_serde::json as j;
/// let value = json!(["alex", "bart", "cucumberbatch"]);
/// assert_that!(
///     value,
///     j::elements_are![
///         "alex",
///         json!("bart"),
///         char_count(eq(13))
///     ]
/// );
/// ```
///
/// ```rust,should_panic
/// # use googletest::prelude::*;
/// # use serde_json::json;
/// # use crate::googletest_json_serde::json as j;
/// let value = json!(["cucumberbatch", "alex", "bart"]);
/// assert_that!(
///     value,
///     j::elements_are![
///         "alex",
///         json!("bart"),
///         char_count(eq(13))
///     ]
/// );
/// ```
///
/// # Errors
///
/// Fails when the value is not a JSON array or when lengths differ.
///
/// # Supported Inputs
/// - Literal JSON-compatible values
/// - Direct `serde_json::Value`
/// - Native googletest matchers
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
