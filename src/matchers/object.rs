//! Utility matchers and macros for concise JSON object pattern matching using googletest.

/// Macro to create a [`JsonObjectMatcher`] matcher from a JSON-like literal.
///
/// Supports:
/// - nested JSON objects
/// - optional JSON values (`Option<Value>`)
///
/// Each key maps to a matcher that is executed on the corresponding value.
/// Extra keys in the actual JSON are ignored unless explicitly matched.
///
/// Example:
/// ```rust
/// use googletest_serde_json::json;
/// use googletest::prelude::*;
///
/// let val = serde_json::json!({"name": "Alice", "age": 30.0});
/// assert_that!(
///     val,
///     json::pat!({
///         "name": eq("Alice"),
///         "age": eq(30.0),
///     })
/// );
/// ```
#[macro_export]
macro_rules! __json_pat {
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
    /// Matcher for JSON objects with specified field matchers.
    #[derive(MatcherBase)]
    pub struct JsonObjectMatcher {
        /// List of `(key, matcher)` pairs evaluated against the JSON object.
        fields: Vec<FieldMatcherPair>,
        /// If true, requires the actual JSON object to have exactly the same fields as declared.
        /// Defaults to true in `json_pat!` without `..`, and false when using `..`.
        /// This controls whether extra unexpected fields in the JSON will fail the match.
        strict: bool,
    }

    impl JsonObjectMatcher {
        /// Creates a new `JsonObjectMatcher` with the given field matchers and strictness.
        ///
        /// - `fields`: A list of `(key, matcher)` pairs to apply to the JSON object.
        /// - `strict`: If `true`, requires the object to have exactly the specified fields, `false`
        ///   allows extra fields not explicitly matched.
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

#[cfg(test)]
mod tests {
    use crate::json;
    use googletest::prelude::*;
    use serde_json::{Value, json};

    #[test]
    fn match_option_with_object() {
        let val = Some(json!({"field": "value"}));
        assert_that!(
            val,
            json::pat!({
                "field": eq("value")
            })
        );
    }

    #[gtest]
    fn match_object_with_extra_fields() -> Result<()> {
        let val = json!({"field": "value", "unexpected": 123});
        if let Err(err) = verify_that!(
            val,
            json::pat!({
                "field": eq("value")
            })
        ) {
            assert_that!(
                err.description,
                contains_substring("unexpected field 'unexpected' present")
            );
        } else {
            fail!("expected failure but matcher reported success")?;
        }
        Ok(())
    }
    #[test]
    fn match_object_with_anything_field() {
        let val = json!({"field": "value", "unexpected": 123});
        assert_that!(
            val,
            json::pat!({
                "field": eq("value"),
                "unexpected": anything()
            })
        );
    }

    #[gtest]
    fn explain_mismatch_option_nested_object() -> Result<()> {
        let val = Some(json!({
            "field": {
                "subfield": 123,
                "flag": false
            },
            "extra": "hello"
        }));
        if let Err(err) = verify_that!(
            val,
            json::pat!({
                "field": json::pat!({
                    "subfield": eq(999),
                    "flag": eq(true)
                }),
                "extra": eq("world")
            })
        ) {
            assert_that!(
                err.description,
                all![
                    contains_substring("field 'field': had 2 field mismatches"),
                    contains_substring("field 'subfield': which isn't equal to 999"),
                    contains_substring("field 'flag': which isn't equal to true"),
                    contains_substring("field 'extra': which isn't equal to \"world\""),
                ]
            );
        } else {
            fail!("expected failure but matcher reported success")?;
        }
        Ok(())
    }
    #[test]
    fn match_option_none() {
        let val: Option<Value> = None;
        assert_that!(
            val,
            not(json::pat!({
                "field": eq("value")
            }))
        );
    }

    #[test]
    fn match_object_with_wrong_field() {
        let val = json!({"field": "other"});
        assert_that!(
            val,
            not(json::pat!({
                "field": eq("value")
            }))
        );
    }

    #[gtest]
    fn explain_mismatch_nested_object() -> Result<()> {
        let val = json!({
            "field": {
                "subfield": 123,
                "flag": false
            },
            "extra": "hello"
        });
        if let Err(err) = verify_that!(
            val,
            json::pat!({
                "field": json::pat!({
                    "subfield": eq(999),
                    "flag": eq(true)
                }),
                "extra": eq("world")
            })
        ) {
            assert_that!(
                err.description,
                all![
                    contains_substring("field 'field': had 2 field mismatches"),
                    contains_substring("field 'subfield': which isn't equal to 999"),
                    contains_substring("field 'flag': which isn't equal to true"),
                    contains_substring("field 'extra': which isn't equal to \"world\""),
                ]
            );
        } else {
            fail!("expected failure but matcher reported success")?;
        }
        Ok(())
    }

    #[test]
    fn match_strict_object() {
        let val = json!({"field": "value", "float":20.0});
        assert_that!(
            val,
            json::pat!({
                "field": eq("value"),
                "float": eq(20.0),
            })
        );
    }

    #[test]
    fn match_non_strict_object() {
        let val = json!({"field": "value", "extra": 123});
        assert_that!(
            val,
            json::pat!({
                "field": eq("value"),
                ..
            })
        );
    }
}
