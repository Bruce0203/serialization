use std::{marker::PhantomData, ops::Add};

use crate::{Decode, Encode};

use super::{
    edge::{Edge, PhantomEdge},
    end::End,
    field::FieldOffset,
    flatten::CompoundWrapper,
    len::{Len, Size, UNSIZED},
};

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
        encoder.encode_vec_len(self.0.len())?;
        Ok(())
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

impl<C, S, S2> Add<End<C, S2>> for Vectored<S> {
    type Output = PhantomEdge<C, S, (Vectored<S>, End<C, S>)>;

    fn add(self, _rhs: End<C, S2>) -> Self::Output {
        unreachable!()
    }
}

impl<C, S, T, B> Add<PhantomEdge<C, S, B>> for Vectored<T> {
    type Output = PhantomEdge<C, S, (Vectored<T>, PhantomEdge<C, S, B>)>;

    fn add(self, _rhs: PhantomEdge<C, S, B>) -> Self::Output {
        unreachable!()
    }
}

impl<T> FieldOffset for Vectored<T>
where
    T: FieldOffset,
{
    type Offset = T::Offset;
}

impl<C, T> Edge<C> for Vectored<T>
where
    T: Vector<Item: Edge<C>>,
{
    type First = End<C, Self>;

    type Second = <<T as Vector>::Item as Edge<C>>::Second;
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

impl<C, S, T> CompoundWrapper<C, S> for Vectored<T> {
    type Compound = Vectored<T>;
}
