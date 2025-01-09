use std::marker::PhantomData;

use super::{compound::CompoundWrapper, edge::Edge};

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

impl<S, T, const I: usize> CompoundWrapper<S> for PhantomField<S, T, I>
where
    T: CompoundWrapper<S>,
{
    type Compound = T::Compound;
}
