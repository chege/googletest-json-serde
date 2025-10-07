//! Utility matchers and macros for concise JSON assertions using googletest.

use crate::matchers::__internal_unstable_do_not_depend_on_these::JsonPredicateMatcher;

/// Matches JSON null values.
pub fn is_null() -> JsonPredicateMatcher {
    JsonPredicateMatcher::new(|v| v.is_null(), "JSON Null", "which is not JSON null")
}

/// Matches any JSON value except null.
pub fn any_value() -> JsonPredicateMatcher {
    JsonPredicateMatcher::new(
        |v| !v.is_null(),
        "any JSON Value",
        "which is not any JSON Value",
    )
}

/// Matches JSON string values.
pub fn is_string() -> JsonPredicateMatcher {
    JsonPredicateMatcher::new(
        |v| v.is_string(),
        "a JSON String",
        "which is not a JSON String",
    )
}

/// Matches JSON number values.
pub fn is_number() -> JsonPredicateMatcher {
    JsonPredicateMatcher::new(
        |v| v.is_number(),
        "a JSON Number",
        "which is not a JSON Number",
    )
}

/// Matches JSON boolean values.
pub fn is_boolean() -> JsonPredicateMatcher {
    JsonPredicateMatcher::new(
        |v| v.is_boolean(),
        "a JSON Boolean",
        "which is not a JSON Boolean",
    )
}

/// Matches JSON array values.
pub fn is_array() -> JsonPredicateMatcher {
    JsonPredicateMatcher::new(
        |v| v.is_array(),
        "a JSON Array",
        "which is not a JSON Array",
    )
}

/// Matches JSON object values.
pub fn is_object() -> JsonPredicateMatcher {
    JsonPredicateMatcher::new(
        |v| v.is_object(),
        "a JSON Object",
        "which is not a JSON Object",
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
            (self.predicate)(actual).into()
        }

        fn describe(&self, matcher_result: MatcherResult) -> Description {
            match matcher_result {
                Match => self.positive_description.into(),
                NoMatch => self.negative_description.into(),
            }
        }
        fn explain_match(&self, actual: &Value) -> Description {
            let kind = match actual {
                Value::String(_) => "a JSON String",
                Value::Number(_) => "a JSON Number",
                Value::Bool(_) => "a JSON Boolean",
                Value::Null => "JSON Null",
                Value::Array(_) => "a JSON Array",
                Value::Object(_) => "a JSON Object",
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
