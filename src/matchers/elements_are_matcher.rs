//! JSON array matcher for concise assertions using googletest, split from the generic JSON matchers module.

/// Internal macro to build a JSON array matcher.
/// Accepts both bracketed (`__json_elements_are!([ ... ])`) and unbracketed (`__json_elements_are!(...)`) forms.
/// Callers should prefer the public `json::elements_are!` wrapper.
#[macro_export]
macro_rules! __json_elements_are {
    // Preferred bracketed form: __json_elements_are!([ m1, m2, ... ])
    ([$($matcher:expr),* $(,)?]) => {{
        $crate::matchers::__internal_unstable_do_not_depend_on_these::JsonElementsAre::new(vec![
            $(Box::new($matcher) as Box<dyn for<'a> googletest::matcher::Matcher<&'a serde_json::Value>>),*
        ])
    }};
    // Convenience: allow unbracketed list and forward to the bracketed arm.
    ($($matcher:expr),* $(,)?) => {{
        $crate::__json_elements_are!([$($matcher),*])
    }};
}

#[doc(hidden)]
pub mod internal {
    use googletest::description::Description;
    use googletest::matcher::{Matcher, MatcherBase, MatcherResult};
    use serde_json::Value;

    /// Concrete JSON array matcher. Hidden from public API; use the
    /// `json_elements_are!` macro to construct it.
    pub struct JsonElementsAre {
        elements: Vec<Box<dyn for<'a> Matcher<&'a Value>>>,
    }

    impl JsonElementsAre {
        /// Factory used by the `json_elements_are!` macro.
        pub fn new(elements: Vec<Box<dyn for<'a> Matcher<&'a Value>>>) -> Self {
            Self { elements }
        }
    }

    impl MatcherBase for JsonElementsAre {}

    impl Matcher<&Value> for JsonElementsAre {
        fn matches(&self, actual: &Value) -> MatcherResult {
            match actual {
                Value::Array(arr) => {
                    if arr.len() != self.elements.len() {
                        return MatcherResult::NoMatch;
                    }
                    for (item, matcher) in arr.iter().zip(&self.elements) {
                        if matcher.matches(item).is_no_match() {
                            return MatcherResult::NoMatch;
                        }
                    }
                    MatcherResult::Match
                }
                _ => MatcherResult::NoMatch,
            }
        }

        fn describe(&self, result: MatcherResult) -> Description {
            let verb = if result.into() { "has" } else { "doesn't have" };
            let inner = self
                .elements
                .iter()
                .map(|m| m.describe(MatcherResult::Match))
                .collect::<Description>()
                .enumerate()
                .indent();

            format!("{verb} JSON array elements:\n{inner}").into()
        }

