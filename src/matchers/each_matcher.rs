/// Matches every element of a JSON array against the given matcher.
///
/// This expands the inner expression using `IntoJsonMatcher`,
/// allowing use of native googletest matchers (e.g. `gt(0)`, `starts_with("a")`)
/// as well as all `json::*` matchers.
///
/// # Examples
///
/// ```rust
/// use googletest::prelude::*;
/// use googletest_json_serde::json;
/// use serde_json::json as j;
///
/// assert_that!(j!([1, 2, 3]), json::each!(gt(0)));
/// assert_that!(j!(["ab", "ax"]), json::each!(starts_with("a")));
///
/// ```
///
/// Fails if:
/// - The value is not an array
/// - Any element fails the inner matcher
#[macro_export]
macro_rules! __json_each {
    ($inner:expr) => {
        $crate::matchers::__internal_unstable_do_not_depend_on_these::JsonEachMatcher::new(
            $crate::matchers::__internal_unstable_do_not_depend_on_these::IntoJsonMatcher::into_json_matcher($inner)
        )
    };
}

pub mod internal {
    use crate::matchers::__internal_unstable_do_not_depend_on_these::JsonMatcher;
    use googletest::description::Description;
    use googletest::matcher::{Matcher, MatcherBase, MatcherResult};
    use serde_json::Value;

    #[derive(MatcherBase)]
    pub struct JsonEachMatcher {
        inner: Box<dyn JsonMatcher>,
    }

    impl JsonEachMatcher {
        pub fn new(inner: Box<dyn JsonMatcher>) -> Self {
            Self { inner }
        }
    }

    impl JsonMatcher for JsonEachMatcher {}
    impl Matcher<&Value> for JsonEachMatcher {
        fn matches(&self, actual: &Value) -> MatcherResult {
            let arr = match actual {
                Value::Array(a) => a,
                _ => return MatcherResult::NoMatch,
            };
            for v in arr {
                if self.inner.matches(v) == MatcherResult::NoMatch {
                    return MatcherResult::NoMatch;
                }
            }
            MatcherResult::Match
        }

        fn describe(&self, result: MatcherResult) -> Description {
            match result {
                MatcherResult::Match => format!(
                    "JSON array where each element {}",
                    self.inner.describe(MatcherResult::Match)
                )
                .into(),
                MatcherResult::NoMatch => format!(
                    "JSON array where each element {}",
                    self.inner.describe(MatcherResult::NoMatch)
                )
                .into(),
            }
        }

        fn explain_match(&self, actual: &Value) -> Description {
            let arr = match actual {
                Value::Array(a) => a,
                _ => return Description::new().text("which is not a JSON array"),
            };
            for (i, v) in arr.iter().enumerate() {
                if self.inner.matches(v) == MatcherResult::NoMatch {
                    return format!(
                        "element #{} ({}) did not match: {}",
                        i,
                        v,
                        self.inner.explain_match(v)
                    )
                    .into();
                }
            }
            format!(
                "all {} elements matched: {}",
                arr.len(),
                self.inner.describe(MatcherResult::Match)
            )
            .into()
        }
    }
}
