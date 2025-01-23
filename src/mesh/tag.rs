use std::mem::MaybeUninit;

use typenum::Const;

use crate::{
    impl_field_token,
    prelude::{
        CompoundUnwrapper, CompoundWrapper, Edge, End, Field, FieldOffset, Len, PhantomEdge, Size,
        UNSIZED,
    },
    Decode, Encode,
};

use super::prelude::Vector;

pub struct Tag<T>(T);

impl<T> Encode for Tag<T>
where
    T: Vector,
{
    fn encode<E: crate::Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        todo!()
    }
}

impl<T> Decode for Tag<T> {
    fn decode_in_place<D: crate::Decoder>(
        decoder: &mut D,
        out: &mut MaybeUninit<Self>,
    ) -> Result<(), D::Error> {
        todo!()
    }
}

const _: () = {
    impl_field_token!();

    impl<T> FieldOffset for __FieldToken<Tag<T>, u8, 0> {
        type Offset = Const<0>;
    }

    impl<T> FieldOffset for __FieldToken<Tag<T>, T, 1> {
        type Offset = Const<0>;
    }

    impl<T> Edge for Tag<T>
    where
        T: Edge,
    {
        type First = End<Self>;

        type Second = PhantomEdge<Self, (Field<__FieldToken<Self, u8, 0>>, End<Self>)>;
    }

    impl<T> Len for Tag<T>
    where
        T: Len,
    {
        const SIZE: usize = UNSIZED;
    }

    impl<T> Size for Tag<T>
    where
        T: Size,
    {
        const SIZE: usize = 0;
    }

    impl<S, T> CompoundWrapper<S> for Tag<T>
    where
        T: Edge,
    {
        type Compound = <Self as CompoundUnwrapper<S>>::Output;
    }
};
