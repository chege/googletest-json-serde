//! Utility matchers and macros for concise JSON assertions using googletest.

use crate::json::__internal_unstable_do_not_depend_on_these;
use crate::matchers::__internal_unstable_do_not_depend_on_these::JsonPredicateMatcher;
use googletest::description::Description;
use serde_json::Value;

/// Creates a custom JSON matcher from an arbitrary predicate function.
///
/// This function allows defining ad-hoc JSON matchers inline by supplying a closure or function
/// that returns `true` for matching values.
/// The resulting matcher can optionally be extended with:
/// - `.with_description("expected", "not expected")` — to provide custom messages, and
/// - `.with_explain_fn(|v| Description::new().text(...))` — to describe mismatches dynamically.
///
/// # Example
/// ```
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json;
/// # use serde_json::json as j;
///
/// let matcher = json::predicate(|v| v.as_i64().map_or(false, |n| n > 0))
///     .with_description("a positive number", "a non-positive number");
///
/// verify_that!(j!(42), &matcher);
/// verify_that!(j!(-1), not(&matcher));
/// ```
///
/// Use this when no built-in matcher (like `is_string()` or `is_null()`) fits your case.
pub fn predicate<P>(
    predicate: P,
) -> JsonPredicateMatcher<
    P,
    __internal_unstable_do_not_depend_on_these::NoDescription,
    __internal_unstable_do_not_depend_on_these::NoDescription,
>
where
    P: Fn(&Value) -> bool,
{
    JsonPredicateMatcher::new(
        predicate,
        __internal_unstable_do_not_depend_on_these::NoDescription,
        __internal_unstable_do_not_depend_on_these::NoDescription,
    )
}
/// Matches any JSON value except null.
pub fn is_null() -> JsonPredicateMatcher<impl Fn(&Value) -> bool, &'static str, &'static str> {
    JsonPredicateMatcher::new(|v| v.is_null(), "JSON null", "which is not JSON null")
        .with_explain_fn(__internal_unstable_do_not_depend_on_these::describe_json_type)
}
/// Matches any JSON value except null.
pub fn is_not_null() -> JsonPredicateMatcher<impl Fn(&Value) -> bool, &'static str, &'static str> {
    JsonPredicateMatcher::new(|v| !v.is_null(), "not JSON null", "which is JSON null")
        .with_explain_fn(__internal_unstable_do_not_depend_on_these::describe_json_type)
}

/// Matches any JSON value except null.
#[deprecated(since = "0.2.2", note = "Use `is_not_null` instead")]
pub fn any_value() -> JsonPredicateMatcher<impl Fn(&Value) -> bool, &'static str, &'static str> {
    JsonPredicateMatcher::new(|v| !v.is_null(), "any JSON value", "is not any JSON value")
        .with_explain_fn(__internal_unstable_do_not_depend_on_these::describe_json_type)
}

/// Matches JSON string values.
pub fn is_string() -> JsonPredicateMatcher<impl Fn(&Value) -> bool, &'static str, &'static str> {
    JsonPredicateMatcher::new(
        |v| v.is_string(),
        "a JSON string",
        "which is not a JSON string",
    )
    .with_explain_fn(__internal_unstable_do_not_depend_on_these::describe_json_type)
}

/// Matches JSON number values.
pub fn is_number() -> JsonPredicateMatcher<impl Fn(&Value) -> bool, &'static str, &'static str> {
    JsonPredicateMatcher::new(
        |v| v.is_number(),
        "a JSON number",
        "which is not a JSON number",
    )
    .with_explain_fn(__internal_unstable_do_not_depend_on_these::describe_json_type)
}

/// Matches JSON boolean values.
pub fn is_boolean() -> JsonPredicateMatcher<impl Fn(&Value) -> bool, &'static str, &'static str> {
    JsonPredicateMatcher::new(
        |v| v.is_boolean(),
        "a JSON boolean",
        "which is not a JSON boolean",
    )
    .with_explain_fn(__internal_unstable_do_not_depend_on_these::describe_json_type)
}

/// Matches JSON array values.
pub fn is_array() -> JsonPredicateMatcher<impl Fn(&Value) -> bool, &'static str, &'static str> {
    JsonPredicateMatcher::new(
        |v| v.is_array(),
        "a JSON array",
        "which is not a JSON array",
    )
    .with_explain_fn(__internal_unstable_do_not_depend_on_these::describe_json_type)
}

/// Matches an empty JSON array (`[]`).
pub fn is_empty_array() -> JsonPredicateMatcher<impl Fn(&Value) -> bool, &'static str, &'static str>
{
    JsonPredicateMatcher::new(
        |v| v.as_array().is_some_and(|a| a.is_empty()),
        "an empty JSON array",
        "which is not an empty JSON array",
    )
    .with_explain_fn(|v| {
        if v.is_array() {
            Description::new().text("which is a non-empty JSON array")
        } else {
            __internal_unstable_do_not_depend_on_these::describe_json_type(v)
        }
    })
}

