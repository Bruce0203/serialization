#![cfg_attr(test, feature(test))]
#![feature(generic_const_exprs)]

mod mesh;

 #[cfg(test)]
 mod fuzz;

pub use mesh::*;
pub use serialization_derive::Serializable;

/// Fix derive macro
extern crate self as serialization;
