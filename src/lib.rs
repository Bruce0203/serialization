//TODO remove it
#![allow(warnings, reason = "dev")]
#![allow(
    incomplete_features,
    reason = "`generic_const_exprs` feature required for field offset calculation on generic type"
)]
#![feature(generic_const_exprs)]
#![cfg_attr(test, feature(test))]

#[cfg(test)]
extern crate self as serialization;
#[cfg(test)]
extern crate test;

#[cfg(test)]
mod fuzz;
#[cfg(test)]
mod mock;

mod buffer;
mod codec;
mod derive;
mod mesh;

pub use buffer::*;
pub use codec::*;
pub use mesh::*;
pub use serialization_derive::Serializable;
