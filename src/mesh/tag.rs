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

    impl<C, T> Edge<C> for Tag<T>
    where
        T: Edge<C>,
    {
        type First = End<C, Self>;

        type Second = PhantomEdge<C, Self, (Field<__FieldToken<Self, u8, 0>>, End<C, Self>)>;
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

    impl<C, S, T> CompoundWrapper<C, S> for Tag<T>
    where
        T: Edge<C>,
    {
        type Compound = <Self as CompoundUnwrapper<C, S>>::Output;
    }
};
