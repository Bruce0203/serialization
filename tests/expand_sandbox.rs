#![allow(warnings)]
#![feature(core_intrinsics)]
#![feature(fmt_helpers_for_derive)]
#![feature(derive_eq)]
#![feature(structural_match)]
#![feature(coverage_attribute)]
#![feature(panic_internals)]
#![feature(rustc_attrs)]
#![feature(print_internals)]
#![feature(prelude_import)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::marker::PhantomData;
pub struct A14<T> {
    vaule: std::marker::PhantomData<T>,
}
const _: () = {
    impl<T, __C> ::serialization::__private::Edge<__C> for A14<T>
    where
        T: ::serialization::__private::Edge<__C>,
    {
        type First = ::serialization::__private::End<__C, A14<T>>;
        type Second = <<::serialization::__private::End<__C, A14<T>> as core::ops::Add<
            ::serialization::__private::Padding<
                __C,
                A14<T>,
                ::serialization::__private::Field<
                    __FieldToken<A14<T>, std::marker::PhantomData<T>, 0>,
                >,
            >,
        >>::Output as core::ops::Add<
            ::serialization::__private::Field<__FieldToken<A14<T>, std::marker::PhantomData<T>, 0>>,
        >>::Output;
    }
    impl<T> ::serialization::__private::Size for A14<T> {
        const SIZE: usize = core::mem::size_of::<A14<T>>();
    }
    impl<T> ::serialization::__private::Len for A14<T> {
        const SIZE: usize = core::mem::size_of::<A14<T>>();
    }
    impl<T, __C, __S> ::serialization::__private::CompoundWrapper<__C, __S> for A14<T>
    where
        Self: ::serialization::__private::CompoundUnwrapper<__C, __S>,
    {
        type Compound = <A14<T> as ::serialization::__private::CompoundUnwrapper<__C, __S>>::Output;
    }
    impl<T> ::serialization::Encode for A14<T> {
        fn encode<E: ::serialization::Encoder>(&self, _encoder: &mut E) -> Result<(), E::Error> {
            Ok(())
        }
    }
    #[repr(transparent)]
    pub struct __FieldToken<S, T, const I: usize>(
        core::mem::MaybeUninit<T>,
        core::marker::PhantomData<S>,
    );
    impl<C, S, T, const I: usize> ::serialization::__private::Edge<C> for __FieldToken<S, T, I>
    where
        T: ::serialization::__private::Edge<C>,
    {
        type First = T::First;
        type Second = T::Second;
    }
    impl<S, T, const I: usize> ::serialization::__private::Len for __FieldToken<S, T, I>
    where
        T: ::serialization::__private::Len,
    {
        const SIZE: usize = T::SIZE;
    }
    impl<S, T, const I: usize> ::serialization::__private::Size for __FieldToken<S, T, I>
    where
        T: ::serialization::__private::Size,
    {
        const SIZE: usize = T::SIZE;
    }
    impl<S, T, const I: usize> ::serialization::Encode for __FieldToken<S, T, I>
    where
        T: ::serialization::Encode,
    {
        fn encode<E: ::serialization::Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
            unsafe { self.0.assume_init_ref() }.encode(encoder)
        }
    }
    impl<C, S, S2, T, const I: usize> ::serialization::__private::CompoundWrapper<C, S>
        for __FieldToken<S2, T, I>
    where
        T: ::serialization::__private::CompoundWrapper<C, S>,
    {
        type Compound = <T as ::serialization::__private::CompoundWrapper<C, S>>::Compound;
    }
    impl<S, T, const I: usize> ::serialization::Decode for __FieldToken<S, T, I>
    where
        T: ::serialization::Decode,
    {
        fn decode_in_place<D: ::serialization::Decoder>(
            decoder: &mut D,
            out: &mut core::mem::MaybeUninit<Self>,
        ) -> Result<(), D::Error> {
            T::decode_in_place(decoder, unsafe { core::mem::transmute(out) })
        }
    }
    impl<S, T, const I: usize> ::serialization::__private::Vector for __FieldToken<S, T, I>
    where
        T: ::serialization::__private::Vector,
    {
        type Item = T::Item;
        fn as_iter(&self) -> impl Iterator<Item = &Self::Item> {
            unsafe { self.0.assume_init_ref() }.as_iter()
        }
        fn as_ptr(&self) -> *const Self::Item {
            unsafe { self.0.assume_init_ref() }.as_ptr()
        }
        fn len(&self) -> usize {
            unsafe { self.0.assume_init_ref() }.len()
        }
        fn set_len(&mut self, len: usize) {
            unsafe { self.0.assume_init_mut() }.set_len(len)
        }
    }
    const _: () = {
        impl<T> ::serialization::__private::FieldOffset
            for __FieldToken<A14<T>, std::marker::PhantomData<T>, 0>
        where
            [(); __field_offset::<T>()]:,
        {
            type Offset = ::serialization::__private::typenum::Const<{ __field_offset::<T>() }>;
        }
        pub const fn __field_offset<T>() -> usize {
            {
                use core::mem::MaybeUninit;
                let origin: MaybeUninit<A14<T>> = MaybeUninit::uninit();
                #[allow(unused_variables)]
                let A14 { vaule } = unsafe { origin.assume_init_ref() };
                unsafe {
                    ::serialization::__private::sub_ptr(
                        vaule as *const _ as *const u8,
                        origin.assume_init_ref() as *const _ as *const u8,
                    )
                }
            }
        }
    };
};
#[automatically_derived]
impl<T: ::core::fmt::Debug> ::core::fmt::Debug for A14<T> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "A14", "vaule", &&self.vaule)
    }
}
#[automatically_derived]
impl<T: ::core::cmp::Eq> ::core::cmp::Eq for A14<T> {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<std::marker::PhantomData<T>>;
    }
}
#[automatically_derived]
impl<T> ::core::marker::StructuralPartialEq for A14<T> {}
#[automatically_derived]
impl<T: ::core::cmp::PartialEq> ::core::cmp::PartialEq for A14<T> {
    #[inline]
    fn eq(&self, other: &A14<T>) -> bool {
        self.vaule == other.vaule
    }
}
pub struct A15<T> {
    value: T,
}
const _: () = {
    impl<T, __C> ::serialization::__private::Edge<__C> for A15<T>
    where
        T: ::serialization::__private::Edge<__C>,
    {
        type First = ::serialization::__private::End<__C, A15<T>>;
        type Second = <<::serialization::__private::End<__C, A15<T>> as core::ops::Add<
            ::serialization::__private::Padding<
                __C,
                A15<T>,
                ::serialization::__private::Field<__FieldToken<A15<T>, T, 0>>,
            >,
        >>::Output as core::ops::Add<
            ::serialization::__private::Field<__FieldToken<A15<T>, T, 0>>,
        >>::Output;
    }
    impl<T> ::serialization::__private::Size for A15<T> {
        const SIZE: usize = core::mem::size_of::<A15<T>>();
    }
    impl<T> ::serialization::__private::Len for A15<T> {
        const SIZE: usize = core::mem::size_of::<A15<T>>();
    }
    impl<T, __C, __S> ::serialization::__private::CompoundWrapper<__C, __S> for A15<T>
    where
        Self: ::serialization::__private::CompoundUnwrapper<__C, __S>,
    {
        type Compound = <A15<T> as ::serialization::__private::CompoundUnwrapper<__C, __S>>::Output;
    }
    impl<T> ::serialization::Encode for A15<T> {
        fn encode<E: ::serialization::Encoder>(&self, _encoder: &mut E) -> Result<(), E::Error> {
            Ok(())
        }
    }
    #[repr(transparent)]
    pub struct __FieldToken<S, T, const I: usize>(
        core::mem::MaybeUninit<T>,
        core::marker::PhantomData<S>,
    );
    impl<C, S, T, const I: usize> ::serialization::__private::Edge<C> for __FieldToken<S, T, I>
    where
        T: ::serialization::__private::Edge<C>,
    {
        type First = T::First;
        type Second = T::Second;
    }
    impl<S, T, const I: usize> ::serialization::__private::Len for __FieldToken<S, T, I>
    where
        T: ::serialization::__private::Len,
    {
        const SIZE: usize = T::SIZE;
    }
    impl<S, T, const I: usize> ::serialization::__private::Size for __FieldToken<S, T, I>
    where
        T: ::serialization::__private::Size,
    {
        const SIZE: usize = T::SIZE;
    }
    impl<S, T, const I: usize> ::serialization::Encode for __FieldToken<S, T, I>
    where
        T: ::serialization::Encode,
    {
        fn encode<E: ::serialization::Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
            unsafe { self.0.assume_init_ref() }.encode(encoder)
        }
    }
    impl<C, S, S2, T, const I: usize> ::serialization::__private::CompoundWrapper<C, S>
        for __FieldToken<S2, T, I>
    where
        T: ::serialization::__private::CompoundWrapper<C, S>,
    {
        type Compound = <T as ::serialization::__private::CompoundWrapper<C, S>>::Compound;
    }
    impl<S, T, const I: usize> ::serialization::Decode for __FieldToken<S, T, I>
    where
        T: ::serialization::Decode,
    {
        fn decode_in_place<D: ::serialization::Decoder>(
            decoder: &mut D,
            out: &mut core::mem::MaybeUninit<Self>,
        ) -> Result<(), D::Error> {
            T::decode_in_place(decoder, unsafe { core::mem::transmute(out) })
        }
    }
    impl<S, T, const I: usize> ::serialization::__private::Vector for __FieldToken<S, T, I>
    where
        T: ::serialization::__private::Vector,
    {
        type Item = T::Item;
        fn as_iter(&self) -> impl Iterator<Item = &Self::Item> {
            unsafe { self.0.assume_init_ref() }.as_iter()
        }
        fn as_ptr(&self) -> *const Self::Item {
            unsafe { self.0.assume_init_ref() }.as_ptr()
        }
        fn len(&self) -> usize {
            unsafe { self.0.assume_init_ref() }.len()
        }
        fn set_len(&mut self, len: usize) {
            unsafe { self.0.assume_init_mut() }.set_len(len)
        }
    }
    const _: () = {
        impl<T> ::serialization::__private::FieldOffset for __FieldToken<A15<T>, T, 0>
        where
            [(); __field_offset::<T>()]:,
        {
            type Offset = ::serialization::__private::typenum::Const<{ __field_offset::<T>() }>;
        }
        pub const fn __field_offset<T>() -> usize {
            {
                use core::mem::MaybeUninit;
                let origin: MaybeUninit<A15<T>> = MaybeUninit::uninit();
                #[allow(unused_variables)]
                let A15 { value } = unsafe { origin.assume_init_ref() };
                unsafe {
                    ::serialization::__private::sub_ptr(
                        value as *const _ as *const u8,
                        origin.assume_init_ref() as *const _ as *const u8,
                    )
                }
            }
        }
    };
};
#[automatically_derived]
impl<T: ::core::fmt::Debug> ::core::fmt::Debug for A15<T> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "A15", "value", &&self.value)
    }
}
#[automatically_derived]
impl<T: ::core::cmp::Eq> ::core::cmp::Eq for A15<T> {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<T>;
    }
}
#[automatically_derived]
impl<T> ::core::marker::StructuralPartialEq for A15<T> {}
#[automatically_derived]
impl<T: ::core::cmp::PartialEq> ::core::cmp::PartialEq for A15<T> {
    #[inline]
    fn eq(&self, other: &A15<T>) -> bool {
        self.value == other.value
    }
}
pub enum A16<T, T2> {
    A(T),
    B(std::marker::PhantomData<T2>),
}
const _: () = {
    pub struct __VariantToken<T, T2, const I: usize>(core::marker::PhantomData<(T, T2)>);
    pub struct __Variants<T, T2>(core::marker::PhantomData<(T, T2)>);
    impl<T, T2, __C> ::serialization::__private::Edge<__C> for __Variants<T, T2>
    where
        T: ::serialization::__private::Edge<__C>,
        T2: ::serialization::__private::Edge<__C>,
    {
        type First = ::serialization::__private::End<__C, Self>;
        type Second = <<::serialization::__private::End<
            __C,
            __VariantToken<T, T2, { ({ (0) + 1 }) + 1 }>,
        > as core::ops::Add<
            ::serialization::__private::Variant<__VariantToken<T, T2, { (0) + 1 }>, { (0) + 1 }>,
        >>::Output as core::ops::Add<
            ::serialization::__private::Variant<__VariantToken<T, T2, 0>, 0>,
        >>::Output;
    }
    impl<T, T2, const I: usize> ::serialization::__private::Len for __VariantToken<T, T2, I> {
        const SIZE: usize = <A16<T, T2> as ::serialization::__private::Len>::SIZE;
    }
    impl<T, T2, const I: usize> ::serialization::__private::Size for __VariantToken<T, T2, I> {
        const SIZE: usize = <A16<T, T2> as ::serialization::__private::Size>::SIZE;
    }
    #[repr(transparent)]
    pub struct __FieldToken<S, T, const I: usize>(
        core::mem::MaybeUninit<T>,
        core::marker::PhantomData<S>,
    );
    impl<C, S, T, const I: usize> ::serialization::__private::Edge<C> for __FieldToken<S, T, I>
    where
        T: ::serialization::__private::Edge<C>,
    {
        type First = T::First;
        type Second = T::Second;
    }
    impl<S, T, const I: usize> ::serialization::__private::Len for __FieldToken<S, T, I>
    where
        T: ::serialization::__private::Len,
    {
        const SIZE: usize = T::SIZE;
    }
    impl<S, T, const I: usize> ::serialization::__private::Size for __FieldToken<S, T, I>
    where
        T: ::serialization::__private::Size,
    {
        const SIZE: usize = T::SIZE;
    }
    impl<S, T, const I: usize> ::serialization::Encode for __FieldToken<S, T, I>
    where
        T: ::serialization::Encode,
    {
        fn encode<E: ::serialization::Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
            unsafe { self.0.assume_init_ref() }.encode(encoder)
        }
    }
    impl<C, S, S2, T, const I: usize> ::serialization::__private::CompoundWrapper<C, S>
        for __FieldToken<S2, T, I>
    where
        T: ::serialization::__private::CompoundWrapper<C, S>,
    {
        type Compound = <T as ::serialization::__private::CompoundWrapper<C, S>>::Compound;
    }
    impl<S, T, const I: usize> ::serialization::Decode for __FieldToken<S, T, I>
    where
        T: ::serialization::Decode,
    {
        fn decode_in_place<D: ::serialization::Decoder>(
            decoder: &mut D,
            out: &mut core::mem::MaybeUninit<Self>,
        ) -> Result<(), D::Error> {
            T::decode_in_place(decoder, unsafe { core::mem::transmute(out) })
        }
    }
    impl<S, T, const I: usize> ::serialization::__private::Vector for __FieldToken<S, T, I>
    where
        T: ::serialization::__private::Vector,
    {
        type Item = T::Item;
        fn as_iter(&self) -> impl Iterator<Item = &Self::Item> {
            unsafe { self.0.assume_init_ref() }.as_iter()
        }
        fn as_ptr(&self) -> *const Self::Item {
            unsafe { self.0.assume_init_ref() }.as_ptr()
        }
        fn len(&self) -> usize {
            unsafe { self.0.assume_init_ref() }.len()
        }
        fn set_len(&mut self, len: usize) {
            unsafe { self.0.assume_init_mut() }.set_len(len)
        }
    }
    impl<T, T2, __C> ::serialization::__private::Edge<__C> for A16<T, T2>
    where
        T: ::serialization::__private::Edge<__C>,
        T2: ::serialization::__private::Edge<__C>,
    {
        type First = ::serialization::__private::End<__C, A16<T, T2>>;
        type Second = ::serialization::__private::PhantomEdge<
            __C,
            Self,
            (
                ::serialization::__private::Enum<A16<T, T2>, __Variants<T, T2>>,
                ::serialization::__private::End<__C, Self>,
            ),
        >;
    }
    impl<T, T2> ::serialization::__private::Size for A16<T, T2> {
        const SIZE: usize = core::mem::size_of::<A16<T, T2>>();
    }
    impl<T, T2> ::serialization::__private::Len for A16<T, T2> {
        const SIZE: usize = core::mem::size_of::<A16<T, T2>>();
    }
    impl<T, T2, __C, __S> ::serialization::__private::CompoundWrapper<__C, __S> for A16<T, T2>
    where
        Self: ::serialization::__private::CompoundUnwrapper<__C, __S>,
    {
        type Compound =
            <A16<T, T2> as ::serialization::__private::CompoundUnwrapper<__C, __S>>::Output;
    }
    impl<T, T2> ::serialization::Encode for A16<T, T2> {
        fn encode<E: ::serialization::Encoder>(&self, _encoder: &mut E) -> Result<(), E::Error> {
            Ok(())
        }
    }
    impl<T, T2, __C> ::serialization::__private::Edge<__C> for __VariantToken<T, T2, 0usize>
    where
        T: ::serialization::__private::Edge<__C>,
        T2: ::serialization::__private::Edge<__C>,
    {
        type First = ::serialization::__private::End<__C, ()>;
        type Second = <<::serialization::__private::End<
            __C,
            __VariantToken<T, T2, 0usize>,
        > as core::ops::Add<
            ::serialization::__private::Padding<
                __C,
                __VariantToken<T, T2, 0usize>,
                ::serialization::__private::Field<
                    __FieldToken<__VariantToken<T, T2, 0usize>, T, 0>,
                >,
            >,
        >>::Output as core::ops::Add<
            ::serialization::__private::Field<
                __FieldToken<__VariantToken<T, T2, 0usize>, T, 0>,
            >,
        >>::Output;
    }
    const _: () = {
        impl<T, T2> ::serialization::__private::FieldOffset
            for __FieldToken<__VariantToken<T, T2, 0usize>, T, 0>
        where
            [(); __field_offset::<T, T2>()]:,
        {
            type Offset = ::serialization::__private::typenum::Const<{ __field_offset::<T, T2>() }>;
        }
        pub const fn __field_offset<T, T2>() -> usize {
            {
                use core::mem::MaybeUninit;
                unsafe {
                    let origin = {
                        let v0 = MaybeUninit::zeroed().assume_init();
                        let origin: A16<T, T2> = A16::A(v0);
                        MaybeUninit::new(origin)
                    };
                    match origin.assume_init_ref() {
                        A16::A(v0) => ::serialization::__private::sub_ptr(
                            v0 as *const _ as *const u8,
                            origin.assume_init_ref() as *const _ as *const u8,
                        ),
                        _ => ::core::panicking::panic("internal error: entered unreachable code"),
                    }
                }
            }
        }
    };
    impl<T, T2, __C> ::serialization::__private::Edge<__C> for __VariantToken<T, T2, 1usize>
    where
        T: ::serialization::__private::Edge<__C>,
        T2: ::serialization::__private::Edge<__C>,
    {
        type First = ::serialization::__private::End<__C, ()>;
        type Second = <<::serialization::__private::End<
            __C,
            __VariantToken<T, T2, 1usize>,
        > as core::ops::Add<
            ::serialization::__private::Padding<
                __C,
                __VariantToken<T, T2, 1usize>,
                ::serialization::__private::Field<
                    __FieldToken<
                        __VariantToken<T, T2, 1usize>,
                        std::marker::PhantomData<T2>,
                        0,
                    >,
                >,
            >,
        >>::Output as core::ops::Add<
            ::serialization::__private::Field<
                __FieldToken<
                    __VariantToken<T, T2, 1usize>,
                    std::marker::PhantomData<T2>,
                    0,
                >,
            >,
        >>::Output;
    }
    const _: () = {
        impl<T, T2> ::serialization::__private::FieldOffset
            for __FieldToken<__VariantToken<T, T2, 1usize>, std::marker::PhantomData<T2>, 0>
        where
            [(); __field_offset::<T, T2>()]:,
        {
            type Offset = ::serialization::__private::typenum::Const<{ __field_offset::<T, T2>() }>;
        }
        pub const fn __field_offset<T, T2>() -> usize {
            {
                use core::mem::MaybeUninit;
                unsafe {
                    let origin = {
                        let v0 = MaybeUninit::zeroed().assume_init();
                        let origin: A16<T, T2> = A16::B(v0);
                        MaybeUninit::new(origin)
                    };
                    match origin.assume_init_ref() {
                        A16::B(v0) => ::serialization::__private::sub_ptr(
                            v0 as *const _ as *const u8,
                            origin.assume_init_ref() as *const _ as *const u8,
                        ),
                        _ => ::core::panicking::panic("internal error: entered unreachable code"),
                    }
                }
            }
        }
    };
};
#[automatically_derived]
impl<T: ::core::fmt::Debug, T2: ::core::fmt::Debug> ::core::fmt::Debug for A16<T, T2> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            A16::A(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "A", &__self_0)
            }
            A16::B(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "B", &__self_0)
            }
        }
    }
}
#[automatically_derived]
impl<T: ::core::cmp::Eq, T2: ::core::cmp::Eq> ::core::cmp::Eq for A16<T, T2> {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<T>;
        let _: ::core::cmp::AssertParamIsEq<std::marker::PhantomData<T2>>;
    }
}
#[automatically_derived]
impl<T, T2> ::core::marker::StructuralPartialEq for A16<T, T2> {}
#[automatically_derived]
impl<T: ::core::cmp::PartialEq, T2: ::core::cmp::PartialEq> ::core::cmp::PartialEq for A16<T, T2> {
    #[inline]
    fn eq(&self, other: &A16<T, T2>) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (A16::A(__self_0), A16::A(__arg1_0)) => __self_0 == __arg1_0,
                (A16::B(__self_0), A16::B(__arg1_0)) => __self_0 == __arg1_0,
                _ => unsafe { ::core::intrinsics::unreachable() },
            }
    }
}
struct A17<'a> {
    value: PhantomData<&'a ()>,
}
const _: () = {
    impl<'a, __C> ::serialization::__private::Edge<__C> for A17<'a>
    where
        'a: 'static,
    {
        type First = ::serialization::__private::End<__C, A17<'a>>;
        type Second = <<::serialization::__private::End<__C, A17<'a>> as core::ops::Add<
            ::serialization::__private::Padding<
                __C,
                A17<'a>,
                ::serialization::__private::Field<__FieldToken<A17<'a>, PhantomData<&'a ()>, 0>>,
            >,
        >>::Output as core::ops::Add<
            ::serialization::__private::Field<__FieldToken<A17<'a>, PhantomData<&'a ()>, 0>>,
        >>::Output;
    }
    impl<'a> ::serialization::__private::Size for A17<'a>
    where
        'a: 'static,
    {
        const SIZE: usize = core::mem::size_of::<A17<'a>>();
    }
    impl<'a> ::serialization::__private::Len for A17<'a>
    where
        'a: 'static,
    {
        const SIZE: usize = core::mem::size_of::<A17<'a>>();
    }
    impl<'a, __C, __S> ::serialization::__private::CompoundWrapper<__C, __S> for A17<'a>
    where
        'a: 'static,
        Self: ::serialization::__private::CompoundUnwrapper<__C, __S>,
    {
        type Compound =
            <A17<'a> as ::serialization::__private::CompoundUnwrapper<__C, __S>>::Output;
    }
    impl<'a> ::serialization::Encode for A17<'a>
    where
        'a: 'static,
    {
        fn encode<E: ::serialization::Encoder>(&self, _encoder: &mut E) -> Result<(), E::Error> {
            Ok(())
        }
    }
    #[repr(transparent)]
    pub struct __FieldToken<S, T, const I: usize>(
        core::mem::MaybeUninit<T>,
        core::marker::PhantomData<S>,
    );
    impl<C, S, T, const I: usize> ::serialization::__private::Edge<C> for __FieldToken<S, T, I>
    where
        T: ::serialization::__private::Edge<C>,
    {
        type First = T::First;
        type Second = T::Second;
    }
    impl<S, T, const I: usize> ::serialization::__private::Len for __FieldToken<S, T, I>
    where
        T: ::serialization::__private::Len,
    {
        const SIZE: usize = T::SIZE;
    }
    impl<S, T, const I: usize> ::serialization::__private::Size for __FieldToken<S, T, I>
    where
        T: ::serialization::__private::Size,
    {
        const SIZE: usize = T::SIZE;
    }
    impl<S, T, const I: usize> ::serialization::Encode for __FieldToken<S, T, I>
    where
        T: ::serialization::Encode,
    {
        fn encode<E: ::serialization::Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
            unsafe { self.0.assume_init_ref() }.encode(encoder)
        }
    }
    impl<C, S, S2, T, const I: usize> ::serialization::__private::CompoundWrapper<C, S>
        for __FieldToken<S2, T, I>
    where
        T: ::serialization::__private::CompoundWrapper<C, S>,
    {
        type Compound = <T as ::serialization::__private::CompoundWrapper<C, S>>::Compound;
    }
    impl<S, T, const I: usize> ::serialization::Decode for __FieldToken<S, T, I>
    where
        T: ::serialization::Decode,
    {
        fn decode_in_place<D: ::serialization::Decoder>(
            decoder: &mut D,
            out: &mut core::mem::MaybeUninit<Self>,
        ) -> Result<(), D::Error> {
            T::decode_in_place(decoder, unsafe { core::mem::transmute(out) })
        }
    }
    impl<S, T, const I: usize> ::serialization::__private::Vector for __FieldToken<S, T, I>
    where
        T: ::serialization::__private::Vector,
    {
        type Item = T::Item;
        fn as_iter(&self) -> impl Iterator<Item = &Self::Item> {
            unsafe { self.0.assume_init_ref() }.as_iter()
        }
        fn as_ptr(&self) -> *const Self::Item {
            unsafe { self.0.assume_init_ref() }.as_ptr()
        }
        fn len(&self) -> usize {
            unsafe { self.0.assume_init_ref() }.len()
        }
        fn set_len(&mut self, len: usize) {
            unsafe { self.0.assume_init_mut() }.set_len(len)
        }
    }
    const _: () = {
        impl<'a> ::serialization::__private::FieldOffset for __FieldToken<A17<'a>, PhantomData<&'a ()>, 0>
        where
            'a: 'static,
            [(); __field_offset::<'a>()]:,
        {
            type Offset = ::serialization::__private::typenum::Const<{ __field_offset::<'a>() }>;
        }
        pub const fn __field_offset<'a>() -> usize
        where
            'a: 'static,
        {
            {
                use core::mem::MaybeUninit;
                let origin: MaybeUninit<A17<'a>> = MaybeUninit::uninit();
                #[allow(unused_variables)]
                let A17 { value } = unsafe { origin.assume_init_ref() };
                unsafe {
                    ::serialization::__private::sub_ptr(
                        value as *const _ as *const u8,
                        origin.assume_init_ref() as *const _ as *const u8,
                    )
                }
            }
        }
    };
};
#[automatically_derived]
impl<'a> ::core::fmt::Debug for A17<'a> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "A17", "value", &&self.value)
    }
}
#[automatically_derived]
impl<'a> ::core::cmp::Eq for A17<'a> {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<PhantomData<&'a ()>>;
    }
}
#[automatically_derived]
impl<'a> ::core::marker::StructuralPartialEq for A17<'a> {}
#[automatically_derived]
impl<'a> ::core::cmp::PartialEq for A17<'a> {
    #[inline]
    fn eq(&self, other: &A17<'a>) -> bool {
        self.value == other.value
    }
}
struct AAA<T1, T2> {
    a: T1,
    b: T2,
}
const _: () = {
    impl<T1, T2, __C> ::serialization::__private::Edge<__C> for AAA<T1, T2>
    where
        T1: ::serialization::__private::Edge<__C>,
        T2: ::serialization::__private::Edge<__C>,
    {
        type First = ::serialization::__private::End<__C, AAA<T1, T2>>;
        type Second = <<<<::serialization::__private::End<__C, AAA<T1, T2>> as core::ops::Add<
            ::serialization::__private::Padding<
                __C,
                AAA<T1, T2>,
                ::serialization::__private::Field<__FieldToken<AAA<T1, T2>, T2, { (0) + 1 }>>,
            >,
        >>::Output as core::ops::Add<
            ::serialization::__private::Field<__FieldToken<AAA<T1, T2>, T2, { (0) + 1 }>>,
        >>::Output as core::ops::Add<
            ::serialization::__private::Padding<
                __C,
                AAA<T1, T2>,
                ::serialization::__private::Field<__FieldToken<AAA<T1, T2>, T1, 0>>,
            >,
        >>::Output as core::ops::Add<
            ::serialization::__private::Field<__FieldToken<AAA<T1, T2>, T1, 0>>,
        >>::Output;
    }
    impl<T1, T2> ::serialization::__private::Size for AAA<T1, T2> {
        const SIZE: usize = core::mem::size_of::<AAA<T1, T2>>();
    }
    impl<T1, T2> ::serialization::__private::Len for AAA<T1, T2> {
        const SIZE: usize = core::mem::size_of::<AAA<T1, T2>>();
    }
    impl<T1, T2, __C, __S> ::serialization::__private::CompoundWrapper<__C, __S> for AAA<T1, T2>
    where
        Self: ::serialization::__private::CompoundUnwrapper<__C, __S>,
    {
        type Compound =
            <AAA<T1, T2> as ::serialization::__private::CompoundUnwrapper<__C, __S>>::Output;
    }
    impl<T1, T2> ::serialization::Encode for AAA<T1, T2> {
        fn encode<E: ::serialization::Encoder>(&self, _encoder: &mut E) -> Result<(), E::Error> {
            Ok(())
        }
    }
    #[repr(transparent)]
    pub struct __FieldToken<S, T, const I: usize>(
        core::mem::MaybeUninit<T>,
        core::marker::PhantomData<S>,
    );
    impl<C, S, T, const I: usize> ::serialization::__private::Edge<C> for __FieldToken<S, T, I>
    where
        T: ::serialization::__private::Edge<C>,
    {
        type First = T::First;
        type Second = T::Second;
    }
    impl<S, T, const I: usize> ::serialization::__private::Len for __FieldToken<S, T, I>
    where
        T: ::serialization::__private::Len,
    {
        const SIZE: usize = T::SIZE;
    }
    impl<S, T, const I: usize> ::serialization::__private::Size for __FieldToken<S, T, I>
    where
        T: ::serialization::__private::Size,
    {
        const SIZE: usize = T::SIZE;
    }
    impl<S, T, const I: usize> ::serialization::Encode for __FieldToken<S, T, I>
    where
        T: ::serialization::Encode,
    {
        fn encode<E: ::serialization::Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
            unsafe { self.0.assume_init_ref() }.encode(encoder)
        }
    }
    impl<C, S, S2, T, const I: usize> ::serialization::__private::CompoundWrapper<C, S>
        for __FieldToken<S2, T, I>
    where
        T: ::serialization::__private::CompoundWrapper<C, S>,
    {
        type Compound = <T as ::serialization::__private::CompoundWrapper<C, S>>::Compound;
    }
    impl<S, T, const I: usize> ::serialization::Decode for __FieldToken<S, T, I>
    where
        T: ::serialization::Decode,
    {
        fn decode_in_place<D: ::serialization::Decoder>(
            decoder: &mut D,
            out: &mut core::mem::MaybeUninit<Self>,
        ) -> Result<(), D::Error> {
            T::decode_in_place(decoder, unsafe { core::mem::transmute(out) })
        }
    }
    impl<S, T, const I: usize> ::serialization::__private::Vector for __FieldToken<S, T, I>
    where
        T: ::serialization::__private::Vector,
    {
        type Item = T::Item;
        fn as_iter(&self) -> impl Iterator<Item = &Self::Item> {
            unsafe { self.0.assume_init_ref() }.as_iter()
        }
        fn as_ptr(&self) -> *const Self::Item {
            unsafe { self.0.assume_init_ref() }.as_ptr()
        }
        fn len(&self) -> usize {
            unsafe { self.0.assume_init_ref() }.len()
        }
        fn set_len(&mut self, len: usize) {
            unsafe { self.0.assume_init_mut() }.set_len(len)
        }
    }
    const _: () = {
        impl<T1, T2> ::serialization::__private::FieldOffset for __FieldToken<AAA<T1, T2>, T1, 0>
        where
            [(); __field_offset::<T1, T2>()]:,
        {
            type Offset =
                ::serialization::__private::typenum::Const<{ __field_offset::<T1, T2>() }>;
        }
        pub const fn __field_offset<T1, T2>() -> usize {
            {
                use core::mem::MaybeUninit;
                let origin: MaybeUninit<AAA<T1, T2>> = MaybeUninit::uninit();
                #[allow(unused_variables)]
                let AAA { a, b } = unsafe { origin.assume_init_ref() };
                unsafe {
                    ::serialization::__private::sub_ptr(
                        a as *const _ as *const u8,
                        origin.assume_init_ref() as *const _ as *const u8,
                    )
                }
            }
        }
    };
    const _: () = {
        impl<T1, T2> ::serialization::__private::FieldOffset for __FieldToken<AAA<T1, T2>, T2, { (0) + 1 }>
        where
            [(); __field_offset::<T1, T2>()]:,
        {
            type Offset =
                ::serialization::__private::typenum::Const<{ __field_offset::<T1, T2>() }>;
        }
        pub const fn __field_offset<T1, T2>() -> usize {
            {
                use core::mem::MaybeUninit;
                let origin: MaybeUninit<AAA<T1, T2>> = MaybeUninit::uninit();
                #[allow(unused_variables)]
                let AAA { a, b } = unsafe { origin.assume_init_ref() };
                unsafe {
                    ::serialization::__private::sub_ptr(
                        b as *const _ as *const u8,
                        origin.assume_init_ref() as *const _ as *const u8,
                    )
                }
            }
        }
    };
};
#[automatically_derived]
impl<T1: ::core::fmt::Debug, T2: ::core::fmt::Debug> ::core::fmt::Debug for AAA<T1, T2> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(f, "AAA", "a", &self.a, "b", &&self.b)
    }
}
#[automatically_derived]
impl<T1: ::core::cmp::Eq, T2: ::core::cmp::Eq> ::core::cmp::Eq for AAA<T1, T2> {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<T1>;
        let _: ::core::cmp::AssertParamIsEq<T2>;
    }
}
#[automatically_derived]
impl<T1, T2> ::core::marker::StructuralPartialEq for AAA<T1, T2> {}
#[automatically_derived]
impl<T1: ::core::cmp::PartialEq, T2: ::core::cmp::PartialEq> ::core::cmp::PartialEq
    for AAA<T1, T2>
{
    #[inline]
    fn eq(&self, other: &AAA<T1, T2>) -> bool {
        self.a == other.a && self.b == other.b
    }
}
