#![allow(warnings)]
use std::mem::MaybeUninit;

use serialization::{
    __private::{Edge, End, FieldOffset, PhantomEdge, sub_ptr},
    impl_field_token, meshup, offset_of_enum, wrap_brace,
};

pub enum A {
    V1(u32),
    V2(u8),
    V3(u8, i16),
    V4,
    V5 {},
    V6 { value: u32 },
    V7 { value: u32, value2: i32 },
}

const _: () = {
    struct __VariantToken<const I: usize>;
    impl<const I: usize> __VariantToken<I> {
        fn get_value() -> A {
            #[allow(invalid_value)]
            A::V1(unsafe { MaybeUninit::uninit().assume_init() })
        }
    }

    impl_field_token!();

    impl FieldOffset for __VariantToken<0> {
        type Offset = typenum::Const<{ offset_of_enum!(parentheses, A, {}, V1, (v0), v0) }>;
    }

    impl Edge for __VariantToken<0> {
        type First = End<Self>;

        type Second = meshup!(0, (__VariantToken), {0}; {u32});
    }

    impl FieldOffset for __VariantToken<1> {
        type Offset = typenum::Const<{ offset_of_enum!(parentheses, A, {}, V2, (v0), v0) }>;
    }

    impl Edge for __VariantToken<1> {
        type First = End<Self>;

        type Second = meshup!(0, (__VariantToken), {0}; {u32});
    }
};
