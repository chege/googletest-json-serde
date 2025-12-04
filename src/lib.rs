#![doc = include_str!("../README.md")]
#[doc(hidden)]
pub mod matcher_support;

#[doc(hidden)]
pub mod matchers;

pub mod json {
    #[allow(deprecated)]
    #[doc(inline)]
    pub use super::matchers::{
        any_value, contains_each, each, elements_are, has_only_paths, has_path_with, has_paths,
        is_array, is_boolean, is_contained_in, is_empty_array, is_empty_object, is_false,
        is_fractional_number, is_integer, is_non_empty_array, is_non_empty_object, is_not_null,
        is_null, is_number, is_object, is_string, is_true, is_whole_number, len, matches_pattern,
        optional, pat, predicate, primitive, unordered_elements_are, value,
    };
}

// Show matchers on the crate root in generated docs without changing the runtime API.
#[cfg(doc)]
#[doc(inline)]
pub use json::{
    any_value, contains_each, each, elements_are, has_only_paths, has_path_with, has_paths,
    is_array, is_boolean, is_contained_in, is_empty_array, is_empty_object, is_false,
    is_fractional_number, is_integer, is_non_empty_array, is_non_empty_object, is_not_null,
    is_null, is_number, is_object, is_string, is_true, is_whole_number, len, matches_pattern,
    optional, pat, predicate, primitive, unordered_elements_are, value,
};
