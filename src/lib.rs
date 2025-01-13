#![allow(
    incomplete_features,
    reason = "`generic_const_exprs` feature required for field offset calculation on generic type"
)]
#![feature(generic_const_exprs)]
#![cfg_attr(test, feature(test))]

/// Fix derive macro
extern crate self as serialization;

mod mesh;

pub use mesh::*;
pub use serialization_derive::Serializable;
