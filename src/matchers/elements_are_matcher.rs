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
/// # Example
///
/// ```
/// # use googletest::prelude::*;
/// # use serde_json::json as j;
/// # use crate::googletest_json_serde::json;
/// let value = j!(["a", "b", "c"]);
/// assert_that!(
///     value,
///     json::elements_are![eq("a"), eq("b"), eq("c")]
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
///         json::elements_are![eq("x"), eq("y")],
///         json::elements_are![eq("z")]
///     ]
/// );
/// ```
///
/// # See also
///
/// [`googletest::matcher::Matcher`], [`crate::json::elements_are!`]
#[macro_export]
#[doc(hidden)]
macro_rules! __json_elements_are {
    // Preferred bracketed form: __json_elements_are!([ m1, m2, ... ])
    ([$($matcher:expr),* $(,)?]) => {{
        $crate::matchers::__internal_unstable_do_not_depend_on_these::JsonElementsAre::new(vec![
            $(Box::new($matcher) as Box<dyn for<'a> googletest::matcher::Matcher<&'a serde_json::Value>>),*
        ])
    }};
    // Convenience: allow unbracketed list and forward to the bracketed arm.
    ($($matcher:expr),* $(,)?) => {{
        $crate::__json_elements_are!([$($matcher),*])
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
        $crate::matchers::__internal_unstable_do_not_depend_on_these::JsonUnorderedElementsAreMatcher::new(
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
    use googletest::description::Description;
    use googletest::matcher::{Matcher, MatcherBase, MatcherResult};
    use serde_json::Value;

    #[doc(hidden)]
    #[derive(MatcherBase)]
    pub struct JsonElementsAre {
        elements: Vec<Box<dyn for<'a> Matcher<&'a Value>>>,
    }

    impl JsonElementsAre {
        pub fn new(elements: Vec<Box<dyn for<'a> Matcher<&'a Value>>>) -> Self {
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
