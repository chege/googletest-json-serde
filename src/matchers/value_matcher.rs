//! Utility matchers and macros for concise JSON assertions using googletest.

/// Matches a JSON value (string, number, or boolean) against the given matcher.
///
/// This macro enables matching specific primitive values inside a JSON structure
/// by delegating to a matcher for the corresponding Rust type. It supports:
/// - `String` values (e.g. `json::value!(eq("hello"))`)
/// - `Number` values as `i64` or `f64` (e.g. `json::value!(ge(0))`)
/// - `Boolean` values (e.g. `json::value!(eq(true))`)
///
/// Fails if the value is not of the expected JSON type.
///
/// # Example
/// ```
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json;
/// # use serde_json::json as j;
/// let data = j!({"active": true, "count": 3});
///
/// verify_that!(data["active"], json::value!(eq(true)));
/// verify_that!(data["count"], json::value!(ge(0)));
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! __json_value {
    ($matcher:expr) => {
        $crate::matchers::__internal_unstable_do_not_depend_on_these::JsonValueMatcher::new(
            $matcher,
        )
    };
}

pub fn is_null() -> crate::matchers::__internal_unstable_do_not_depend_on_these::IsJsonNull {
    crate::matchers::__internal_unstable_do_not_depend_on_these::IsJsonNull
}

#[doc(hidden)]
pub mod internal {
    use googletest::description::Description;
    use googletest::matcher::{Matcher, MatcherBase, MatcherResult};
    use serde_json::Value;

    #[doc(hidden)]
    #[derive(MatcherBase)]
    pub struct JsonValueMatcher<M, T> {
        inner: M,
        phantom: std::marker::PhantomData<T>,
    }

    impl<M, T> JsonValueMatcher<M, T> {
        pub fn new(inner: M) -> Self {
            Self {
                inner,
                phantom: std::marker::PhantomData,
            }
        }
    }

    impl<M> Matcher<&Value> for JsonValueMatcher<M, String>
    where
        M: for<'a> Matcher<&'a str>,
    {
        fn matches(&self, actual: &Value) -> MatcherResult {
            match actual {
                Value::String(s) => self.inner.matches(s),
                _ => MatcherResult::NoMatch,
            }
        }
        fn describe(&self, r: MatcherResult) -> Description {
            self.inner.describe(r)
        }
        fn explain_match(&self, actual: &Value) -> Description {
            match actual {
                Value::String(s) => self.inner.explain_match(s),
                _ => Description::new().text("which is not a JSON string".to_string()),
            }
        }
    }

    impl<M> Matcher<&Value> for JsonValueMatcher<M, i64>
    where
        M: Matcher<i64>,
    {
        fn matches(&self, actual: &Value) -> MatcherResult {
            match actual {
                Value::Number(n) => n
                    .as_i64()
                    .map_or(MatcherResult::NoMatch, |i| self.inner.matches(i)),
                _ => MatcherResult::NoMatch,
            }
        }
        fn describe(&self, r: MatcherResult) -> Description {
            self.inner.describe(r)
        }
        fn explain_match(&self, actual: &Value) -> Description {
            match actual {
                Value::Number(n) => match n.as_i64() {
                    Some(i) => self.inner.explain_match(i),
                    None => Description::new().text(format!("number out of i64 range: {n}")),
                },
                _ => Description::new().text("which is not a JSON number"),
            }
        }
    }

    impl<M> Matcher<&Value> for JsonValueMatcher<M, f64>
    where
        M: Matcher<f64>,
    {
        fn matches(&self, actual: &Value) -> MatcherResult {
            match actual {
                Value::Number(n) => n
                    .as_f64()
                    .map_or(MatcherResult::NoMatch, |f| self.inner.matches(f)),
                _ => MatcherResult::NoMatch,
            }
        }
        fn describe(&self, r: MatcherResult) -> Description {
            self.inner.describe(r)
        }
        fn explain_match(&self, actual: &Value) -> Description {
            match actual {
                Value::Number(n) => match n.as_f64() {
                    Some(f) => self.inner.explain_match(f),
                    None => Description::new().text(format!("number not convertible to f64: {n}")),
                },
                _ => Description::new().text("which is not a JSON number"),
            }
        }
    }

    impl<M> Matcher<&Value> for JsonValueMatcher<M, bool>
    where
        M: Matcher<bool>,
    {
        fn matches(&self, actual: &Value) -> MatcherResult {
            match actual {
                Value::Bool(b) => self.inner.matches(*b),
                _ => MatcherResult::NoMatch,
            }
        }
        fn describe(&self, r: MatcherResult) -> Description {
            self.inner.describe(r)
        }
        fn explain_match(&self, actual: &Value) -> Description {
            match actual {
                Value::Bool(b) => self.inner.explain_match(*b),
                _ => Description::new().text("which is not a JSON boolean"),
            }
        }
    }

    #[derive(MatcherBase)]
    pub struct IsJsonNull;
    impl Matcher<&Value> for IsJsonNull {
        fn matches(&self, actual: &Value) -> MatcherResult {
            match actual {
                Value::Null => MatcherResult::Match,
                _ => MatcherResult::NoMatch,
            }
        }

        fn describe(&self, _: MatcherResult) -> Description {
            Description::new().text("JSON null")
        }

        fn explain_match(&self, actual: &Value) -> Description {
            match actual {
                Value::Null => Description::new().text("which is null"),
                _ => Description::new().text("which is not JSON null"),
            }
        }
    }
}
