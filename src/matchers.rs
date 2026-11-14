mod as_matcher;
mod each_matcher;
mod each_object_matcher;
mod elements_are_matcher;
mod json_matcher;
mod len_matcher;
mod matches_pattern_matcher;
mod optional_matcher;
mod path_matcher;
mod primitive_matcher;
mod set_matcher;
mod unordered_elements_are_matcher;

#[doc(hidden)]
#[macro_export]
macro_rules! __json_array_value {
    // Detect repeat syntax without parsing JSON objects as Rust expressions.
    (@scan [$($before:tt)*] ; $($after:tt)*) => {
        [$($before)*; $($after)*]
    };
    (@scan [$($before:tt)*] $next:tt $($rest:tt)*) => {
        $crate::__json_array_value!(@scan [$($before)* $next] $($rest)*)
    };
    (@scan [$($array:tt)*]) => {{
        let mut values = ::std::vec::Vec::new();
        $crate::__json_array_value!(@parse values; $($array)*);
        $crate::matchers::__internal_unstable_do_not_depend_on_these::JsonValue::Array(values)
    }};
    (@parse $values:ident;) => {};
    (@parse $values:ident; null $(, $($rest:tt)*)?) => {{
        $values.push($crate::matchers::__internal_unstable_do_not_depend_on_these::json!(null));
        $crate::__json_array_value!(@parse $values; $($($rest)*)?);
    }};
    (@parse $values:ident; [$($array:tt)*] $(, $($rest:tt)*)?) => {{
        $values.push($crate::matchers::__internal_unstable_do_not_depend_on_these::json!(
            $crate::__json_array_value!([$($array)*])
        ));
        $crate::__json_array_value!(@parse $values; $($($rest)*)?);
    }};
    (@parse $values:ident; {$($object:tt)*} $(, $($rest:tt)*)?) => {{
        $values.push($crate::matchers::__internal_unstable_do_not_depend_on_these::json!({$($object)*}));
        $crate::__json_array_value!(@parse $values; $($($rest)*)?);
    }};
    (@parse $values:ident; $value:expr $(, $($rest:tt)*)?) => {{
        $values.push($crate::matchers::__internal_unstable_do_not_depend_on_these::json!($value));
        $crate::__json_array_value!(@parse $values; $($($rest)*)?);
    }};
    ([$($array:tt)*]) => {
        $crate::__json_array_value!(@scan [] $($array)*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __json_array_arguments {
    (@parse $kind:ident, $values:ident;) => {};
    // Batch simple tokens or pairs of expressions to reduce expansion depth.
    (@parse $kind:ident, $values:ident; $($item:tt),+ $(,)?) => {{
        $($crate::__json_array_arguments!(@push_item $kind, $values; $item);)+
    }};
    (@parse $kind:ident, $values:ident; [$($first:tt)*], [$($second:tt)*] $(, $($rest:tt)*)?) => {{
        $crate::__json_array_arguments!(@push_array $kind, $values; [$($first)*]);
        $crate::__json_array_arguments!(@push_array $kind, $values; [$($second)*]);
        $crate::__json_array_arguments!(@parse $kind, $values; $($($rest)*)?);
    }};
    (@parse $kind:ident, $values:ident; [$($first:tt)*], $second:expr $(, $($rest:tt)*)?) => {{
        $crate::__json_array_arguments!(@push_array $kind, $values; [$($first)*]);
        $crate::__json_array_arguments!(@push_expr $kind, $values; $second);
        $crate::__json_array_arguments!(@parse $kind, $values; $($($rest)*)?);
    }};
    (@parse $kind:ident, $values:ident; $first:expr, [$($second:tt)*] $(, $($rest:tt)*)?) => {{
        $crate::__json_array_arguments!(@push_expr $kind, $values; $first);
        $crate::__json_array_arguments!(@push_array $kind, $values; [$($second)*]);
        $crate::__json_array_arguments!(@parse $kind, $values; $($($rest)*)?);
    }};
    (@parse $kind:ident, $values:ident; $first:expr, $second:expr $(, $($rest:tt)*)?) => {{
        $crate::__json_array_arguments!(@push_expr $kind, $values; $first);
        $crate::__json_array_arguments!(@push_expr $kind, $values; $second);
        $crate::__json_array_arguments!(@parse $kind, $values; $($($rest)*)?);
    }};
    (@parse $kind:ident, $values:ident; [$($array:tt)*] $(, $($rest:tt)*)?) => {{
        $crate::__json_array_arguments!(@push_array $kind, $values; [$($array)*]);
        $crate::__json_array_arguments!(@parse $kind, $values; $($($rest)*)?);
    }};
    (@parse $kind:ident, $values:ident; $value:expr $(, $($rest:tt)*)?) => {{
        $crate::__json_array_arguments!(@push_expr $kind, $values; $value);
        $crate::__json_array_arguments!(@parse $kind, $values; $($($rest)*)?);
    }};
    (@push_item $kind:ident, $values:ident; [$($array:tt)*]) => {
        $crate::__json_array_arguments!(@push_array $kind, $values; [$($array)*]);
    };
    (@push_item $kind:ident, $values:ident; $value:tt) => {
        $crate::__json_array_arguments!(@push_expr $kind, $values; $value);
    };
    (@push_array $kind:ident, $values:ident; [$($array:tt)*]) => {
        $crate::__json_array_arguments!(@push_expr $kind, $values; $crate::__json_array_value!([$($array)*]));
    };
    (@push_expr matcher, $values:ident; $value:expr) => {
        $values.push(
            $crate::matchers::__internal_unstable_do_not_depend_on_these::IntoJsonMatcher::into_json_matcher($value)
        );
    };
    (@push_expr value, $values:ident; $value:expr) => {
        $values.push($crate::matchers::__internal_unstable_do_not_depend_on_these::json!($value));
    };
    ($kind:ident; $(,)?) => {
        ::std::vec::Vec::new()
    };
    ($kind:ident; $($values:tt)+) => {{
        let mut values = ::std::vec::Vec::new();
        $crate::__json_array_arguments!(@parse $kind, values; $($values)*);
        values
    }};
}

pub use as_matcher::{
    as_array, as_bool, as_f64, as_i8, as_i16, as_i32, as_i64, as_object, as_string, as_u8, as_u16,
    as_u32, as_u64, as_usize,
};
pub use each_object_matcher::{each_key, each_value};
#[allow(deprecated)]
pub use json_matcher::{
    any_value, is_array, is_boolean, is_empty_array, is_empty_object, is_empty_string, is_false,
    is_fractional_number, is_integer, is_non_empty_array, is_non_empty_object, is_non_empty_string,
    is_not_null, is_null, is_number, is_object, is_string, is_true, is_whole_number, predicate,
};
mod each_is_matcher;
pub use each_is_matcher::{
    each_is_array, each_is_boolean, each_is_null, each_is_number, each_is_object, each_is_string,
};
pub use path_matcher::{has_only_paths, has_paths};

#[allow(deprecated)]
#[doc(inline)]
pub use crate::{
    __json_contains_each as contains_each, __json_each as each,
    __json_elements_are as elements_are, __json_has_path_with as has_path_with,
    __json_is_contained_in as is_contained_in, __json_len as len, __json_matches_pattern as pat,
    __json_matches_pattern as matches_pattern, __json_optional as optional,
    __json_primitive as primitive, __json_subset_of as subset_of,
    __json_superset_of as superset_of, __json_unordered_elements_are as unordered_elements_are,
    __json_value as value,
};

#[doc(hidden)]
pub mod __internal_unstable_do_not_depend_on_these {
    pub use super::as_matcher::internal::JsonAsMatcher;
    pub use super::each_matcher::internal::JsonEachMatcher;
    pub use super::each_object_matcher::internal::{JsonEachKeyMatcher, JsonEachValueMatcher};
    pub use super::elements_are_matcher::internal::JsonElementsAre;
    pub use super::json_matcher::internal::IntoJsonMatcher;
    pub use super::json_matcher::internal::JsonMatcher;
    pub use super::json_matcher::internal::JsonPredicateMatcher;
    pub use super::json_matcher::internal::Literal;
    pub use super::json_matcher::internal::NoDescription;
    pub use super::json_matcher::internal::PredicateDescription;
    pub use super::json_matcher::internal::describe_json_type;
    pub use super::len_matcher::internal::JsonLenMatcher;
    pub use super::matches_pattern_matcher::internal::JsonObjectMatcher;
    pub use super::optional_matcher::internal::JsonOptionalMatcher;
    pub use super::path_matcher::internal::JsonPathWithMatcher;
    pub use super::primitive_matcher::internal::JsonPrimitiveMatcher;
    pub use super::set_matcher::internal::{JsonSetMatcher, Relation as JsonSetRelation};
    pub use super::unordered_elements_are_matcher::internal::JsonUnorderedElementsAreMatcher;
    pub use crate::matcher_support::match_matrix::internal::Requirements;
    pub use serde_json::{Value as JsonValue, json};
}
