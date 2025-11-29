/// Matches the length of a JSON array against a literal or matcher.
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json;
/// # use serde_json::json as j;
/// verify_that!(j!(["a", "b", "c"]), json::len!(3));
/// verify_that!(j!(["a", "b", "c"]), json::len!(ge(2)));
/// verify_that!(j!(["a", "b", "c"]), json::len!(j!(3)));
/// assert_that!(j!(["a"]), not(json::len!(2)));
/// ```
///
/// # Errors
///
/// Fails when the value is not a JSON array.
///
/// # Supported Inputs
/// - Literal JSON-compatible values
/// - Direct `serde_json::Value`
/// - Native googletest matchers
#[macro_export]
#[doc(hidden)]
macro_rules! __json_len {
    ($lit:literal) => {{
        $crate::matchers::__internal_unstable_do_not_depend_on_these::JsonLenMatcher::new(
            $crate::matchers::__internal_unstable_do_not_depend_on_these::IntoJsonMatcher::into_json_matcher(
                googletest::matchers::eq($lit)
            ),
        )
    }};
    ($inner:expr) => {{
        $crate::matchers::__internal_unstable_do_not_depend_on_these::JsonLenMatcher::new(
            $crate::matchers::__internal_unstable_do_not_depend_on_these::IntoJsonMatcher::into_json_matcher($inner),
        )
    }};
}

pub mod internal {
    use crate::matchers::__internal_unstable_do_not_depend_on_these::JsonMatcher;
    use googletest::description::Description;
    use googletest::matcher::{Matcher, MatcherBase, MatcherResult};
    use serde_json::Value;

    /// A JSON-aware length matcher that works for arrays and strings,
    /// without requiring the type to implement IntoIterator.
    #[derive(MatcherBase)]
    pub struct JsonLenMatcher {
        inner: Box<dyn JsonMatcher>,
    }

    impl JsonLenMatcher {
        pub fn new(inner: Box<dyn JsonMatcher>) -> Self {
            Self { inner }
        }
    }

    impl Matcher<&Value> for JsonLenMatcher {
        fn matches(&self, value: &Value) -> MatcherResult {
            let len = match value {
                Value::Array(arr) => arr.len(),
                _ => return MatcherResult::NoMatch,
            };
            let as_value = Value::from(len);
            self.inner.matches(&as_value)
        }

        fn describe(&self, result: MatcherResult) -> Description {
            format!("has length, which {}", self.inner.describe(result)).into()
        }

        fn explain_match(&self, value: &Value) -> Description {
            match value {
                Value::Array(arr) => {
                    let len = arr.len();
                    let as_value = Value::from(len);
                    format!(
                        "which has length {}, {}",
                        len,
                        self.inner.explain_match(&as_value)
                    )
                    .into()
                }
                _ => Description::new().text("which is not a JSON array"),
            }
        }
    }

    impl JsonMatcher for JsonLenMatcher {}
}
