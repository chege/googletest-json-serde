mod elements_are_matcher;
mod matches_pattern_matcher;
mod unordered_elements_are_matcher;
mod value_matcher;

pub use value_matcher::is_null;

#[doc(inline)]
pub use crate::{
    __json_contains_each as contains_each, __json_elements_are as elements_are,
    __json_is_contained_in as is_contained_in, __json_matches_pattern as pat,
    __json_matches_pattern as matches_pattern,
    __json_unordered_elements_are as unordered_elements_are, __json_value as value,
};

#[doc(hidden)]
pub mod __internal_unstable_do_not_depend_on_these {
    pub use super::elements_are_matcher::internal::JsonElementsAre;
    pub use super::matches_pattern_matcher::internal::JsonObjectMatcher;
    pub use super::unordered_elements_are_matcher::internal::JsonUnorderedElementsAreMatcher;
    pub use super::value_matcher::internal::IsJsonNull;
    pub use super::value_matcher::internal::JsonValueMatcher;
    pub use crate::matcher_support::match_matrix::internal::Requirements;
}
