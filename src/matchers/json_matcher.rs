//! Utility matchers and macros for concise JSON assertions using googletest.

use crate::matchers::__internal_unstable_do_not_depend_on_these::JsonPredicateMatcher;

/// Matches JSON null values.
pub fn is_null() -> JsonPredicateMatcher {
    JsonPredicateMatcher::new(|v| v.is_null(), "JSON null", "which is not JSON null")
}

/// Matches any JSON value except null.
pub fn any_value() -> JsonPredicateMatcher {
    JsonPredicateMatcher::new(
        |v| !v.is_null(),
        "any JSON value",
        "which is not any JSON value",
    )
}

/// Matches JSON string values.
pub fn is_string() -> JsonPredicateMatcher {
    JsonPredicateMatcher::new(
        |v| v.is_string(),
        "a JSON string",
        "which is not a JSON string",
    )
}

/// Matches JSON number values.
pub fn is_number() -> JsonPredicateMatcher {
    JsonPredicateMatcher::new(
        |v| v.is_number(),
        "a JSON number",
        "which is not a JSON number",
    )
}

/// Matches JSON boolean values.
pub fn is_boolean() -> JsonPredicateMatcher {
    JsonPredicateMatcher::new(
        |v| v.is_boolean(),
        "a JSON boolean",
        "which is not a JSON boolean",
    )
}

/// Matches JSON array values.
pub fn is_array() -> JsonPredicateMatcher {
    JsonPredicateMatcher::new(
        |v| v.is_array(),
        "a JSON array",
        "which is not a JSON array",
    )
}

/// Matches JSON object values.
pub fn is_object() -> JsonPredicateMatcher {
    JsonPredicateMatcher::new(
        |v| v.is_object(),
        "a JSON object",
        "which is not a JSON object",
    )
}

#[doc(hidden)]
pub mod internal {
    use googletest::description::Description;
    use googletest::matcher::MatcherResult::{Match, NoMatch};
    use googletest::matcher::{Matcher, MatcherBase, MatcherResult};
    use serde_json::Value;

    #[derive(MatcherBase)]
    pub struct JsonPredicateMatcher {
        predicate: fn(&Value) -> bool,
        positive_description: &'static str,
        negative_description: &'static str,
    }
    impl JsonMatcher for JsonPredicateMatcher {}

    impl JsonPredicateMatcher {
        pub fn new(
            predicate: fn(&Value) -> bool,
            positive_description: &'static str,
            negative_description: &'static str,
        ) -> Self {
            Self {
                predicate,
                positive_description,
                negative_description,
            }
        }
    }
    impl Matcher<&Value> for JsonPredicateMatcher {
        fn matches(&self, actual: &Value) -> MatcherResult {
            match (self.predicate)(actual) {
                true => Match,
                false => NoMatch,
            }
        }

        fn describe(&self, matcher_result: MatcherResult) -> Description {
            match matcher_result {
                Match => self.positive_description.into(),
                NoMatch => self.negative_description.into(),
            }
        }
        fn explain_match(&self, actual: &Value) -> Description {
            let kind = match actual {
                Value::String(_) => "a JSON string",
                Value::Number(_) => "a JSON number",
                Value::Bool(_) => "a JSON boolean",
                Value::Null => "a JSON null",
                Value::Array(_) => "a JSON array",
                Value::Object(_) => "a JSON object",
            };
            Description::new().text(format!("which is {kind}"))
        }
    }

    /// Marker trait for JSON-aware matchers.
    pub trait JsonMatcher: for<'a> Matcher<&'a Value> {}

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
