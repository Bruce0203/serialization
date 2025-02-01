use std::{
    marker::PhantomData,
    mem::{discriminant, transmute, Discriminant, MaybeUninit},
    num::NonZeroUsize,
    ops::Add,
};

use typenum::Const;

use crate::{Codec, Decode, Decoder, Encode, Encoder, EnumVariantIndex};

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

pub struct FrontOffsetToken;

impl FieldOffset for FrontOffsetToken {
    type Offset = Const<0>;
}

impl Size for FrontOffsetToken {
    const SIZE: usize = 0;
}

impl Len for FrontOffsetToken {
    const SIZE: usize = 0;
}

pub struct Variant<T, const I: usize>(PhantomData<T>);

pub trait EnumDiscriminantDecoder<T>: Sized {
    fn decode_enum_discriminant(variant_index: &EnumVariantIndex, out: &mut MaybeUninit<T>);
}

impl<C, T, const I: usize> Edge<C> for Variant<T, I>
where
    T: Edge<C>,
{
    type First = <T as Edge<C>>::First;

    type Second = <T as Edge<C>>::Second;
}

impl<C, S, T, const I: usize> CompoundWrapper<C, S> for Variant<T, I> {
    type Compound = Variant<T, I>;
}

//TODO try change T to S and PhantomEdge's S to S from S2
impl<C, S2, T, const I: usize> Add<End<C, S2>> for Variant<T, I> {
    type Output = PhantomEdge<C, S2, (Variant<T, I>, End<C, S2>)>;

    fn add(self, _rhs: End<C, S2>) -> Self::Output {
        unreachable!()
    }
}

impl<T, V> Encode for Enum<T, V> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
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
        Ok(())
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
