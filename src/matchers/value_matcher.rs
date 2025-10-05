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

#[deprecated(since = "0.2.0", note = "please use `json::primitive!` instead")]
#[macro_export]
#[doc(hidden)]
macro_rules! __json_value {
    ($matcher:expr) => {
        $crate::__json_primitive!($matcher)
    };
}

/// Matches a JSON value (string, number, or boolean) against the given matcher.
///
/// This macro enables matching specific primitive values inside a JSON structure
/// by delegating to a matcher for the corresponding Rust type. It supports:
/// - `String` values (e.g. `json::primitive!(eq("hello"))`)
/// - `Number` values as `i64` or `f64` (e.g. `json::primitive!(ge(0))`)
/// - `Boolean` values (e.g. `json::primitive!(eq(true))`)
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
/// verify_that!(data["active"], json::primitive!(eq(true)));
/// verify_that!(data["count"], json::primitive!(ge(0)));
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! __json_primitive {
    ($matcher:expr) => {
        $crate::matchers::__internal_unstable_do_not_depend_on_these::JsonValueMatcher::new(
            $matcher,
        )
    };
}
pub fn any_value()
-> crate::matchers::__internal_unstable_do_not_depend_on_these::JsonAnyValueMatcher {
    crate::matchers::__internal_unstable_do_not_depend_on_these::JsonAnyValueMatcher::new(
        |v| !v.is_null(),
        "any JSON value",
    )
}
pub fn any_non_null_value()
-> crate::matchers::__internal_unstable_do_not_depend_on_these::JsonAnyValueMatcher {
    crate::matchers::__internal_unstable_do_not_depend_on_these::JsonAnyValueMatcher::new(
        |v| !v.is_null(),
        "any non-null JSON value",
    )
}

pub fn is_null() -> crate::matchers::__internal_unstable_do_not_depend_on_these::JsonAnyValueMatcher
{
    crate::matchers::__internal_unstable_do_not_depend_on_these::JsonAnyValueMatcher::new(
        |v| v.is_null(),
        "any Null JSON value",
    )
}

pub fn any_string()
-> crate::matchers::__internal_unstable_do_not_depend_on_these::JsonAnyValueMatcher {
    crate::matchers::__internal_unstable_do_not_depend_on_these::JsonAnyValueMatcher::new(
        |v| v.is_string(),
        "any String JSON value",
    )
}

pub fn any_number()
-> crate::matchers::__internal_unstable_do_not_depend_on_these::JsonAnyValueMatcher {
    crate::matchers::__internal_unstable_do_not_depend_on_these::JsonAnyValueMatcher::new(
        |v| v.is_number(),
        "any Number JSON value",
    )
}

pub fn any_boolean()
-> crate::matchers::__internal_unstable_do_not_depend_on_these::JsonAnyValueMatcher {
    crate::matchers::__internal_unstable_do_not_depend_on_these::JsonAnyValueMatcher::new(
        |v| v.is_boolean(),
        "any Boolean JSON value",
    )
}

pub fn any_array()
-> crate::matchers::__internal_unstable_do_not_depend_on_these::JsonAnyValueMatcher {
    crate::matchers::__internal_unstable_do_not_depend_on_these::JsonAnyValueMatcher::new(
        |v| v.is_array(),
        "any Array JSON value",
    )
}
pub fn any_object()
-> crate::matchers::__internal_unstable_do_not_depend_on_these::JsonAnyValueMatcher {
    crate::matchers::__internal_unstable_do_not_depend_on_these::JsonAnyValueMatcher::new(
        |v| v.is_object(),
        "any Object JSON value",
    )
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
    pub struct JsonAnyValueMatcher {
        kind_check: fn(&Value) -> bool,
        description: &'static str,
    }

    impl JsonAnyValueMatcher {
        pub fn new(kind_check: fn(&Value) -> bool, description: &'static str) -> Self {
            Self {
                kind_check,
                description,
            }
        }
    }

    impl Matcher<&Value> for JsonAnyValueMatcher {
        fn matches(&self, actual: &Value) -> MatcherResult {
            if (self.kind_check)(actual) {
                MatcherResult::Match
            } else {
                MatcherResult::NoMatch
            }
        }

        fn describe(&self, _matcher_result: MatcherResult) -> Description {
            Description::new().text(self.description)
        }

        fn explain_match(&self, actual: &Value) -> Description {
            Description::new().text(format!("which is {actual}"))
        }
    }

    impl JsonMatcher for JsonAnyValueMatcher {}

    /// Marker trait for JSON-aware matchers.
    pub trait JsonMatcher: for<'a> Matcher<&'a Value> {}

    impl<M, T> JsonMatcher for JsonValueMatcher<M, T> where
        JsonValueMatcher<M, T>: for<'a> Matcher<&'a Value>
    {
    }
}