        fn explain_match(&self, actual: &Value) -> Description {
            match actual {
                Value::Array(arr) => {
                    let mut mismatches = Vec::new();
                    let actual_len = arr.len();
                    let expected_len = self.elements.len();

                    for (index, (item, matcher)) in arr.iter().zip(&self.elements).enumerate() {
                        if matcher.matches(item).is_no_match() {
                            mismatches.push(format!(
                                "element #{index} is {item:?}, {}",
                                matcher.explain_match(item)
                            ));
                        }
                    }

                    if mismatches.is_empty() {
                        if actual_len == expected_len {
                            "whose elements all match".into()
                        } else {
                            format!("whose size is {}", actual_len).into()
                        }
                    } else if mismatches.len() == 1 {
                        let description = mismatches.into_iter().collect::<Description>();
                        format!("where {description}").into()
                    } else {
                        let description = mismatches.into_iter().collect::<Description>();
                        format!("where:\n{}", description.bullet_list().indent()).into()
                    }
                }
                _ => Description::new().text("where the type is not array".to_string()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::json;
    use googletest::prelude::*;
    use serde_json::json;
    #[test]
    fn full_match() {
        let val = json!(["a", "b", "c"]);
        assert_that!(val, json::elements_are![eq("a"), eq("b"), eq("c")]);
    }

    #[gtest]
    fn partial_match() -> Result<()> {
        let val = json!(["a", "b", "c"]);
        if let Err(err) = verify_that!(val, json::elements_are![eq("a"), eq("x"), eq("y")]) {
            assert_that!(
                err.description,
                eq("Value of: val\n\
                    Expected: has JSON array elements:\n  \
                    0. is equal to \"a\"\n  \
                    1. is equal to \"x\"\n  \
                    2. is equal to \"y\"\n\
                    Actual: Array [String(\"a\"), String(\"b\"), String(\"c\")],\n  where:\n    \
                    * element #1 is String(\"b\"), which isn't equal to \"x\"\n    \
                    * element #2 is String(\"c\"), which isn't equal to \"y\"")
            );
        } else {
            fail!("expected failure but matcher reported success")?;
        }
        Ok(())
    }
    #[test]
    fn wrong_order() {
        let val = json!(["a", "b", "c"]);
        assert_that!(val, not(json::elements_are![eq("c"), eq("b"), eq("a")]));
    }

    #[gtest]
    fn mixed_types() {
        let val = json!(["hello", 42, true]);
        assert_that!(val, json::elements_are![eq("hello"), eq(42), eq(true)]);
    }

    #[test]
    fn mixed_types_unmatch() {
        let val = json!(["hello", 42, true]);
        assert_that!(
            val,
            not(json::elements_are![eq("hello"), eq(999), eq(true)])
        );
    }

    #[gtest]
    fn length_mismatch() -> Result<()> {
        let val = json!(["a", "b"]);
        if let Err(err) = verify_that!(val, json::elements_are![eq("a"), eq("b"), eq("c")]) {
            assert_that!(
                err.description,
                eq("Value of: val\n\
                   Expected: has JSON array elements:\n  \
                   0. is equal to \"a\"\n  \
                   1. is equal to \"b\"\n  \
                   2. is equal to \"c\"\n\
                   Actual: Array [String(\"a\"), String(\"b\")],\n  \
                   whose size is 2")
            );
        } else {
            fail!("expected failure but matcher reported success")?;
        }
        Ok(())
    }

    #[test]
    fn wrong_type() {
        let val = json!(["a", 42, true]);
        assert_that!(val, not(json::elements_are![eq("a"), eq("b"), eq(true)]));
    }

    #[gtest]
    fn nested_arrays_match() {
        let val = json!([["x", "y"], ["z"]]);
        assert_that!(
            val,
            json::elements_are![
                json::elements_are![eq("x"), eq("y")],
                json::elements_are![eq("z")]
            ]
        );
    }

    #[test]
    fn nested_arrays_unmatch() {
        let val = json!([["x", "y"], ["z"]]);
        assert_that!(
            val,
            not(json::elements_are![
                json::elements_are![eq("x"), eq("z")],
                json::elements_are![eq("z")]
            ])
        );
    }
    #[test]
    fn empty_match() {
        let val = json!([]);
        assert_that!(val, json::elements_are![]);
    }

    #[test]
    fn empty_unmatch() {
        let val = json!(["unexpected"]);
        assert_that!(val, not(json::elements_are![]));
    }

    #[gtest]
    fn dupes_match() {
        let val = json!(["x", "x", "x"]);
        assert_that!(val, json::elements_are![eq("x"), eq("x"), eq("x")]);
    }

    #[test]
    fn dupes_unmatch() {
        let val = json!(["x", "y", "x"]);
        assert_that!(val, not(json::elements_are![eq("x"), eq("x"), eq("x")]));
    }
    #[gtest]
    fn not_array() -> Result<()> {
        let val = json!("not-an-array");
        if let Err(err) = verify_that!(val, json::elements_are![eq("a"), eq("b"), eq("c")]) {
            assert_that!(
                err.description,
                eq("Value of: val\n\
                   Expected: has JSON array elements:\n  \
                   0. is equal to \"a\"\n  \
                   1. is equal to \"b\"\n  \
                   2. is equal to \"c\"\n\
                   Actual: String(\"not-an-array\"),\n  \
                   where the type is not array")
            );
        } else {
            return fail!("expected failure but matcher reported success");
        }
        Ok(())
    }
}
