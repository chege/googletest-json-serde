//! Utility matchers and macros for concise JSON assertions using googletest.

use googletest::description::Description;
use googletest::matcher::{Matcher, MatcherBase, MatcherResult};
use serde_json::Value;

#[doc(hidden)]
pub mod internal {
    use super::*;

    /// `JsonScalarMatcher` is an adapter matcher that extracts a scalar (string, number, or bool)
    /// from a JSON `Value` and delegates to an inner matcher operating on that scalar type.
    pub struct JsonScalarMatcher<M, T> {
        pub(crate) inner: M,
        pub(crate) phantom: std::marker::PhantomData<T>,
    }

    impl<M, T> JsonScalarMatcher<M, T> {
        /// Constructs a new `JsonScalarMatcher` adapter with the provided inner matcher.
        pub fn new(inner: M) -> Self {
            Self {
                inner,
                phantom: std::marker::PhantomData,
            }
        }
    }

    impl<M> MatcherBase for JsonScalarMatcher<M, String> {}
    impl<M> MatcherBase for JsonScalarMatcher<M, f64> {}
    impl<M> MatcherBase for JsonScalarMatcher<M, i64> {}
    impl<M> MatcherBase for JsonScalarMatcher<M, bool> {}

    impl<M> Matcher<&Value> for JsonScalarMatcher<M, String>
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
            Description::new()
                .text("JSON string that")
                .nested(self.inner.describe(r))
        }
        fn explain_match(&self, actual: &Value) -> Description {
            match actual {
                Value::String(s) => self.inner.explain_match(s),
                _ => Description::new().text("which is not a JSON string".to_string()),
            }
        }
    }

    impl<M> Matcher<&Value> for JsonScalarMatcher<M, i64>
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
            Description::new()
                .text("JSON integer that")
                .nested(self.inner.describe(r))
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

    impl<M> Matcher<&Value> for JsonScalarMatcher<M, f64>
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
            Description::new()
                .text("JSON number that")
                .nested(self.inner.describe(r))
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

    impl<M> Matcher<&Value> for JsonScalarMatcher<M, bool>
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
            Description::new()
                .text("JSON boolean that")
                .nested(self.inner.describe(r))
        }
        fn explain_match(&self, actual: &Value) -> Description {
            match actual {
                Value::Bool(b) => self.inner.explain_match(*b),
                _ => Description::new().text("which is not a JSON boolean"),
            }
        }
    }
}

/// Matches a JSON scalar (string, number, or boolean) against the given matcher,
/// allowing direct assertions on scalar values inside JSON.
pub fn json_scalar<M, T>(matcher: M) -> internal::JsonScalarMatcher<M, T> {
    internal::JsonScalarMatcher::new(matcher)
}

#[cfg(test)]
mod tests {
    use crate::json;
    use googletest::prelude::*;
    use serde_json::json;

    #[test]
    fn i64_match() {
        let val = json!(42);
        assert_that!(val, json::scalar(gt(41)));
    }

    #[test]
    fn i64_unmatch() {
        let val = json!(42);
        assert_that!(val, not(json::scalar(gt(99))));
    }

    #[test]
    fn string_match() {
        let val = json!("hello");
        assert_that!(val, json::scalar(starts_with("hello")));
    }

    #[test]
    fn string_unmatch() {
        let val = json!("hello");
        assert_that!(val, not(json::scalar(starts_with("world"))));
    }

    #[test]
    fn f64_match() {
        let val = json!(3.1);
        assert_that!(val, json::scalar(gt(2.1)));
    }

    #[test]
    fn f64_unmatch() {
        let val = json!(3.1);
        assert_that!(val, not(json::scalar(gt(4.1))));
    }

    #[test]
    fn bool_match() {
        let val = json!(true);
        assert_that!(val, json::scalar(is_true()));
    }

    #[test]
    fn bool_unmatch() {
        let val = json!(true);
        assert_that!(val, not(json::scalar(is_false())));
    }

    #[test]
    fn string_wrong_type() {
        let val = json!(123);
        if let Err(err) = verify_that!(val, json::scalar(starts_with("hello"))) {
            assert_that!(
                err.description,
                eq(
                    "Value of: val\nExpected: JSON string that\n  starts with prefix \"hello\"\nActual: Number(123),\n  which is not a JSON string"
                )
            );
        } else {
            fail!("expected failure but matcher reported success").unwrap();
        }
    }
}
