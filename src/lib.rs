#![cfg_attr(test, feature(test))]

mod mesh;

pub use mesh::*;
pub use serialization_derive::Serializable;

/// Fix derive macro
extern crate self as serialization;
