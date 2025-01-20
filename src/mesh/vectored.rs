use std::{marker::PhantomData, ops::Add};

use crate::Encode;

use super::{
    edge::{Edge, PhantomEdge},
    end::End,
    field::FieldOffset,
    flatten::CompoundWrapper,
    len::{Len, Size, UNSIZED},
};

pub struct Vectored<T, V>(PhantomData<(T, V)>);

pub trait Vector {
    type Item;
    fn as_ptr(&self) -> *const Self::Item;
    fn len(&self) -> usize;
}

impl<T, V> Encode for Vectored<T, V> {
    fn encode<E: crate::Encoder>(&self, _encoder: &mut E) -> Result<(), E::Error> {
        #[cfg(debug_assertions)]
        println!("HI vec<T> vectored!");
        Ok(())
    }
}

impl<S, S2, V> Add<End<S2>> for Vectored<S, V> {
    type Output = PhantomEdge<S, (Vectored<S, V>, End<S>)>;

    fn add(self, _rhs: End<S2>) -> Self::Output {
        unreachable!()
    }
}

impl<S, T, B, V> Add<PhantomEdge<S, B>> for Vectored<T, V> {
    type Output = PhantomEdge<S, (Vectored<T, V>, PhantomEdge<S, B>)>;

    fn add(self, _rhs: PhantomEdge<S, B>) -> Self::Output {
        unreachable!()
    }
}

impl<T, V> FieldOffset for Vectored<T, V>
where
    V: FieldOffset,
{
    type Offset = V::Offset;
}

impl<T, V> Edge for Vectored<T, V>
where
    V: Edge,
{
    type First = End<Self>;

    type Second = V::Second;
}

impl<T, V> Len for Vectored<T, V> {
    const SIZE: usize = UNSIZED;
}

impl<T, V> Size for Vectored<T, V>
where
    T: Size,
{
    const SIZE: usize = T::SIZE;
}

impl<S, T, V> CompoundWrapper<S> for Vectored<T, V> {
    type Compound = Vectored<T, V>;
}
