//! Utility matchers and macros for concise JSON assertions using googletest.
//!
//! Examples:
//! ```
//! # use googletest::prelude::*;
//! # use googletest_json_serde::json;
//! # use serde_json::json as j;
//! let data = j!({"small": 12i8, "large": 65000u16});
//! verify_that!(data["small"], json::primitive!(eq(12i8)));
//! verify_that!(data["large"], json::primitive!(ge(65000u16)));
//! ```

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
/// let data = j!({"active": true, "count": 3, "small": 12i8, "limit": 65000u16});
///
/// verify_that!(data["active"], json::primitive!(eq(true)));
/// verify_that!(data["count"], json::primitive!(ge(0)));
/// verify_that!(data["small"], json::primitive!(eq(12i8)));
/// verify_that!(data["limit"], json::primitive!(eq(65000u16)));
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! __json_primitive {
    ($matcher:expr) => {
        $crate::matchers::__internal_unstable_do_not_depend_on_these::JsonPrimitiveMatcher::new(
            $matcher,
        )
    };
}

#[doc(hidden)]
pub mod internal {
    use crate::matchers::json_matcher::internal::{IntoJsonMatcher, JsonMatcher};
    use googletest::description::Description;
    use googletest::matcher::{Matcher, MatcherBase, MatcherResult};
    use serde_json::Value;

    #[doc(hidden)]
    #[derive(MatcherBase)]
    pub struct JsonPrimitiveMatcher<M, T> {
        inner: M,
        phantom: std::marker::PhantomData<T>,
    }

    impl<M, T> JsonPrimitiveMatcher<M, T> {
        pub fn new(inner: M) -> Self {
            Self {
                inner,
                phantom: std::marker::PhantomData,
            }
        }
    }

    impl<M> Matcher<&Value> for JsonPrimitiveMatcher<M, String>
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

    impl<M> Matcher<&Value> for JsonPrimitiveMatcher<M, i64>
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

    impl<M> Matcher<&Value> for JsonPrimitiveMatcher<M, f64>
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

    impl<M> Matcher<&Value> for JsonPrimitiveMatcher<M, bool>
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

    impl<M, T> JsonMatcher for JsonPrimitiveMatcher<M, T> where
        JsonPrimitiveMatcher<M, T>: for<'a> Matcher<&'a Value>
    {
    }

    /// Trait for converting into a boxed JSON matcher.
    impl<M> IntoJsonMatcher<i64> for M
    where
        M: Matcher<i64> + 'static,
    {
        fn into_json_matcher(self) -> Box<dyn JsonMatcher> {
            Box::new(JsonPrimitiveMatcher::<M, i64>::new(self))
        }
    }

    impl<M> Matcher<&Value> for JsonPrimitiveMatcher<M, u64>
    where
        M: Matcher<u64>,
    {
        fn matches(&self, actual: &Value) -> MatcherResult {
            match actual {
                Value::Number(n) => n
                    .as_u64()
                    .map_or(MatcherResult::NoMatch, |u| self.inner.matches(u)),
                _ => MatcherResult::NoMatch,
            }
        }
        fn describe(&self, r: MatcherResult) -> Description {
            self.inner.describe(r)
        }
        fn explain_match(&self, actual: &Value) -> Description {
            match actual {
                Value::Number(n) => match n.as_u64() {
                    Some(u) => self.inner.explain_match(u),
                    None => Description::new().text(format!("number out of u64 range: {n}")),
                },
                _ => Description::new().text("which is not a JSON number"),
            }
        }
    }

    impl<M> IntoJsonMatcher<u64> for M
    where
        M: Matcher<u64> + 'static,
    {
        fn into_json_matcher(self) -> Box<dyn JsonMatcher> {
            Box::new(JsonPrimitiveMatcher::<M, u64>::new(self))
        }
    }

    impl<M> IntoJsonMatcher<f64> for M
    where
        M: Matcher<f64> + 'static,
    {
        fn into_json_matcher(self) -> Box<dyn JsonMatcher> {
            Box::new(JsonPrimitiveMatcher::<M, f64>::new(self))
        }
    }

    impl<M> IntoJsonMatcher<String> for M
    where
        M: for<'a> Matcher<&'a str> + 'static,
    {
        fn into_json_matcher(self) -> Box<dyn JsonMatcher> {
            Box::new(JsonPrimitiveMatcher::<M, String>::new(self))
        }
    }

    impl<M> IntoJsonMatcher<bool> for M
    where
        M: Matcher<bool> + 'static,
    {
        fn into_json_matcher(self) -> Box<dyn JsonMatcher> {
            Box::new(JsonPrimitiveMatcher::<M, bool>::new(self))
        }
    }

