use std::marker::PhantomData;

use super::{
    compound::{Compound, CompoundWrapper},
    edge::{Edge, PhantomEdge},
    len::Len,
};

pub trait FieldOffset {
    type Offset;
}

pub struct PhantomField<S, T, const I: usize>(PhantomData<(S, T)>);

impl<S, T, const I: usize> Edge for PhantomField<S, T, I>
where
    T: Edge,
{
    type First = T::First;

    type Second = T::Second;
}

pub trait FieldWrapper<T> {
    type Output;
}

impl<S, T, const I: usize> FieldOffset for PhantomField<S, T, I>
where
    T: FieldOffset,
{
    type Offset = T::Offset;
}

impl<S, T, const I: usize> CompoundWrapper<S> for PhantomField<S, T, I>
where
    T: CompoundWrapper<S>,
{
    type Compound = T::Compound;
}

impl<S, T, const I: usize> Len for PhantomField<S, T, I>
where
    T: Len,
{
    const SIZE: usize = T::SIZE;
}
