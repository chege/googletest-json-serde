/// Matches a JSON object against a pattern of key-value matchers.
///
/// Fields listed in the pattern must match; a trailing `..` allows extra fields.
///
/// # Examples
///
/// ```rust
/// # use googletest::prelude::*;
/// # use serde_json::json as j;
/// # use googletest_json_serde::json;
/// let value = j!({
///     "name": "Alice",
///     "age": 30,
///     "active": true,
///     "role": j!("admin")
/// });
/// assert_that!(
///     value,
///     json::pat!({
///         "name": starts_with("Al"),
///         "age": ge(29),
///         "active": true,
///         "role": j!("admin"),
///         .. // allows additional fields
///     })
/// );
/// ```
///
/// # Errors
///
/// Fails when the value is not a JSON object, when a required field is missing, when a field value mismatches, or when extra fields appear without `..`.
///
/// This macro is reexported as [`json::pat!`](crate::json::pat).
///
/// # Supported Inputs
/// - Literal JSON-compatible values
/// - Direct `serde_json::Value`
/// - Native googletest matchers
#[macro_export]
#[doc(hidden)]
macro_rules! __json_matches_pattern {
    // Literal arm: handles string/number/bool literals like `"x"` or `1i64`.
    (@wrap_matcher $lit:literal) => {
        $crate::matchers::__internal_unstable_do_not_depend_on_these::IntoJsonMatcher::<
            $crate::matchers::__internal_unstable_do_not_depend_on_these::Literal
        >::into_json_matcher($lit)
    };
    // Nested object arm: handles `key: { ... }` by delegating to the pattern macro.
    (@wrap_matcher { $($inner:tt)* }) => {
        $crate::matchers::__internal_unstable_do_not_depend_on_these::IntoJsonMatcher::into_json_matcher(
            $crate::__json_matches_pattern!({ $($inner)* })
        )
    };
    // Expression arm: handles matchers and values like `eq(1)` or `j!(...)`.
    (@wrap_matcher $expr:expr) => {
        $crate::matchers::__internal_unstable_do_not_depend_on_these::IntoJsonMatcher::into_json_matcher($expr)
    };
    // Parse completion: no more tokens to consume.
    (@parse $fields:ident $strict:ident; ) => {};
    // Spread operator arm: `..` makes the object relaxed (must be last).
    (@parse $fields:ident $strict:ident; ..) => {
        $strict = false;
    };
    // Error case: `..` is only valid at the end of the object pattern.
    (@parse $fields:ident $strict:ident; .. , $($rest:tt)+) => {
        compile_error!("`..` must be the last token in a json::pat! object pattern");
    };
    // Nested object value: recurse into the inner pattern (e.g., `"user": { "id": eq(1) }`).
    (@parse $fields:ident $strict:ident;
        $key:literal : { $($inner:tt)* } $(, $($rest:tt)*)?
    ) => {{
        $fields.push((
            $key,
            $crate::__json_matches_pattern!(@wrap_matcher { $($inner)* }),
        ));
        $crate::__json_matches_pattern!(@parse $fields $strict; $($($rest)*)?);
    }};
    // Leaf value: handles `key: expr` when the value is not an object.
    (@parse $fields:ident $strict:ident;
        $key:literal : $val:expr $(, $($rest:tt)*)?
    ) => {{
        $fields.push((
            $key,
            $crate::__json_matches_pattern!(@wrap_matcher $val),
        ));
        $crate::__json_matches_pattern!(@parse $fields $strict; $($($rest)*)?);
    }};
    // Entry point: build the field list and parse the pattern tokens.
    ({ $($tokens:tt)* }) => {{
        let mut fields = Vec::new();
        let mut strict = true;
        $crate::__json_matches_pattern!(@parse fields strict; $($tokens)*);
        $crate::matchers::__internal_unstable_do_not_depend_on_these::JsonObjectMatcher::new(
            fields,
            strict,
        )
    }};
}

#[doc(hidden)]
pub mod internal {
    use crate::matchers::json_matcher::internal::JsonMatcher;
    use googletest::{
        description::Description,
        matcher::{Matcher, MatcherBase, MatcherResult},
    };
    use serde_json::{Map, Value};