    impl<M> Matcher<&Value> for JsonPrimitiveMatcher<M, i32>
    where
        M: Matcher<i32>,
    {
        fn matches(&self, actual: &Value) -> MatcherResult {
            match actual {
                Value::Number(n) => match n.as_i64() {
                    Some(i) => match i32::try_from(i) {
                        Ok(i32_val) => self.inner.matches(i32_val),
                        Err(_) => MatcherResult::NoMatch,
                    },
                    None => MatcherResult::NoMatch,
                },
                _ => MatcherResult::NoMatch,
            }
        }
        fn describe(&self, r: MatcherResult) -> Description {
            self.inner.describe(r)
        }
        fn explain_match(&self, actual: &Value) -> Description {
            match actual {
                Value::Number(n) => match n.as_i64() {
                    Some(i) => match i32::try_from(i) {
                        Ok(i32_val) => self.inner.explain_match(i32_val),
                        Err(_) => Description::new().text(format!("number out of i32 range: {n}")),
                    },
                    None => Description::new().text(format!("number out of i64 range: {n}")),
                },
                _ => Description::new().text("which is not a JSON number"),
            }
        }
    }

    impl<M> IntoJsonMatcher<i32> for M
    where
        M: Matcher<i32> + 'static,
    {
        fn into_json_matcher(self) -> Box<dyn JsonMatcher> {
            Box::new(JsonPrimitiveMatcher::<M, i32>::new(self))
        }
    }

    impl<M> Matcher<&Value> for JsonPrimitiveMatcher<M, i8>
    where
        M: Matcher<i8>,
    {
        fn matches(&self, actual: &Value) -> MatcherResult {
            match actual {
                Value::Number(n) => match n.as_i64() {
                    Some(i) => match i8::try_from(i) {
                        Ok(i8_val) => self.inner.matches(i8_val),
                        Err(_) => MatcherResult::NoMatch,
                    },
                    None => MatcherResult::NoMatch,
                },
                _ => MatcherResult::NoMatch,
            }
        }
        fn describe(&self, r: MatcherResult) -> Description {
            self.inner.describe(r)
        }
        fn explain_match(&self, actual: &Value) -> Description {
            match actual {
                Value::Number(n) => match n.as_i64() {
                    Some(i) => match i8::try_from(i) {
                        Ok(i8_val) => self.inner.explain_match(i8_val),
                        Err(_) => Description::new().text(format!("number out of i8 range: {n}")),
                    },
                    None => Description::new().text(format!("number out of i64 range: {n}")),
                },
                _ => Description::new().text("which is not a JSON number"),
            }
        }
    }

    impl<M> IntoJsonMatcher<i8> for M
    where
        M: Matcher<i8> + 'static,
    {
        fn into_json_matcher(self) -> Box<dyn JsonMatcher> {
            Box::new(JsonPrimitiveMatcher::<M, i8>::new(self))
        }
    }

    impl<M> Matcher<&Value> for JsonPrimitiveMatcher<M, i16>
    where
        M: Matcher<i16>,
    {
        fn matches(&self, actual: &Value) -> MatcherResult {
            match actual {
                Value::Number(n) => match n.as_i64() {
                    Some(i) => match i16::try_from(i) {
                        Ok(i16_val) => self.inner.matches(i16_val),
                        Err(_) => MatcherResult::NoMatch,
                    },
                    None => MatcherResult::NoMatch,
                },
                _ => MatcherResult::NoMatch,
            }
        }
        fn describe(&self, r: MatcherResult) -> Description {
            self.inner.describe(r)
        }
        fn explain_match(&self, actual: &Value) -> Description {
            match actual {
                Value::Number(n) => match n.as_i64() {
                    Some(i) => match i16::try_from(i) {
                        Ok(i16_val) => self.inner.explain_match(i16_val),
                        Err(_) => Description::new().text(format!("number out of i16 range: {n}")),
                    },
                    None => Description::new().text(format!("number out of i64 range: {n}")),
                },
                _ => Description::new().text("which is not a JSON number"),
            }
        }
    }

    impl<M> IntoJsonMatcher<i16> for M
    where
        M: Matcher<i16> + 'static,
    {
        fn into_json_matcher(self) -> Box<dyn JsonMatcher> {
            Box::new(JsonPrimitiveMatcher::<M, i16>::new(self))
        }
    }

    impl<M> Matcher<&Value> for JsonPrimitiveMatcher<M, u8>
    where
        M: Matcher<u8>,
    {
        fn matches(&self, actual: &Value) -> MatcherResult {
            match actual {
                Value::Number(n) => match n.as_u64() {
                    Some(u) => match u8::try_from(u) {
                        Ok(u8_val) => self.inner.matches(u8_val),
                        Err(_) => MatcherResult::NoMatch,
                    },
                    None => MatcherResult::NoMatch,
                },
                _ => MatcherResult::NoMatch,
            }
        }
        fn describe(&self, r: MatcherResult) -> Description {
            self.inner.describe(r)
        }
        fn explain_match(&self, actual: &Value) -> Description {
            match actual {
                Value::Number(n) => match n.as_u64() {
                    Some(u) => match u8::try_from(u) {
                        Ok(u8_val) => self.inner.explain_match(u8_val),
                        Err(_) => Description::new().text(format!("number out of u8 range: {n}")),
                    },
                    None => Description::new().text(format!("number out of u64 range: {n}")),
                },
                _ => Description::new().text("which is not a JSON number"),
            }
        }
    }

