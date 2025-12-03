mod each_matcher;
mod elements_are_matcher;
mod json_matcher;
mod len_matcher;
mod matches_pattern_matcher;
mod optional_matcher;
mod path_matcher;
mod primitive_matcher;
mod unordered_elements_are_matcher;

#[allow(deprecated)]
pub use json_matcher::{
    any_value, is_array, is_boolean, is_empty_array, is_empty_object, is_false,
    is_fractional_number, is_integer, is_not_null, is_null, is_number, is_object, is_string,
    is_true, is_whole_number, predicate,
};
pub use path_matcher::{has_only_paths, has_paths};

#[allow(deprecated)]
#[doc(inline)]
pub use crate::{
    __json_contains_each as contains_each, __json_each as each,
    __json_elements_are as elements_are, __json_has_path_with as has_path_with,
    __json_is_contained_in as is_contained_in, __json_len as len, __json_matches_pattern as pat,
    __json_matches_pattern as matches_pattern, __json_optional as optional,
    __json_primitive as primitive, __json_unordered_elements_are as unordered_elements_are,
    __json_value as value,
};

#[doc(hidden)]
pub mod __internal_unstable_do_not_depend_on_these {
    pub use super::each_matcher::internal::JsonEachMatcher;
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
    pub use super::unordered_elements_are_matcher::internal::JsonUnorderedElementsAreMatcher;
    pub use crate::matcher_support::match_matrix::internal::Requirements;
}
