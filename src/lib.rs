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
mod benches;
#[cfg(test)]
mod fuzz;
pub mod mock;

mod codec;
mod derive;
mod macros;
mod mesh;

pub use codec::*;
pub use macros::*;
pub use mesh::*;
pub use serialization_derive::Serializable;