    impl<M> IntoJsonMatcher<u8> for M
    where
        M: Matcher<u8> + 'static,
    {
        fn into_json_matcher(self) -> Box<dyn JsonMatcher> {
            Box::new(JsonPrimitiveMatcher::<M, u8>::new(self))
        }
    }

    impl<M> Matcher<&Value> for JsonPrimitiveMatcher<M, u16>
    where
        M: Matcher<u16>,
    {
        fn matches(&self, actual: &Value) -> MatcherResult {
            match actual {
                Value::Number(n) => match n.as_u64() {
                    Some(u) => match u16::try_from(u) {
                        Ok(u16_val) => self.inner.matches(u16_val),
                        Err(_) => MatcherResult::NoMatch,
                    },
                    None => MatcherResult::NoMatch,
                },
                _ => MatcherResult::NoMatch,
            }
        }
        fn describe(&self, r: MatcherResult) -> Description {
            self.inner.describe(r)
        }
        fn explain_match(&self, actual: &Value) -> Description {
            match actual {
                Value::Number(n) => match n.as_u64() {
                    Some(u) => match u16::try_from(u) {
                        Ok(u16_val) => self.inner.explain_match(u16_val),
                        Err(_) => Description::new().text(format!("number out of u16 range: {n}")),
                    },
                    None => Description::new().text(format!("number out of u64 range: {n}")),
                },
                _ => Description::new().text("which is not a JSON number"),
            }
        }
    }

    impl<M> IntoJsonMatcher<u16> for M
    where
        M: Matcher<u16> + 'static,
    {
        fn into_json_matcher(self) -> Box<dyn JsonMatcher> {
            Box::new(JsonPrimitiveMatcher::<M, u16>::new(self))
        }
    }

    impl<M> Matcher<&Value> for JsonPrimitiveMatcher<M, u32>
    where
        M: Matcher<u32>,
    {
        fn matches(&self, actual: &Value) -> MatcherResult {
            match actual {
                Value::Number(n) => match n.as_u64() {
                    Some(u) => match u32::try_from(u) {
                        Ok(u32_val) => self.inner.matches(u32_val),
                        Err(_) => MatcherResult::NoMatch,
                    },
                    None => MatcherResult::NoMatch,
                },
                _ => MatcherResult::NoMatch,
            }
        }
        fn describe(&self, r: MatcherResult) -> Description {
            self.inner.describe(r)
        }
        fn explain_match(&self, actual: &Value) -> Description {
            match actual {
                Value::Number(n) => match n.as_u64() {
                    Some(u) => match u32::try_from(u) {
                        Ok(u32_val) => self.inner.explain_match(u32_val),
                        Err(_) => Description::new().text(format!("number out of u32 range: {n}")),
                    },
                    None => Description::new().text(format!("number out of u64 range: {n}")),
                },
                _ => Description::new().text("which is not a JSON number"),
            }
        }
    }

    impl<M> IntoJsonMatcher<u32> for M
    where
        M: Matcher<u32> + 'static,
    {
        fn into_json_matcher(self) -> Box<dyn JsonMatcher> {
            Box::new(JsonPrimitiveMatcher::<M, u32>::new(self))
        }
    }

    // usize support
    impl<M> Matcher<&Value> for JsonPrimitiveMatcher<M, usize>
    where
        M: Matcher<usize>,
    {
        fn matches(&self, actual: &Value) -> MatcherResult {
            match actual {
                Value::Number(n) => match n.as_u64() {
                    Some(u) => match usize::try_from(u) {
                        Ok(usize_val) => self.inner.matches(usize_val),
                        Err(_) => MatcherResult::NoMatch,
                    },
                    None => MatcherResult::NoMatch,
                },
                _ => MatcherResult::NoMatch,
            }
        }

        fn describe(&self, r: MatcherResult) -> Description {
            self.inner.describe(r)
        }

        fn explain_match(&self, actual: &Value) -> Description {
            match actual {
                Value::Number(n) => match n.as_u64() {
                    Some(u) => match usize::try_from(u) {
                        Ok(usize_val) => self.inner.explain_match(usize_val),
                        Err(_) => {
                            Description::new().text(format!("number out of usize range: {n}"))
                        }
                    },
                    None => Description::new().text(format!("number not convertible to u64: {n}")),
                },
                _ => Description::new().text("which is not a JSON number"),
            }
        }
    }

    impl<M> IntoJsonMatcher<usize> for M
    where
        M: Matcher<usize> + 'static,
    {
        fn into_json_matcher(self) -> Box<dyn JsonMatcher> {
            Box::new(JsonPrimitiveMatcher::<M, usize>::new(self))
        }
    }
}
