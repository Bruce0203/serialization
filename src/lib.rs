#![feature(const_copy_from_slice)]
#![feature(const_ptr_sub_ptr)]
#![feature(ptr_sub_ptr)]
#![feature(generic_arg_infer)]
#![feature(const_try)]
#![feature(const_type_id)]
#![feature(const_trait_impl)]
#![feature(generic_const_items)]
#![feature(generic_const_exprs)]
#![feature(const_for)]
#![feature(inline_const_pat)]
#![feature(negative_impls)]
#![feature(specialization)]
#![feature(trivial_bounds)]
#![feature(auto_traits)]
#![feature(min_specialization)]

pub use serialization_derive::*;

pub use crate::traits::*;
mod traits;

mod impls;

pub mod binary_format;
