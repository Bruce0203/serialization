#![allow(warnings)]
use std::mem::MaybeUninit;

use serialization::{
    __private::{
        sub_ptr, CompoundWrapper, Edge, End, Enum, FieldOffset, Len, PhantomEdge, Size, UNSIZED,
    },
    impl_field_token, meshup, offset_of_enum, wrap_brace,
};

 enum A {
    V1(u32),
    V2(u8),
    V3(u8, i16),
    V4,
    V5 {},
    V6 { value: u32 },
    V7 { value: u32, value2: i32 },
}

const _: () = {
    impl<C> Edge<C> for A {
        type First = End<C, Self>;

        type Second = PhantomEdge<C, Self, (Enum<Self>, End<C, Self>)>;
    }

    impl Size for A {
        const SIZE: usize = core::mem::size_of::<A>();
    }

    impl Len for A {
        const SIZE: usize = UNSIZED;
    }

    struct __VariantToken<const I: usize>;
    impl<const I: usize> __VariantToken<I> {
        fn get_value() -> A {
            #[allow(invalid_value)]
            A::V1(unsafe { MaybeUninit::uninit().assume_init() })
        }
    }

    impl_field_token!();

    impl FieldOffset for __FieldToken<__VariantToken<0>, u32, 0> {
        type Offset = typenum::Const<{ offset_of_enum!(parentheses, A, {}, V1, (v0), v0) }>;
    }

    impl<__C> Edge<__C> for __VariantToken<0> {
        type First = End<__C, Self>;

        type Second = meshup!(0, (__VariantToken), {0}; {u32});
    }

    impl FieldOffset for __VariantToken<1> {
        type Offset = typenum::Const<{ offset_of_enum!(parentheses, A, {}, V2, (v0), v0) }>;
    }

    impl FieldOffset for __VariantToken<5> {
        type Offset = typenum::Const<{ offset_of_enum!(brace, A, {}, V6, (value), value) }>;
    }

    fn asdf() {
        use core::mem::MaybeUninit;
        unsafe {
            let origin = {
                let value = MaybeUninit::zeroed().assume_init();
                let origin= wrap_brace!(brace, (A::V6), value);
                MaybeUninit::new(origin)
            };
        }
    }

    impl<__C> Edge<__C> for __VariantToken<1> {
        type First = End<__C, Self>;

        type Second = meshup!(0, (__VariantToken), {0}; {u32});
    }
};
