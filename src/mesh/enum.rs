use std::{
    marker::PhantomData,
    mem::{discriminant, transmute, Discriminant, MaybeUninit},
    num::NonZeroUsize,
    ops::Add,
};

use crate::{Codec, Decode, Decoder, Encode, Encoder, EnumIdentifier};

use super::{
    edge::{Edge, PhantomEdge},
    end::End,
    field::FieldOffset,
    flatten::CompoundWrapper,
    len::{Len, Size},
    prelude::Vectored,
    segment::{Mesh, SegmentCodec, SegmentWalker},
};

pub struct Enum<T, V>(PhantomData<(T, V)>);

//TODO impl Into<Id> for &T(MyEnum)
pub trait VariantIndexById: Sized {
    fn index_by_identifier(id: EnumIdentifier<Self>) -> usize;
}

pub trait Name {}

pub struct Variant<T, const I: usize>(PhantomData<T>);

impl<C, T, const I: usize> Edge<C> for Variant<T, I>
where
    T: Edge<C>,
{
    type First = <T as Edge<C>>::First;

    type Second = <T as Edge<C>>::Second;
}

impl<T, V> Encode for Enum<T, V> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        let discriminant = discriminant(self);
        Ok(())
    }
}

impl<T, V> Decode for Enum<T, V>
where
    [(); { size_of::<Discriminant<T>>() }]:,
{
    fn decode_in_place<D: Decoder>(
        decoder: &mut D,
        out: &mut MaybeUninit<Self>,
    ) -> Result<(), D::Error> {
        decoder.read_array::<u8, { size_of::<Discriminant<T>>() }>(unsafe { transmute(out) });
        todo!()
    }
}

impl<C, S, T, V> Add<End<C, S>> for Enum<T, V> {
    type Output = PhantomEdge<C, S, (Enum<T, V>, End<C, S>)>;

    fn add(self, rhs: End<C, S>) -> Self::Output {
        unreachable!()
    }
}

impl<C, S, B, T, V> Add<PhantomEdge<C, S, B>> for Enum<T, V> {
    type Output = PhantomEdge<C, S, (Enum<T, V>, PhantomEdge<C, S, B>)>;

    fn add(self, rhs: PhantomEdge<C, S, B>) -> Self::Output {
        unreachable!()
    }
}

impl<T, V> FieldOffset for Enum<T, V>
where
    T: FieldOffset,
{
    type Offset = T::Offset;
}

impl<C, T, V> Edge<C> for Enum<T, V> {
    type First = End<C, Self>;

    type Second = End<C, Self>;
}

impl<T, V> Len for Enum<T, V>
where
    T: Size,
{
    const SIZE: usize = T::SIZE;
}

impl<V, C, S, T> CompoundWrapper<C, S> for Enum<T, V> {
    type Compound = Enum<T, V>;
}
