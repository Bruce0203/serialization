#![feature(generic_const_exprs)]
#![allow(warnings)]
use std::{
    marker::PhantomData,
    mem::{discriminant, MaybeUninit},
};

use serialization::{
    __private::{
        sub_ptr, CompoundWrapper, Edge, End, Enum, FieldOffset, Instantiate, Len, Mesh,
        PhantomEdge, SegmentCodec, SegmentWalker, Size, Variant, UNSIZED,
    }, impl_field_token, meshup, offset_of_enum, variant_meshup, wrap_brace, Codec
};

enum A<T> {
    V1(u32),
    V2(u8),
    V3(u8, i16),
    V4,
    V5 {},
    V6 { value: u32 },
    V7 { value: u32, value2: i32 },
    V8(T),
}

const _: () = {
    pub struct __Variants;

    impl<C, T> Edge<C> for A<T> {
        type First = End<C, Self>;

        //TODO impl macro for meshup enum variant
        type Second = PhantomEdge<C, Self, (Enum<Self, __Variants>, End<C, Self>)>;
    }

    impl<__C> Edge<__C> for __Variants {
        type First = End<__C, Self>;

        type Second = ();
    }

    impl<T> Size for A<T> {
        const SIZE: usize = core::mem::size_of::<A<T>>();
    }

    impl<T> Len for A<T> {
        const SIZE: usize = UNSIZED;
    }

    struct __VariantToken<T, const I: usize>(PhantomData<T>);
    impl<T, const I: usize> __VariantToken<T, I> {
        fn get_value() -> A<T> {
            #[allow(invalid_value)]
            A::<T>::V1(unsafe { MaybeUninit::uninit().assume_init() })
        }
    }

    impl_field_token!();

    const fn __offset_of_0<T>() -> usize {
        offset_of_enum!(parentheses, A, { T }, V1, (v0), v0)
    }
    impl<T> FieldOffset for __FieldToken<__VariantToken<T, 0>, u32, 0>
    where
        [(); __offset_of_0::<T>()]:,
    {
        type Offset = typenum::Const<{ __offset_of_0::<T>() }>;
    }

    impl<T, __C> Edge<__C> for __VariantToken<T, 0> {
        type First = End<__C, Self>;

        type Second = meshup!(0, (__VariantToken), {T, 0}; {u32});
    }
    impl<T> Size for __VariantToken<T, 0> {
        const SIZE: usize = <A<T> as Size>::SIZE;
    }

    const _: () = {
        const fn __offset_of<T>() -> usize {
            offset_of_enum!(parentheses, A, { T }, V2, (v0), v0)
        }
        impl<T> FieldOffset for __VariantToken<T, 1>
        where
            [(); __offset_of::<T>()]:,
        {
            type Offset = typenum::Const<{ __offset_of::<T>() }>;
        }
    };

    const _: () = {
        const fn __offset_of<T>() -> usize {
            offset_of_enum!(brace, A, { T }, V6, (value), value)
        }
        impl<T> FieldOffset for __VariantToken<T, 5>
        where
            [(); __offset_of::<T>()]:,
        {
            type Offset = typenum::Const<{ __offset_of::<T>() }>;
        }
    };

    // fn asdf() {
    //     use core::mem::MaybeUninit;
    //     unsafe {
    //         let origin = {
    //             let value = MaybeUninit::zeroed().assume_init();
    //             let origin = wrap_brace!(brace, (A::<T>::V6), value);
    //             MaybeUninit::new(origin)
    //         };
    //     }
    // }

    impl<T, __C> Edge<__C> for __VariantToken<T, 1> {
        type First = End<__C, Self>;

        type Second = meshup!(0, (__VariantToken), {T, 0}; {u32});
    }
};
