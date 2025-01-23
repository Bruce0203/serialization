use std::ops::Add;

use super::{
    edge::{Edge, PhantomEdge},
    flatten::{Compound, CompoundWrapper},
    leaf::PhantomLeaf,
    len::{Len, Size},
    prelude::Vectored,
};

pub trait FieldOffset {
    type Offset;
}

pub struct Field<T>(T);

impl<C, T> Edge<C> for Field<T>
where
    T: Edge<C>,
{
    type First = T::First;

    type Second = T::Second;
}

impl<T> FieldOffset for Field<T>
where
    T: FieldOffset,
{
    type Offset = T::Offset;
}

impl<C, S, T> CompoundWrapper<C, S> for Field<T>
where
    T: CompoundWrapper<C, S>,
    Field<<T as CompoundWrapper<C, S>>::Compound>: FieldUnwrapper,
{
    type Compound = <Field<T::Compound> as FieldUnwrapper>::Output;
}

impl<T> Len for Field<T>
where
    T: Len,
{
    const SIZE: usize = T::SIZE;
}

impl<T> Size for Field<T>
where
    T: Size,
{
    const SIZE: usize = T::SIZE;
}

impl<C, S, A, B, T> Add<PhantomEdge<C, S, (A, B)>> for Field<T> {
    type Output = PhantomEdge<C, S, (Field<T>, PhantomEdge<C, S, (A, B)>)>;

    fn add(self, _rhs: PhantomEdge<C, S, (A, B)>) -> Self::Output {
        unreachable!()
    }
}

pub trait FieldUnwrapper {
    type Output;
}

impl<T> FieldUnwrapper for Field<PhantomLeaf<T>> {
    type Output = PhantomLeaf<Field<T>>;
}

impl<S, T> FieldUnwrapper for Field<Compound<S, T>> {
    type Output = Compound<S, T>;
}

impl<T> FieldUnwrapper for Field<Vectored<T>> {
    type Output = Vectored<T>;
}
