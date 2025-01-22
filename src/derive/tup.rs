use std::mem::MaybeUninit;

use typenum::Const;

use crate::{
    impl_field_token, meshup,
    prelude::{
        sub_ptr, CompoundUnwrapper, CompoundWrapper, ConstifyPadding, Edge, End, FieldOffset, Len,
        Size, Sorted,
    },
    Decode, Decoder, Encode, Encoder,
};

impl<A, B> Encode for (A, B)
where
    A: Encode,
    B: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        A::encode(&self.0, encoder)?;
        B::encode(&self.1, encoder)?;
        Ok(())
    }
}

impl<A, B> Decode for (A, B) {
    fn decode_in_place<D: crate::Decoder>(
        decoder: &mut D,
        out: &mut MaybeUninit<Self>,
    ) -> Result<(), D::Error> {
        todo!()
    }
}

impl<A, B> Size for (A, B) {
    const SIZE: usize = size_of::<Self>();
}

impl<A, B> Len for (A, B) {
    const SIZE: usize = size_of::<Self>();
}

const _: () = {
    impl_field_token!();

    const _: () = {
        impl<A, B> FieldOffset for __FieldToken<(A, B), A, 0>
        where
            [(); offset_of::<A, B>()]:,
        {
            type Offset = Const<{ offset_of::<A, B>() }>;
        }

        pub const fn offset_of<A, B>() -> usize {
            use core::mem::MaybeUninit;
            let origin: MaybeUninit<(A, B)> = MaybeUninit::uninit();
            unsafe {
                sub_ptr(
                    &origin.assume_init_ref().0 as *const _ as *const u8,
                    origin.assume_init_ref() as *const _ as *const u8,
                )
            }
        }
    };

    const _: () = {
        impl<A, B> FieldOffset for __FieldToken<(A, B), B, 1>
        where
            [(); offset_of::<A, B>()]:,
        {
            type Offset = Const<{ offset_of::<A, B>() }>;
        }

        pub const fn offset_of<A, B>() -> usize {
            use core::mem::MaybeUninit;
            let origin: MaybeUninit<(A, B)> = MaybeUninit::uninit();
            unsafe {
                sub_ptr(
                    &origin.assume_init_ref().1 as *const _ as *const u8,
                    origin.assume_init_ref() as *const _ as *const u8,
                )
            }
        }
    };

    impl<S, A, B> CompoundWrapper<S> for (A, B)
    where
        Self: Edge<Second: Sorted<Output: ConstifyPadding>>,
    {
        type Compound = <Self as CompoundUnwrapper<S>>::Output;
    }

    pub type Tup<A, B> = (A, B);

    impl<A, B> Edge for (A, B)
    where
        A: Edge,
        B: Edge,
    {
        type First = End<Self>;

        type Second = meshup!(0, (Tup), {A, B,}; {A} {B});
    }
};

impl<A, B, C> Encode for (A, B, C)
where
    A: Encode,
    B: Encode,
    C: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        A::encode(&self.0, encoder)?;
        B::encode(&self.1, encoder)?;
        C::encode(&self.2, encoder)?;
        Ok(())
    }
}

impl<A, B, C> Decode for (A, B, C) {
    fn decode_in_place<D: Decoder>(
        decoder: &mut D,
        out: &mut MaybeUninit<Self>,
    ) -> Result<(), D::Error> {
        todo!()
    }
}

impl<A, B, C> Size for (A, B, C) {
    const SIZE: usize = size_of::<Self>();
}

impl<A, B, C> Len for (A, B, C) {
    const SIZE: usize = size_of::<Self>();
}

const _: () = {
    impl_field_token!();

    const _: () = {
        impl<A, B, C> FieldOffset for __FieldToken<(A, B, C), A, 0>
        where
            [(); offset_of::<A, B, C>()]:,
        {
            type Offset = Const<{ offset_of::<A, B, C>() }>;
        }

        pub const fn offset_of<A, B, C>() -> usize {
            use core::mem::MaybeUninit;
            let origin: MaybeUninit<(A, B, C)> = MaybeUninit::uninit();
            unsafe {
                sub_ptr(
                    &origin.assume_init_ref().0 as *const _ as *const u8,
                    origin.assume_init_ref() as *const _ as *const u8,
                )
            }
        }
    };

    const _: () = {
        impl<A, B, C> FieldOffset for __FieldToken<(A, B, C), B, 1>
        where
            [(); offset_of::<A, B, C>()]:,
        {
            type Offset = Const<{ offset_of::<A, B, C>() }>;
        }

        pub const fn offset_of<A, B, C>() -> usize {
            use core::mem::MaybeUninit;
            let origin: MaybeUninit<(A, B, C)> = MaybeUninit::uninit();
            unsafe {
                sub_ptr(
                    &origin.assume_init_ref().1 as *const _ as *const u8,
                    origin.assume_init_ref() as *const _ as *const u8,
                )
            }
        }
    };

    const _: () = {
        impl<A, B, C> FieldOffset for __FieldToken<(A, B, C), C, 2>
        where
            [(); offset_of::<A, B, C>()]:,
        {
            type Offset = Const<{ offset_of::<A, B, C>() }>;
        }

        pub const fn offset_of<A, B, C>() -> usize {
            use core::mem::MaybeUninit;
            let origin: MaybeUninit<(A, B, C)> = MaybeUninit::uninit();
            unsafe {
                sub_ptr(
                    &origin.assume_init_ref().2 as *const _ as *const u8,
                    origin.assume_init_ref() as *const _ as *const u8,
                )
            }
        }
    };

    impl<S, A, B, C> CompoundWrapper<S> for (A, B, C)
    where
        Self: Edge<Second: Sorted<Output: ConstifyPadding>>,
    {
        type Compound = <Self as CompoundUnwrapper<S>>::Output;
    }

    pub type Tup<A, B, C> = (A, B, C);

    impl<A, B, C> Edge for (A, B, C)
    where
        A: Edge,
        B: Edge,
        C: Edge,
    {
        type First = End<Self>;

        type Second = meshup!(0, (Tup), {A, B, C,}; {A} {B} {C});
    }
};
