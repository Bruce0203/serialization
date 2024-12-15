#![cfg_attr(feature = "fast_binary_format", feature(generic_arg_infer))]
#![cfg_attr(feature = "fast_binary_format", feature(ptr_sub_ptr))]
#![cfg_attr(feature = "fast_binary_format", feature(const_ptr_sub_ptr))]
#![cfg_attr(feature = "fast_binary_format", feature(generic_const_exprs))]
#![cfg_attr(feature = "fast_binary_format", feature(const_trait_impl))]
#![feature(specialization)]
#![feature(min_specialization)]

pub use serialization_derive::*;

pub use crate::traits::*;
mod traits;

mod impls;

pub mod constvec {
    pub use constvec::*;
}

#[cfg(feature = "fast_binary_format")]
pub mod binary_format;
