#![feature(never_type)]
#![feature(ptr_sub_ptr)]
#![feature(const_ptr_sub_ptr)]
#![cfg_attr(test, feature(test))]
#![feature(negative_impls)]
#![feature(auto_traits)]
#![feature(min_specialization)]
#![feature(associated_type_defaults)]

mod mesh;

pub use mesh::*;
//1. field offset
//2. len
//3. actor
//4. derive