    type FieldMatcherPair = (&'static str, Box<dyn JsonMatcher>);
    #[derive(MatcherBase)]
    pub struct JsonObjectMatcher {
        fields: Vec<FieldMatcherPair>,
        strict: bool,
    }

    impl JsonMatcher for JsonObjectMatcher {}

    impl JsonObjectMatcher {
        pub fn new(fields: Vec<FieldMatcherPair>, strict: bool) -> Self {
            Self { fields, strict }
        }

        fn collect_field_mismatches(&self, obj: &Map<String, Value>) -> Vec<String> {
            let mut mismatches = Vec::new();
            for (key, matcher) in &self.fields {
                match obj.get(*key) {
                    Some(value) => {
                        if matcher.matches(value).is_no_match() {
                            mismatches.push(format!(
                                "  field '{}': {}",
                                key,
                                matcher.explain_match(value)
                            ));
                        }
                    }
                    None => {
                        if !matcher.allows_missing() {
                            mismatches.push(format!("  field '{key}': was missing"));
                        }
                    }
                }
            }
            mismatches
        }

        fn collect_unknown_fields(&self, obj: &Map<String, Value>) -> Vec<String> {
            let mut unknown_fields = Vec::new();
            for key in obj.keys() {
                if !self
                    .fields
                    .iter()
                    .any(|(expected_key, _)| expected_key == key)
                {
                    unknown_fields.push(format!("  unexpected field '{key}' present"));
                }
            }
            unknown_fields
        }
    }

    impl Matcher<&Value> for JsonObjectMatcher {
        fn matches(&self, actual: &Value) -> MatcherResult {
            let Value::Object(obj) = actual else {
                return MatcherResult::NoMatch;
            };

            // 1. Check all expected fields
            for (key, matcher) in &self.fields {
                match obj.get(*key) {
                    Some(v) => {
                        if matcher.matches(v).is_no_match() {
                            return MatcherResult::NoMatch;
                        }
                    }
                    None => {
                        // Missing field is fine only if the matcher declares it is optional.
                        if !matcher.allows_missing() {
                            return MatcherResult::NoMatch;
                        }
                    }
                }
            }

            // 2. In strict mode, reject unknown fields
            if self.strict {
                for actual_key in obj.keys() {
                    if !self
                        .fields
                        .iter()
                        .any(|(expected_key, _)| expected_key == actual_key)
                    {
                        return MatcherResult::NoMatch;
                    }
                }
            }

            MatcherResult::Match
        }

        fn describe(&self, result: MatcherResult) -> Description {
            if result.is_match() {
                "has JSON object with expected fields".into()
            } else {
                let expected_fields = self
                    .fields
                    .iter()
                    .map(|(k, m)| format!("  '{}': {}", k, m.describe(MatcherResult::Match)))
                    .collect::<Vec<_>>()
                    .join("\n");
                format!("expected JSON object with fields:\n{expected_fields}").into()
            }
        }
        fn explain_match(&self, actual: &Value) -> Description {
            match actual {
                Value::Object(obj) => {
                    if obj.is_empty() && self.fields.iter().all(|(_, m)| m.allows_missing()) {
                        Description::new()
                    } else {
                        let mut mismatches = self.collect_field_mismatches(obj);

                        if self.strict {
                            let unknown_fields = self.collect_unknown_fields(obj);
                            mismatches.extend(unknown_fields);
                        }

                        if mismatches.is_empty() {
                            Description::new().text("all fields matched as expected")
                        } else if mismatches.len() == 1 {
                            Description::new().text(
                                mismatches
                                    .into_iter()
                                    .next()
                                    .unwrap()
                                    .trim_start()
                                    .to_string(),
                            )
                        } else {
                            Description::new().text(format!(
                                "had {} field mismatches:\n{}",
                                mismatches.len(),
                                mismatches.join("\n")
                            ))
                        }
                    }
                }
                _ => Description::new().text(format!("was {actual} (expected object)")),
            }
        }
    }

    /// Support matching on `Option<Value>` to handle cases where JSON objects may be optional,
    /// such as API responses that might be null.
    impl Matcher<&Option<Value>> for JsonObjectMatcher {
        fn matches(&self, actual: &Option<Value>) -> MatcherResult {
            match actual {
                Some(v) => self.matches(v),
                None => MatcherResult::NoMatch,
            }
        }

        fn describe(&self, result: MatcherResult) -> Description {
            if result.is_match() {
                "has Some(JSON object) with expected fields".into()
            } else {
                let expected_fields = self
                    .fields
                    .iter()
                    .map(|(k, m)| format!("  '{}': {}", k, m.describe(MatcherResult::Match)))
                    .collect::<Vec<_>>()
                    .join("\n");
                format!("expected Some(JSON object) with fields:\n{expected_fields}").into()
            }
        }

        fn explain_match(&self, actual: &Option<Value>) -> Description {
            match actual {
                Some(value) => {
                    // Delegate to the main implementation's explain_match
                    self.explain_match(value)
                }
                None => Description::new().text("was None (expected Some(JSON object))"),
            }
        }
    }
}
