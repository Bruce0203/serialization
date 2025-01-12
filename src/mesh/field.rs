use std::marker::PhantomData;

use super::{
    compound::{Compound, CompoundWrapper},
    edge::{Edge, PhantomEdge},
    len::Len,
};

pub trait FieldOffset {
    type Offset;
}

pub struct Field<T>(PhantomData<T>);

impl<T> Edge for Field<T>
where
    T: Edge,
{
    type First = T::First;

    type Second = T::Second;
}

pub trait FieldWrapper<T> {
    type Output;
}

impl<T> FieldOffset for Field<T>
where
    T: FieldOffset,
{
    type Offset = T::Offset;
}

impl<S, T> CompoundWrapper<S> for Field<T>
where
    T: CompoundWrapper<S>,
{
    type Compound = T::Compound;
}

impl<T> Len for Field<T>
where
    T: Len,
{
    const SIZE: usize = T::SIZE;
}
