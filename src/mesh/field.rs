use std::marker::PhantomData;

use super::{CompoundWrapper, PhantomEdge};

pub trait FieldOffset<S> {
    const OFFSET: usize;
}

pub struct PhantomField<S, T, const I: usize>(PhantomData<(S, T)>);

impl<S, S2, A, B> FieldOffset<S> for PhantomEdge<S2, (A, B)>
where
    A: FieldOffset<S>,
{
    const OFFSET: usize = A::OFFSET;
}

impl<S, T, const I: usize> CompoundWrapper<S> for PhantomField<S, T, I>
where
    T: CompoundWrapper<S>,
{
    type Compound = T::Compound;
}
