//! JSON predicate matchers for googletest assertions.
//!
//! # Examples
//! ```rust
//! # use googletest::prelude::*;
//! # use googletest_json_serde::json;
//! # use serde_json::json as j;
//! assert_that!(j!("hi"), json::is_string());
//! ```

use crate::matchers::__internal_unstable_do_not_depend_on_these;
use crate::matchers::__internal_unstable_do_not_depend_on_these::JsonPredicateMatcher;
use googletest::description::Description;
use serde_json::Value;

/// Builds a JSON matcher from an arbitrary predicate function.
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json;
/// # use serde_json::json as j;
/// let positive = json::predicate(|v| v.as_i64().is_some_and(|n| n > 0));
/// verify_that!(j!(42), &positive);
/// assert_that!(j!(-1), not(&positive));
/// ```
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
/// Matches JSON null values.
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json;
/// # use serde_json::json as j;
/// assert_that!(j!(null), json::is_null());
/// assert_that!(j!("value"), not(json::is_null()));
/// ```
pub fn is_null() -> JsonPredicateMatcher<impl Fn(&Value) -> bool, &'static str, &'static str> {
    JsonPredicateMatcher::new(|v| v.is_null(), "JSON null", "which is not JSON null")
        .with_explain_fn(__internal_unstable_do_not_depend_on_these::describe_json_type)
}
/// Matches JSON values that are not null.
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json;
/// # use serde_json::json as j;
/// assert_that!(j!("text"), json::is_not_null());
/// assert_that!(j!(null), not(json::is_not_null()));
/// ```
pub fn is_not_null() -> JsonPredicateMatcher<impl Fn(&Value) -> bool, &'static str, &'static str> {
    JsonPredicateMatcher::new(|v| !v.is_null(), "not JSON null", "which is JSON null")
        .with_explain_fn(__internal_unstable_do_not_depend_on_these::describe_json_type)
}

/// Matches JSON values that are not null.
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json;
/// # use serde_json::json as j;
/// assert_that!(j!("ok"), json::any_value());
/// assert_that!(j!(null), not(json::any_value()));
/// ```
#[deprecated(since = "0.2.2", note = "Use `is_not_null` instead")]
pub fn any_value() -> JsonPredicateMatcher<impl Fn(&Value) -> bool, &'static str, &'static str> {
    JsonPredicateMatcher::new(|v| !v.is_null(), "any JSON value", "is not any JSON value")
        .with_explain_fn(__internal_unstable_do_not_depend_on_these::describe_json_type)
}

/// Matches JSON string values.
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json;
/// # use serde_json::json as j;
/// assert_that!(j!("hi"), json::is_string());
/// assert_that!(j!(true), not(json::is_string()));
/// ```
pub fn is_string() -> JsonPredicateMatcher<impl Fn(&Value) -> bool, &'static str, &'static str> {
    JsonPredicateMatcher::new(
        |v| v.is_string(),
        "a JSON string",
        "which is not a JSON string",
    )
    .with_explain_fn(__internal_unstable_do_not_depend_on_these::describe_json_type)
}

/// Matches JSON number values.
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json;
/// # use serde_json::json as j;
/// assert_that!(j!(3.14), json::is_number());
/// assert_that!(j!("three"), not(json::is_number()));
/// ```
pub fn is_number() -> JsonPredicateMatcher<impl Fn(&Value) -> bool, &'static str, &'static str> {
    JsonPredicateMatcher::new(
        |v| v.is_number(),
        "a JSON number",
        "which is not a JSON number",
    )
    .with_explain_fn(__internal_unstable_do_not_depend_on_these::describe_json_type)
}

/// Matches JSON numbers that are integers.
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json;
/// # use serde_json::json as j;
/// assert_that!(j!(42), json::is_integer());
/// assert_that!(j!(3.14), not(json::is_integer()));
/// ```
pub fn is_integer() -> JsonPredicateMatcher<impl Fn(&Value) -> bool, &'static str, &'static str> {
    JsonPredicateMatcher::new(
        |v| matches!(v, Value::Number(n) if n.is_i64() || n.is_u64()),
        "an integer JSON number",
        "which is not an integer JSON number",
    )
    .with_explain_fn(|v| {
        if matches!(v, Value::Number(_)) {
            Description::new().text("which is a non-integer JSON number")
        } else {
            __internal_unstable_do_not_depend_on_these::describe_json_type(v)
        }
    })
}

