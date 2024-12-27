#![feature(portable_simd)]
#![feature(generic_const_exprs)]
#![feature(unboxed_closures)]
#![feature(const_ptr_sub_ptr)]
#![feature(ptr_sub_ptr)]
#![feature(specialization)]
#![feature(min_specialization)]
#![feature(inline_const_pat)]
#![allow(internal_features)]
#![feature(core_intrinsics)]
#![feature(new_range_api)]
#![feature(const_trait_impl)]
#![cfg_attr(test, feature(test))]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate self as serialization;

pub use derive::*;
pub use traits::*;

mod derive;
pub(crate) mod macros;
mod traits;

pub mod fastbuf {
    pub use fastbuf::*;
}
