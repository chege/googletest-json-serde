use googletest::matcher::Matcher;
use serde_json::Value;

/// Matches every top-level key of a JSON object against the same matcher.
///
/// Keys are collected as `String` values and matched with googletest's native
/// [`each`](googletest::matchers::each) matcher.
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json as j;
/// # use serde_json::json;
/// assert_that!(
///     json!({"usr_id": 1, "usr_name": "Nadja"}),
///     j::each_key(starts_with("usr_"))
/// );
/// ```
///
/// # Errors
///
/// Fails when the value is not a JSON object or when any key fails the matcher.
pub fn each_key<M>(inner: M) -> internal::JsonEachKeyMatcher<M>
where
    M: for<'a> Matcher<&'a String>,
{
    internal::JsonEachKeyMatcher::new(inner)
}

/// Matches every top-level value of a JSON object against the same JSON matcher.
///
/// Values are collected as `serde_json::Value` values and matched with
/// googletest's native [`each`](googletest::matchers::each) matcher.
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json as j;
/// # use serde_json::json;
/// assert_that!(
///     json!({"hp": 10, "mp": 20}),
///     j::each_value(j::as_i64(gt(0)))
/// );
/// ```
///
/// # Errors
///
/// Fails when the value is not a JSON object or when any value fails the matcher.
pub fn each_value<M>(inner: M) -> internal::JsonEachValueMatcher<M>
where
    M: for<'a> Matcher<&'a Value>,
{
    internal::JsonEachValueMatcher::new(inner)
}

#[doc(hidden)]
pub mod internal {
    use crate::matchers::json_matcher::internal::JsonMatcher;
    use googletest::description::Description;
    use googletest::matcher::{Matcher, MatcherBase, MatcherResult};
    use googletest::matchers::each;
    use serde_json::Value;

    #[derive(MatcherBase)]
    pub struct JsonEachKeyMatcher<M> {
        inner: M,
    }

    impl<M> JsonEachKeyMatcher<M> {
        pub fn new(inner: M) -> Self {
            Self { inner }
        }
    }

    #[derive(MatcherBase)]
    pub struct JsonEachValueMatcher<M> {
        inner: M,
    }

    impl<M> JsonEachValueMatcher<M> {
        pub fn new(inner: M) -> Self {
            Self { inner }
        }
    }

    impl<M> JsonMatcher for JsonEachKeyMatcher<M> where Self: for<'a> Matcher<&'a Value> {}
    impl<M> JsonMatcher for JsonEachValueMatcher<M> where Self: for<'a> Matcher<&'a Value> {}

    impl<M> Matcher<&Value> for JsonEachKeyMatcher<M>
    where
        M: for<'a> Matcher<&'a String>,
    {
        fn matches(&self, actual: &Value) -> MatcherResult {
            let Some(object) = actual.as_object() else {
                return MatcherResult::NoMatch;
            };
            let keys = object.keys().cloned().collect::<Vec<_>>();
            each(&self.inner).matches(&keys)
        }

        fn describe(&self, result: MatcherResult) -> Description {
            match result {
                MatcherResult::Match => format!(
                    "is a JSON object whose every top-level key {}",
                    self.inner.describe(MatcherResult::Match)
                ),
                MatcherResult::NoMatch => format!(
                    "is not a JSON object or has a top-level key that {}",
                    self.inner.describe(MatcherResult::NoMatch)
                ),
            }
            .into()
        }

        fn explain_match(&self, actual: &Value) -> Description {
            let Some(object) = actual.as_object() else {
                return expected_object_description(actual);
            };
            let keys = object.keys().cloned().collect::<Vec<_>>();
            each(&self.inner).explain_match(&keys)
        }
    }

    impl<M> Matcher<&Value> for JsonEachValueMatcher<M>
    where
        M: for<'a> Matcher<&'a Value>,
    {
        fn matches(&self, actual: &Value) -> MatcherResult {
            let Some(object) = actual.as_object() else {
                return MatcherResult::NoMatch;
            };
            let values = object.values().cloned().collect::<Vec<_>>();
            each(&self.inner).matches(&values)
        }

        fn describe(&self, result: MatcherResult) -> Description {
            match result {
                MatcherResult::Match => format!(
                    "is a JSON object whose every top-level value {}",
                    self.inner.describe(MatcherResult::Match)
                ),
                MatcherResult::NoMatch => format!(
                    "is not a JSON object or has a top-level value that {}",
                    self.inner.describe(MatcherResult::NoMatch)
                ),
            }
            .into()
        }

        fn explain_match(&self, actual: &Value) -> Description {
            let Some(object) = actual.as_object() else {
                return expected_object_description(actual);
            };
            let values = object.values().cloned().collect::<Vec<_>>();
            each(&self.inner).explain_match(&values)
        }
    }

    fn expected_object_description(value: &Value) -> Description {
        format!("expected JSON object, but found {}", json_type_name(value)).into()
    }

    fn json_type_name(value: &Value) -> &'static str {
        match value {
            Value::Null => "JSON null",
            Value::Bool(_) => "JSON boolean",
            Value::Number(_) => "JSON number",
            Value::String(_) => "JSON string",
            Value::Array(_) => "JSON array",
            Value::Object(_) => "JSON object",
        }
    }
}
