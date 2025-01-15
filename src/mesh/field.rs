use std::{marker::PhantomData, ops::Add};

use super::{
    flatten::{Compound, CompoundWrapper},
    edge::{Edge, PhantomEdge},
    leaf::PhantomLeaf,
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

impl<T> FieldOffset for Field<T>
where
    T: FieldOffset,
{
    type Offset = T::Offset;
}

impl<S, T> CompoundWrapper<S> for Field<T>
where
    T: CompoundWrapper<S>,
    Field<<T as CompoundWrapper<S>>::Compound>: FieldUnwrapper,
{
    type Compound = <Field<T::Compound> as FieldUnwrapper>::Output;
}

impl<T> Len for Field<T>
where
    T: Len,
{
    const SIZE: usize = T::SIZE;
}

impl<S, A, B, T> Add<PhantomEdge<S, (A, B)>> for Field<T> {
    type Output = PhantomEdge<S, (Field<T>, PhantomEdge<S, (A, B)>)>;

    fn add(self, _rhs: PhantomEdge<S, (A, B)>) -> Self::Output {
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
