use std::marker::PhantomData;

use typenum::Unsigned;

use super::{Compound, CompoundWrapper, Edge, PhantomEdge, PhantomLeaf};

pub trait FieldOffset<S> {
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

pub trait UnwrapField {
    type Output;
}

impl<S, T, const I: usize> UnwrapField for PhantomField<S, T, I> {
    type Output = T;
}