/// Matches JSON object values.
pub fn is_object() -> JsonPredicateMatcher<impl Fn(&Value) -> bool, &'static str, &'static str> {
    JsonPredicateMatcher::new(
        |v| v.is_object(),
        "a JSON object",
        "which is not a JSON object",
    )
    .with_explain_fn(__internal_unstable_do_not_depend_on_these::describe_json_type)
}

#[doc(hidden)]
pub mod internal {
    use googletest::description::Description;
    use googletest::matcher::MatcherResult::{Match, NoMatch};
    use googletest::matcher::{Matcher, MatcherBase, MatcherResult};
    use serde_json::Value;

    /// Trait for types that can provide a description string.
    pub trait PredicateDescription {
        fn to_description(self) -> String;
    }

    impl PredicateDescription for &'static str {
        fn to_description(self) -> String {
            self.to_string()
        }
    }

    impl PredicateDescription for String {
        fn to_description(self) -> String {
            self
        }
    }

    impl<F> PredicateDescription for F
    where
        F: Fn() -> String,
    {
        fn to_description(self) -> String {
            self()
        }
    }
    /// Sentinel type for missing descriptions.
    #[derive(Clone, Copy, Debug)]
    pub struct NoDescription;
    impl PredicateDescription for NoDescription {
        fn to_description(self) -> String {
            String::new()
        }
    }

    /// Type alias for the explain function to reduce type complexity.
    type ExplainFn = Box<dyn Fn(&Value) -> Description>;

    #[derive(MatcherBase)]
    pub struct JsonPredicateMatcher<P, D1 = NoDescription, D2 = NoDescription>
    where
        P: Fn(&Value) -> bool,
        D1: PredicateDescription,
        D2: PredicateDescription,
    {
        predicate: P,
        positive_description: D1,
        negative_description: D2,
        explain_fn: Option<ExplainFn>,
    }

    impl<P, D1, D2> JsonPredicateMatcher<P, D1, D2>
    where
        P: Fn(&Value) -> bool,
        D1: PredicateDescription,
        D2: PredicateDescription,
    {
        pub fn new(predicate: P, positive_description: D1, negative_description: D2) -> Self {
            Self {
                predicate,
                positive_description,
                negative_description,
                explain_fn: None,
            }
        }

        pub fn with_description<D1b, D2b>(
            self,
            positive_description: D1b,
            negative_description: D2b,
        ) -> JsonPredicateMatcher<P, D1b, D2b>
        where
            D1b: PredicateDescription,
            D2b: PredicateDescription,
        {
            JsonPredicateMatcher {
                predicate: self.predicate,
                positive_description,
                negative_description,
                explain_fn: self.explain_fn,
            }
        }

        pub fn with_explain_fn<F>(mut self, f: F) -> Self
        where
            F: Fn(&Value) -> Description + 'static,
        {
            self.explain_fn = Some(Box::new(f));
            self
        }
    }

    impl<P, D1, D2> Matcher<&Value> for JsonPredicateMatcher<P, D1, D2>
    where
        P: Fn(&Value) -> bool,
        D1: PredicateDescription + Clone,
        D2: PredicateDescription + Clone,
    {
        fn matches(&self, actual: &Value) -> MatcherResult {
            if (self.predicate)(actual) {
                Match
            } else {
                NoMatch
            }
        }

        fn describe(&self, result: MatcherResult) -> Description {
            let pos = self.positive_description.clone().to_description();
            let neg = self.negative_description.clone().to_description();

            match result {
                Match if pos.is_empty() => "matches predicate".into(),
                NoMatch if neg.is_empty() => "does not match predicate".into(),
                Match => pos.into(),
                NoMatch => neg.into(),
            }
        }

        fn explain_match(&self, actual: &Value) -> Description {
            if let Some(ref f) = self.explain_fn {
                return f(actual);
            }
            Description::new().text("which does not match the predicate")
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

    impl<P, D1, D2> IntoJsonMatcher<()> for JsonPredicateMatcher<P, D1, D2>
    where
        P: Fn(&Value) -> bool + 'static,
        D1: PredicateDescription + Clone + 'static,
        D2: PredicateDescription + Clone + 'static,
    {
        fn into_json_matcher(self) -> Box<dyn for<'a> Matcher<&'a Value>> {
            Box::new(self)
        }
    }

    pub fn describe_json_type(v: &Value) -> Description {
        match v {
            Value::Null => "which is a JSON null",
            Value::String(_) => "which is a JSON string",
            Value::Number(_) => "which is a JSON number",
            Value::Bool(_) => "which is a JSON boolean",
            Value::Array(_) => "which is a JSON array",
            Value::Object(_) => "which is a JSON object",
        }
        .into()
    }
}
