use std::{marker::PhantomData, ops::Add};

use crate::{Decode, Encode};

use super::{
    edge::{Edge, PhantomEdge},
    end::End,
    field::FieldOffset,
    flatten::CompoundWrapper,
    len::{Len, Size, UNSIZED},
};

//TODO replace V to Vector::Item
#[repr(transparent)]
pub struct Vectored<T>(pub(crate) T);

pub trait Vector {
    type Item;
    fn as_iter(&self) -> impl Iterator<Item = &Self::Item>;
    fn as_ptr(&self) -> *const Self::Item;
    fn len(&self) -> usize;
}

impl<T> Encode for Vectored<T>
where
    T: Vector,
{
    fn encode<E: crate::Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        encoder.encode_vec_len(self.0.len())
    }
}

impl<T> Decode for Vectored<T> {
    fn decode_in_place<D: crate::Decoder>(
        decoder: &mut D,
        out: &mut std::mem::MaybeUninit<Self>,
    ) -> Result<(), D::Error> {
        Ok(())
    }
}

impl<S, S2> Add<End<S2>> for Vectored<S> {
    type Output = PhantomEdge<S, (Vectored<S>, End<S>)>;

    fn add(self, _rhs: End<S2>) -> Self::Output {
        unreachable!()
    }
}

impl<S, T, B> Add<PhantomEdge<S, B>> for Vectored<T> {
    type Output = PhantomEdge<S, (Vectored<T>, PhantomEdge<S, B>)>;

    fn add(self, _rhs: PhantomEdge<S, B>) -> Self::Output {
        unreachable!()
    }
}

impl<T> FieldOffset for Vectored<T>
where
    T: FieldOffset,
{
    type Offset = T::Offset;
}

impl<T> Edge for Vectored<T>
where
    T: Vector<Item: Edge>,
{
    type First = End<Self>;

    type Second = <<T as Vector>::Item as Edge>::Second;
}

impl<T> Len for Vectored<T> {
    const SIZE: usize = UNSIZED;
}

impl<T> Size for Vectored<T>
where
    T: Size,
{
    const SIZE: usize = T::SIZE;
}

impl<S, T> CompoundWrapper<S> for Vectored<T> {
    type Compound = Vectored<T>;
}