/// Matches JSON numbers whose fractional part is zero (e.g., `2` or `2.0`).
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json;
/// # use serde_json::json as j;
/// assert_that!(j!(2.0), json::is_whole_number());
/// assert_that!(j!(2.5), not(json::is_whole_number()));
/// ```
pub fn is_whole_number() -> JsonPredicateMatcher<impl Fn(&Value) -> bool, &'static str, &'static str>
{
    JsonPredicateMatcher::new(
        |v| match v {
            Value::Number(n) => {
                if n.is_i64() || n.is_u64() {
                    true
                } else {
                    n.as_f64()
                        .is_some_and(|f| f.is_finite() && f.fract() == 0.0)
                }
            }
            _ => false,
        },
        "a JSON number with no fractional part",
        "which is not a JSON number with no fractional part",
    )
    .with_explain_fn(|v| {
        if matches!(v, Value::Number(_)) {
            Description::new().text("which is a JSON number with a fractional part")
        } else {
            __internal_unstable_do_not_depend_on_these::describe_json_type(v)
        }
    })
}

/// Matches JSON boolean values.
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json;
/// # use serde_json::json as j;
/// assert_that!(j!(true), json::is_boolean());
/// assert_that!(j!(0), not(json::is_boolean()));
/// ```
pub fn is_boolean() -> JsonPredicateMatcher<impl Fn(&Value) -> bool, &'static str, &'static str> {
    JsonPredicateMatcher::new(
        |v| v.is_boolean(),
        "a JSON boolean",
        "which is not a JSON boolean",
    )
    .with_explain_fn(__internal_unstable_do_not_depend_on_these::describe_json_type)
}

/// Matches the JSON boolean `true` value.
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json;
/// # use serde_json::json as j;
/// assert_that!(j!(true), json::is_true());
/// assert_that!(j!(false), not(json::is_true()));
/// ```
pub fn is_true() -> JsonPredicateMatcher<impl Fn(&Value) -> bool, &'static str, &'static str> {
    JsonPredicateMatcher::new(
        |v| matches!(v, Value::Bool(true)),
        "JSON true",
        "which is not JSON true",
    )
    .with_explain_fn(|v| match v {
        Value::Bool(false) => Description::new().text("which is JSON false"),
        _ => __internal_unstable_do_not_depend_on_these::describe_json_type(v),
    })
}

/// Matches the JSON boolean `false` value.
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json;
/// # use serde_json::json as j;
/// assert_that!(j!(false), json::is_false());
/// assert_that!(j!(true), not(json::is_false()));
/// ```
pub fn is_false() -> JsonPredicateMatcher<impl Fn(&Value) -> bool, &'static str, &'static str> {
    JsonPredicateMatcher::new(
        |v| matches!(v, Value::Bool(false)),
        "JSON false",
        "which is not JSON false",
    )
    .with_explain_fn(|v| match v {
        Value::Bool(true) => Description::new().text("which is JSON true"),
        _ => __internal_unstable_do_not_depend_on_these::describe_json_type(v),
    })
}

/// Matches JSON array values.
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json;
/// # use serde_json::json as j;
/// assert_that!(j!([1, 2]), json::is_array());
/// assert_that!(j!({"a":1}), not(json::is_array()));
/// ```
pub fn is_array() -> JsonPredicateMatcher<impl Fn(&Value) -> bool, &'static str, &'static str> {
    JsonPredicateMatcher::new(
        |v| v.is_array(),
        "a JSON array",
        "which is not a JSON array",
    )
    .with_explain_fn(__internal_unstable_do_not_depend_on_these::describe_json_type)
}

/// Matches an empty JSON array (`[]`).
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json;
/// # use serde_json::json as j;
/// assert_that!(j!([]), json::is_empty_array());
/// assert_that!(j!([1]), not(json::is_empty_array()));
/// ```
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
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json;
/// # use serde_json::json as j;
/// assert_that!(j!({"a": 1}), json::is_object());
/// assert_that!(j!(null), not(json::is_object()));
/// ```
pub fn is_object() -> JsonPredicateMatcher<impl Fn(&Value) -> bool, &'static str, &'static str> {
    JsonPredicateMatcher::new(
        |v| v.is_object(),
        "a JSON object",
        "which is not a JSON object",
    )
    .with_explain_fn(__internal_unstable_do_not_depend_on_these::describe_json_type)
}

