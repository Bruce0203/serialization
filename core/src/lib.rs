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

pub use traits::*;

pub(crate) mod macros;
mod traits;
mod derive;
mod descriptor;
pub use descriptor::*;

pub mod fastbuf {
    pub use fastbuf::*;
}

pub(crate) const unsafe fn const_transmute<A, B>(a: A) -> B {
    if std::mem::size_of::<A>() != std::mem::size_of::<B>() {
        panic!("Size mismatch for generic_array::const_transmute");
    }

    #[repr(C)]
    union Union<A, B> {
        a: std::mem::ManuallyDrop<A>,
        b: std::mem::ManuallyDrop<B>,
    }

    let a = std::mem::ManuallyDrop::new(a);
    std::mem::ManuallyDrop::into_inner(Union { a }.b)
}
