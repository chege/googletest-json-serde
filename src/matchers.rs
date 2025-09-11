mod elements_are_matcher;
mod is_contained_in_matcher;
mod object;
mod scalar;
mod unordered_elements_are_matcher;

pub use scalar::json_scalar as scalar;

pub use crate::{
    __json_elements_are as elements_are, __json_is_contained_in as is_contained_in,
    __json_pat as pat, __json_unordered_elements_are as unordered_elements_are,
};

#[doc(hidden)]
pub mod __internal_unstable_do_not_depend_on_these {
    pub use super::elements_are_matcher::internal::JsonElementsAre;
    pub use super::is_contained_in_matcher::internal::JsonIsContainedIn;
    pub use super::object::internal::JsonObjectMatcher;
    pub use super::unordered_elements_are_matcher::internal::JsonUnorderedElementsAre;
}
