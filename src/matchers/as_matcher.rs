use crate::matchers::as_matcher::internal::JsonAsMatcher;
use googletest::matcher::Matcher;
use serde_json::{Map, Value};

/// Matches a JSON string value against a native string matcher.
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json as j;
/// # use serde_json::json;
/// assert_that!(json!("hello"), j::as_string(starts_with("h")));
/// assert_that!(json!(42), not(j::as_string(anything())));
/// ```
pub fn as_string<M>(inner: M) -> JsonAsMatcher<M, String>
where
    M: for<'a> Matcher<&'a str>,
{
    JsonAsMatcher::new(inner)
}

/// Matches a JSON boolean value against a native boolean matcher.
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json as j;
/// # use serde_json::json;
/// assert_that!(json!(true), j::as_bool(eq(true)));
/// ```
pub fn as_bool<M>(inner: M) -> JsonAsMatcher<M, bool>
where
    M: Matcher<bool>,
{
    JsonAsMatcher::new(inner)
}

/// Matches a JSON number value as an i64 against a native i64 matcher.
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json as j;
/// # use serde_json::json;
/// assert_that!(json!(42), j::as_i64(gt(40)));
/// ```
pub fn as_i64<M>(inner: M) -> JsonAsMatcher<M, i64>
where
    M: Matcher<i64>,
{
    JsonAsMatcher::new(inner)
}

/// Matches a JSON number value as a u64 against a native u64 matcher.
pub fn as_u64<M>(inner: M) -> JsonAsMatcher<M, u64>
where
    M: Matcher<u64>,
{
    JsonAsMatcher::new(inner)
}

/// Matches a JSON number value as an f64 against a native f64 matcher.
pub fn as_f64<M>(inner: M) -> JsonAsMatcher<M, f64>
where
    M: Matcher<f64>,
{
    JsonAsMatcher::new(inner)
}

/// Matches a JSON number value as an i32 against a native i32 matcher.
pub fn as_i32<M>(inner: M) -> JsonAsMatcher<M, i32>
where
    M: Matcher<i32>,
{
    JsonAsMatcher::new(inner)
}

/// Matches a JSON number value as a u32 against a native u32 matcher.
pub fn as_u32<M>(inner: M) -> JsonAsMatcher<M, u32>
where
    M: Matcher<u32>,
{
    JsonAsMatcher::new(inner)
}

/// Matches a JSON number value as an i16 against a native i16 matcher.
pub fn as_i16<M>(inner: M) -> JsonAsMatcher<M, i16>
where
    M: Matcher<i16>,
{
    JsonAsMatcher::new(inner)
}

/// Matches a JSON number value as a u16 against a native u16 matcher.
pub fn as_u16<M>(inner: M) -> JsonAsMatcher<M, u16>
where
    M: Matcher<u16>,
{
    JsonAsMatcher::new(inner)
}

/// Matches a JSON number value as an i8 against a native i8 matcher.
pub fn as_i8<M>(inner: M) -> JsonAsMatcher<M, i8>
where
    M: Matcher<i8>,
{
    JsonAsMatcher::new(inner)
}

/// Matches a JSON number value as a u8 against a native u8 matcher.
pub fn as_u8<M>(inner: M) -> JsonAsMatcher<M, u8>
where
    M: Matcher<u8>,
{
    JsonAsMatcher::new(inner)
}

/// Matches a JSON number value as a usize against a native usize matcher.
pub fn as_usize<M>(inner: M) -> JsonAsMatcher<M, usize>
where
    M: Matcher<usize>,
{
    JsonAsMatcher::new(inner)
}

/// Matches a JSON array value against a native matcher for `&Vec<Value>`.
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json as j;
/// # use serde_json::json;
/// assert_that!(json!([1, 2]), j::as_array(not(is_empty())));
/// ```
pub fn as_array<M>(inner: M) -> JsonAsMatcher<M, Vec<Value>>
where
    M: for<'a> Matcher<&'a Vec<Value>>,
{
    JsonAsMatcher::new(inner)
}

/// Matches a JSON object value against a native matcher for `&Map<String, Value>`.
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use googletest_json_serde::json as j;
/// # use serde_json::json;
/// assert_that!(json!({"a": 1}), j::as_object(contains_key("a")));
/// ```
pub fn as_object<M>(inner: M) -> JsonAsMatcher<M, Map<String, Value>>
where
    M: for<'a> Matcher<&'a Map<String, Value>>,
{
    JsonAsMatcher::new(inner)
}

#[doc(hidden)]
pub mod internal {
    use crate::matchers::__internal_unstable_do_not_depend_on_these::describe_json_type;
    use crate::matchers::json_matcher::internal::JsonMatcher;
    use googletest::description::Description;
    use googletest::matcher::{Matcher, MatcherBase, MatcherResult};
    use serde_json::{Map, Value};
    use std::marker::PhantomData;

    #[doc(hidden)]
    #[derive(MatcherBase)]
    pub struct JsonAsMatcher<M, T> {
        inner: M,
        pub(super) phantom: PhantomData<T>,
    }

    impl<M, T> JsonAsMatcher<M, T> {
        pub fn new(inner: M) -> Self {
            Self {
                inner,
                phantom: PhantomData,
            }
        }
    }

    impl<M, T> JsonMatcher for JsonAsMatcher<M, T> where Self: for<'a> Matcher<&'a Value> {}

