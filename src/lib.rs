//TODO Cow<'a, T> SerialDescriptor impl
//TODO support tuple fast binary encoding
//TODO fix ArrayVec drop dangerous in impl Decode
//TODO fix primitive type's encode/decode to call decode2/encode2
//TODO warnings on proc macro
//TODO use raw vec on decode Vec<T>
#![feature(allocator_api)]
#![feature(alloc_layout_extra)]
#![feature(type_alias_impl_trait)]
#![feature(auto_traits)]
#![feature(negative_impls)]
#![feature(raw_vec_internals)]
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
