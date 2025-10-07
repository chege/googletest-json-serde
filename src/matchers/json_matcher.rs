//! Utility matchers and macros for concise JSON assertions using googletest.

pub fn is_null() -> crate::matchers::__internal_unstable_do_not_depend_on_these::IsJsonNull {
    crate::matchers::__internal_unstable_do_not_depend_on_these::IsJsonNull
}

pub fn any_value()
-> crate::matchers::__internal_unstable_do_not_depend_on_these::JsonAnyValueMatcher {
    crate::matchers::__internal_unstable_do_not_depend_on_these::JsonAnyValueMatcher
}

#[doc(hidden)]
pub mod internal {
    use googletest::description::Description;
    use googletest::matcher::{Matcher, MatcherBase, MatcherResult};
    use serde_json::Value;

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

    #[derive(MatcherBase)]
    pub struct JsonAnyValueMatcher;

    impl JsonMatcher for JsonAnyValueMatcher {}

    impl Matcher<&Value> for JsonAnyValueMatcher {
        fn matches(&self, actual: &Value) -> MatcherResult {
            match actual {
                Value::Null => MatcherResult::NoMatch,
                _ => MatcherResult::Match,
            }
        }

        fn describe(&self, matcher_result: MatcherResult) -> Description {
            match matcher_result {
                MatcherResult::Match => Description::new().text("is any JSON value"),
                MatcherResult::NoMatch => Description::new().text("never matches"),
            }
        }

        fn explain_match(&self, actual: &Value) -> Description {
            Description::new().text(format!("which is {actual}"))
        }
    }

    /// Marker trait for JSON-aware matchers.
    pub trait JsonMatcher: for<'a> Matcher<&'a Value> {}

    impl JsonMatcher for IsJsonNull {}

    /// Trait for converting into a boxed JSON matcher.
    pub trait IntoJsonMatcher<T> {
        fn into_json_matcher(self) -> Box<dyn for<'a> Matcher<&'a Value>>;
    }

    impl<J> IntoJsonMatcher<()> for J
    where
        J: JsonMatcher + 'static,
    {
        fn into_json_matcher(self) -> Box<dyn for<'a> Matcher<&'a Value>> {
            Box::new(self)
        }
    }
}