    impl<M> Matcher<&Value> for JsonAsMatcher<M, String>
    where
        M: for<'a> Matcher<&'a str>,
    {
        fn matches(&self, actual: &Value) -> MatcherResult {
            actual
                .as_str()
                .map_or(MatcherResult::NoMatch, |s| self.inner.matches(s))
        }

        fn describe(&self, result: MatcherResult) -> Description {
            format!("is a JSON string which {}", self.inner.describe(result)).into()
        }

        fn explain_match(&self, actual: &Value) -> Description {
            actual.as_str().map_or_else(
                || describe_json_type(actual),
                |s| self.inner.explain_match(s),
            )
        }
    }

    impl<M> Matcher<&Value> for JsonAsMatcher<M, bool>
    where
        M: Matcher<bool>,
    {
        fn matches(&self, actual: &Value) -> MatcherResult {
            actual
                .as_bool()
                .map_or(MatcherResult::NoMatch, |b| self.inner.matches(b))
        }

        fn describe(&self, result: MatcherResult) -> Description {
            format!("is a JSON boolean which {}", self.inner.describe(result)).into()
        }

        fn explain_match(&self, actual: &Value) -> Description {
            actual.as_bool().map_or_else(
                || describe_json_type(actual),
                |b| self.inner.explain_match(b),
            )
        }
    }

    macro_rules! impl_number_as_matcher {
        ($t:ty, $conv:ident, $desc:expr) => {
            impl<M> Matcher<&Value> for JsonAsMatcher<M, $t>
            where
                M: Matcher<$t>,
            {
                fn matches(&self, actual: &Value) -> MatcherResult {
                    actual
                        .as_number()
                        .and_then(|n| n.$conv())
                        .map_or(MatcherResult::NoMatch, |v| self.inner.matches(v))
                }

                fn describe(&self, result: MatcherResult) -> Description {
                    format!(
                        "is a JSON number ({}) which {}",
                        $desc,
                        self.inner.describe(result)
                    )
                    .into()
                }

                fn explain_match(&self, actual: &Value) -> Description {
                    match actual.as_number() {
                        Some(n) => match n.$conv() {
                            Some(v) => self.inner.explain_match(v),
                            None => {
                                format!("which is a JSON number but out of {} range", $desc).into()
                            }
                        },
                        None => describe_json_type(actual),
                    }
                }
            }
        };
    }

    impl_number_as_matcher!(i64, as_i64, "i64");
    impl_number_as_matcher!(u64, as_u64, "u64");
    impl_number_as_matcher!(f64, as_f64, "f64");

    // Integer types that require try_from
    macro_rules! impl_int_as_matcher {
        ($t:ty, $conv:ident, $desc:expr) => {
            impl<M> Matcher<&Value> for JsonAsMatcher<M, $t>
            where
                M: Matcher<$t>,
            {
                fn matches(&self, actual: &Value) -> MatcherResult {
                    actual
                        .as_number()
                        .and_then(|n| n.$conv())
                        .and_then(|v| <$t>::try_from(v).ok())
                        .map_or(MatcherResult::NoMatch, |v| self.inner.matches(v))
                }

                fn describe(&self, result: MatcherResult) -> Description {
                    format!(
                        "is a JSON number ({}) which {}",
                        $desc,
                        self.inner.describe(result)
                    )
                    .into()
                }

                fn explain_match(&self, actual: &Value) -> Description {
                    match actual.as_number() {
                        Some(n) => match n.$conv() {
                            Some(v) => match <$t>::try_from(v) {
                                Ok(val) => self.inner.explain_match(val),
                                Err(_) => {
                                    format!("which is a JSON number but out of {} range", $desc)
                                        .into()
                                }
                            },
                            None => format!("which is a JSON number but out of range").into(),
                        },
                        None => describe_json_type(actual),
                    }
                }
            }
        };
    }

    impl_int_as_matcher!(i32, as_i64, "i32");
    impl_int_as_matcher!(i16, as_i64, "i16");
    impl_int_as_matcher!(i8, as_i64, "i8");
    impl_int_as_matcher!(u32, as_u64, "u32");
    impl_int_as_matcher!(u16, as_u64, "u16");
    impl_int_as_matcher!(u8, as_u64, "u8");
    impl_int_as_matcher!(usize, as_u64, "usize");

    impl<M> Matcher<&Value> for JsonAsMatcher<M, Vec<Value>>
    where
        M: for<'a> Matcher<&'a Vec<Value>>,
    {
        fn matches(&self, actual: &Value) -> MatcherResult {
            actual
                .as_array()
                .map_or(MatcherResult::NoMatch, |a| self.inner.matches(a))
        }

        fn describe(&self, result: MatcherResult) -> Description {
            format!("is a JSON array which {}", self.inner.describe(result)).into()
        }

        fn explain_match(&self, actual: &Value) -> Description {
            actual.as_array().map_or_else(
                || describe_json_type(actual),
                |a| self.inner.explain_match(a),
            )
        }
    }

    impl<M> Matcher<&Value> for JsonAsMatcher<M, Map<String, Value>>
    where
        M: for<'a> Matcher<&'a Map<String, Value>>,
    {
        fn matches(&self, actual: &Value) -> MatcherResult {
            actual
                .as_object()
                .map_or(MatcherResult::NoMatch, |o| self.inner.matches(o))
        }

        fn describe(&self, result: MatcherResult) -> Description {
            format!("is a JSON object which {}", self.inner.describe(result)).into()
        }

        fn explain_match(&self, actual: &Value) -> Description {
            actual.as_object().map_or_else(
                || describe_json_type(actual),
                |o| self.inner.explain_match(o),
            )
        }
    }
}
