/// Matches a JSON field that may be absent, null, or satisfy an inner matcher.
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json;
/// # use serde_json::json as j;
/// let value = j!({ "id": 42 });
/// assert_that!(
///     value,
///     json::pat!({
///         "id": eq(42),
///         "nickname": json::optional!("Bob"),
///         "alt": json::optional!(j!("Robert")),
///         "tag": json::optional!(starts_with("B"))
///     })
/// );
/// ```
///
/// # Supported Inputs
/// - Literal JSON-compatible values
/// - Direct `serde_json::Value`
/// - Native googletest matchers
#[macro_export]
#[doc(hidden)]
macro_rules! __json_optional {
    ($inner:expr) => {{
        $crate::matchers::__internal_unstable_do_not_depend_on_these::JsonOptionalMatcher::new(
            $crate::matchers::__internal_unstable_do_not_depend_on_these::IntoJsonMatcher::into_json_matcher($inner),
        )
    }};
}

pub mod internal {
    use crate::matchers::json_matcher::internal::JsonMatcher;
    use googletest::description::Description;
    use googletest::matcher::{Matcher, MatcherBase, MatcherResult};
    use serde_json::Value;

    #[derive(MatcherBase)]
    pub struct JsonOptionalMatcher {
        inner: Box<dyn JsonMatcher>,
    }

    impl JsonOptionalMatcher {
        pub fn new(inner: Box<dyn JsonMatcher>) -> Self {
            Self { inner }
        }
    }

    impl JsonMatcher for JsonOptionalMatcher {
        fn allows_missing(&self) -> bool {
            true
        }
    }

    impl Matcher<&Value> for JsonOptionalMatcher {
        fn matches(&self, actual: &Value) -> MatcherResult {
            if actual.is_null() {
                MatcherResult::Match
            } else {
                self.inner.matches(actual)
            }
        }

        fn describe(&self, result: MatcherResult) -> Description {
            match result {
                MatcherResult::Match => "is null or matches inner matcher".into(),
                MatcherResult::NoMatch => "neither null nor matches inner matcher".into(),
            }
        }

        fn explain_match(&self, actual: &Value) -> Description {
            if actual.is_null() {
                Description::new().text("which is null")
            } else {
                self.inner.explain_match(actual)
            }
        }
    }
}
