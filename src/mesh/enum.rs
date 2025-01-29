use std::{
    mem::{discriminant, transmute, Discriminant, MaybeUninit},
    num::NonZeroUsize,
    ops::Add,
};

use crate::{Codec, Decode, Decoder, Encode, Encoder};

use super::{
    edge::{Edge, PhantomEdge},
    end::End,
    field::FieldOffset,
    flatten::CompoundWrapper,
    len::{Len, Size},
    prelude::Vectored,
    segment::{Mesh, SegmentCodec, SegmentWalker},
};

pub struct Enum<T>(MaybeUninit<T>);

impl<T> Encode for Enum<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        let discriminant = discriminant(self);
        Ok(())
    }
}

impl<T> Decode for Enum<T> {
    fn decode_in_place<D: Decoder>(
        decoder: &mut D,
        out: &mut MaybeUninit<Self>,
    ) -> Result<(), D::Error> {
        todo!()
    }
}

impl<C, S, T> Add<End<C, S>> for Enum<T> {
    type Output = PhantomEdge<C, S, (Enum<T>, End<C, S>)>;

    fn add(self, rhs: End<C, S>) -> Self::Output {
        unreachable!()
    }
}

impl<C, S, B, T> Add<PhantomEdge<C, S, B>> for Enum<T> {
    type Output = PhantomEdge<C, S, (Enum<T>, PhantomEdge<C, S, B>)>;

    fn add(self, rhs: PhantomEdge<C, S, B>) -> Self::Output {
        unreachable!()
    }
}

impl<T> FieldOffset for Enum<T>
where
    T: FieldOffset,
{
    type Offset = T::Offset;
}

impl<C, T> Edge<C> for Enum<T> {
    type First = End<C, Self>;

    type Second = End<C, Self>;
}

impl<T> Len for Enum<T>
where
    T: Size,
{
    const SIZE: usize = T::SIZE;
}

impl<C, S, T> CompoundWrapper<C, S> for Enum<T> {
    type Compound = Enum<T>;
}
