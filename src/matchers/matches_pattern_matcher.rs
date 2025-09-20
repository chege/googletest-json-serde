/// Matches a JSON object by specifying a pattern of key-value matchers, similar to
/// GoogleTest’s `matches_pattern!` macro for Rust structs.
///
/// This macro is used for asserting that a `serde_json::Value` representing a JSON object
/// contains the specified fields, with each field matching the corresponding matcher. Extra
/// fields are rejected unless the pattern ends with `..`.
///
/// # Examples
///
/// Basic usage:
/// ```
/// # use googletest::prelude::*;
/// # use serde_json::json;
/// # use googletest_serde_json::json;
/// let value = json!({ "name": "Alice", "age": 30i64 });
/// assert_that!(
///     value,
///     json::pat!({
///         "name": eq("Alice"),
///         "age": json::value!(ge(18i64)),
///         .. // allows additional fields
///     })
/// );
/// ```
///
/// Nested matching:
/// ```
/// # use googletest::prelude::*;
/// # use serde_json::json;
/// # use googletest_serde_json::json;
/// let value = json!({
///     "user": {
///         "id": 1,
///         "active": true
///     }
/// });
/// assert_that!(
///     value,
///     json::pat!({
///         "user": json::pat!({
///             "id": eq(1),
///             "active": eq(true),
///         })
///     })
/// );
/// ```
///
/// # Notes
///
/// - Matchers like `eq(...)` can be used directly.
/// - For non-`Value` matchers (e.g., `starts_with`, `contains_substring`), wrap them in
///   `json::value!(...)`.
///
/// # Alias
///
/// This macro is reexported as [`json::pat!`](crate::json::pat).
#[macro_export]
#[doc(hidden)]
macro_rules! __json_matches_pattern {
    // Strict version: no `..`
    ({ $($key:literal : $val:expr),* $(,)? }) => {{
        let fields = vec![
            $(
                ($key,
                 Box::new($val) as Box<dyn for<'a> googletest::matcher::Matcher<&'a serde_json::Value>>
                )
            ),*
        ];
        $crate::matchers::__internal_unstable_do_not_depend_on_these::JsonObjectMatcher::new ( fields, true )
    }};
    // Non-strict version: trailing `..`
    ({ $($key:literal : $val:expr),* , .. }) => {{
        let fields = vec![
            $(
                ($key,
                 Box::new($val) as Box<dyn for<'a> googletest::matcher::Matcher<&'a serde_json::Value>>
                )
            ),*
        ];
        $crate::matchers::__internal_unstable_do_not_depend_on_these::JsonObjectMatcher::new ( fields, false )
    }};
}

#[doc(hidden)]
pub mod internal {
    use googletest::{
        description::Description,
        matcher::{Matcher, MatcherBase, MatcherResult},
    };
    use serde_json::{Map, Value};

    type FieldMatcherPair = (&'static str, Box<dyn for<'a> Matcher<&'a Value>>);
    #[derive(MatcherBase)]
    pub struct JsonObjectMatcher {
        fields: Vec<FieldMatcherPair>,
        strict: bool,
    }

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
                        mismatches.push(format!("  field '{key}': was missing"));
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
            if let Value::Object(obj) = actual {
                for (k, m) in &self.fields {
                    match obj.get(*k) {
                        Some(v) if m.matches(v).is_match() => (),
                        _ => return MatcherResult::NoMatch,
                    }
                }
                if self.strict && obj.len() != self.fields.len() {
                    return MatcherResult::NoMatch;
                }
                MatcherResult::Match
            } else {
                MatcherResult::NoMatch
            }
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
