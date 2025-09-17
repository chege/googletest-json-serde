mod elements_are_matcher;
mod object;
mod scalar;
mod unordered_elements_are_matcher;

pub use scalar::json_scalar as scalar;

pub use crate::{
    __json_contains_each as contains_each, __json_elements_are as elements_are,
    __json_is_contained_in as is_contained_in, __json_matches_pattern as pat,
    __json_matches_pattern as matches_pattern,
    __json_unordered_elements_are as unordered_elements_are,
};

#[doc(hidden)]
pub mod __internal_unstable_do_not_depend_on_these {
    pub use super::elements_are_matcher::internal::JsonElementsAre;
    pub use super::object::internal::JsonObjectMatcher;
    pub use super::unordered_elements_are_matcher::internal::JsonUnorderedElementsAre;
    pub use super::unordered_elements_are_matcher::internal::Requirements;
}
