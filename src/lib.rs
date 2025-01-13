#![cfg_attr(test, feature(test))]
#![allow(
    incomplete_features,
    reason = "field offset calculation of generic type field"
)]
#![feature(generic_const_exprs)]

mod mesh;

#[cfg(test)]
mod fuzz;

pub use mesh::*;
pub use serialization_derive::Serializable;

/// Fix derive macro
extern crate self as serialization;
