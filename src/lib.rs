#![doc = include_str!("../README.md")]
#[doc(hidden)]
pub mod matcher_support;

#[doc(hidden)]
pub mod matchers;

pub mod json {
    #[doc(inline)]
    pub use super::matchers::*;
}
