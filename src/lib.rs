#![feature(const_trait_impl)]
#![doc = include_str!("../README.md")]

pub use serialization_core::*;
pub use serialization_derive::*;

#[doc(hidden)]
pub mod __private {
    pub use crate::*;
    pub use serialization_descriptor::*;
}
