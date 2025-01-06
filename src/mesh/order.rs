use std::marker::PhantomData;

use super::{CompoundWrapper, PhantomEdge};

pub trait IsGreaterOrEqual {
    const OUTPUT: bool;
}

pub trait Order<const IS_ORDERED: bool> {
    type Output;
}

impl<S, A, B> Order<true> for PhantomEdge<S, (A, B)> {
    type Output = PhantomEdge<S, (A, B)>;
}

impl<S, A, B> Order<false> for PhantomEdge<S, (A, B)> {
    type Output = PhantomEdge<S, (B, A)>;
}

pub trait FieldOffset<S> {
    const OFFSET: usize;
}

impl<S, S2, T> FieldOffset<S> for PhantomEdge<S2, T>
where
    T: FieldOffset<S>,
{
    const OFFSET: usize = T::OFFSET;
}

pub struct PhantomField<S, T, const I: usize>(PhantomData<(S, T)>);

impl<S, A, B> IsGreaterOrEqual for PhantomEdge<S, (A, B)>
where
    A: FieldOffset<S>,
    B: FieldOffset<S>,
{
    const OUTPUT: bool = A::OFFSET <= B::OFFSET;
}

impl<S, S2, T, const I: usize> CompoundWrapper<S> for PhantomField<S2, T, I>
where
    T: CompoundWrapper<S>,
{
    type Compound = T::Compound;
}