/// Matches an empty JSON object (`{}`).
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json;
/// # use serde_json::json as j;
/// assert_that!(j!({}), json::is_empty_object());
/// assert_that!(j!({"a":1}), not(json::is_empty_object()));
/// ```
pub fn is_empty_object() -> JsonPredicateMatcher<impl Fn(&Value) -> bool, &'static str, &'static str>
{
    JsonPredicateMatcher::new(
        |v| v.as_object().is_some_and(|o| o.is_empty()),
        "an empty JSON object",
        "which is not an empty JSON object",
    )
    .with_explain_fn(|v| {
        if v.is_object() {
            Description::new().text("which is a non-empty JSON object")
        } else {
            __internal_unstable_do_not_depend_on_these::describe_json_type(v)
        }
    })
}

// Path-based matchers live in `path_matcher.rs`.

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
    pub trait JsonMatcher: for<'a> Matcher<&'a Value> {
        /// Returns true if this matcher allows the field to be missing in an object.
        fn allows_missing(&self) -> bool {
            false
        }
    }

    /// Trait for converting into a boxed JSON matcher.
    pub trait IntoJsonMatcher<T> {
        fn into_json_matcher(self) -> Box<dyn JsonMatcher>;
    }

    impl<J> IntoJsonMatcher<()> for J
    where
        J: JsonMatcher + 'static,
    {
        fn into_json_matcher(self) -> Box<dyn JsonMatcher> {
            Box::new(self)
        }
    }

    // A concrete matcher that checks equality with an owned serde_json::Value.
    // This avoids lifetime issues of using googletest::eq on &Value and gives
    // us control over descriptions.
    #[derive(googletest::matcher::MatcherBase)]
    struct JsonEqMatcher {
        expected: Value,
    }

    impl Matcher<&Value> for JsonEqMatcher {
        fn matches(&self, actual: &Value) -> MatcherResult {
            if *actual == self.expected {
                Match
            } else {
                NoMatch
            }
        }

        fn describe(&self, result: MatcherResult) -> Description {
            match result {
                Match => format!("is equal to {:?}", self.expected).into(),
                NoMatch => format!("isn't equal to {:?}", self.expected).into(),
            }
        }

        fn explain_match(&self, _actual: &Value) -> Description {
            // Framework prints the actual value already. Provide the expected.
            format!("which isn't equal to {:?}", self.expected).into()
        }
    }

    impl JsonMatcher for JsonEqMatcher {}

    // Allow &serde_json::Value to be used seamlessly with JSON macros
    impl IntoJsonMatcher<Value> for &Value {
        fn into_json_matcher(self) -> Box<dyn JsonMatcher> {
            Box::new(JsonEqMatcher {
                expected: self.clone(),
            })
        }
    }

    impl IntoJsonMatcher<Value> for Value {
        fn into_json_matcher(self) -> Box<dyn JsonMatcher> {
            Box::new(JsonEqMatcher { expected: self })
        }
    }

    // Literal support marker type
    pub struct Literal;

    impl IntoJsonMatcher<Literal> for &str {
        fn into_json_matcher(self) -> Box<dyn JsonMatcher> {
            Box::new(JsonEqMatcher {
                expected: Value::from(self),
            })
        }
    }

    impl IntoJsonMatcher<Literal> for String {
        fn into_json_matcher(self) -> Box<dyn JsonMatcher> {
            Box::new(JsonEqMatcher {
                expected: Value::from(self),
            })
        }
    }

    impl IntoJsonMatcher<Literal> for bool {
        fn into_json_matcher(self) -> Box<dyn JsonMatcher> {
            Box::new(JsonEqMatcher {
                expected: Value::from(self),
            })
        }
    }

    impl IntoJsonMatcher<Literal> for i64 {
        fn into_json_matcher(self) -> Box<dyn JsonMatcher> {
            Box::new(JsonEqMatcher {
                expected: Value::from(self),
            })
        }
    }
    impl IntoJsonMatcher<Literal> for i32 {
        fn into_json_matcher(self) -> Box<dyn JsonMatcher> {
            Box::new(JsonEqMatcher {
                expected: Value::from(self),
            })
        }
    }

    impl IntoJsonMatcher<Literal> for u64 {
        fn into_json_matcher(self) -> Box<dyn JsonMatcher> {
            Box::new(JsonEqMatcher {
                expected: Value::from(self),
            })
        }
    }

    impl IntoJsonMatcher<Literal> for f64 {
        fn into_json_matcher(self) -> Box<dyn JsonMatcher> {
            Box::new(JsonEqMatcher {
                expected: Value::from(self),
            })
        }
    }

    impl<P, D1, D2> JsonMatcher for JsonPredicateMatcher<P, D1, D2>
    where
        P: Fn(&Value) -> bool + 'static,
        D1: PredicateDescription + Clone + 'static,
        D2: PredicateDescription + Clone + 'static,
    {
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
