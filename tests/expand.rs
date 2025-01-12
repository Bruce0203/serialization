#![feature(core_intrinsics)]
#![feature(fmt_helpers_for_derive)]
#![feature(derive_eq)]
#![feature(structural_match)]
#![feature(coverage_attribute)]
#![feature(panic_internals)]
#![feature(rustc_attrs)]
#![feature(print_internals)]

use std::marker::PhantomData;

use serialization::__private::{CompoundWrapper, PhantomField};

pub struct A<T> {
    value: T,
}
pub struct __Token<T>(PhantomData<T>);

pub trait Foo {
    type Output;
}

impl<T> Foo for A<T> {
    type Output = __Token<T>;
}
impl<S, T> CompoundWrapper<S> for __Token<PhantomField<S, T, 0>> {
    type Compound = ();
}
