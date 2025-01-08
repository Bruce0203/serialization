use std::{marker::PhantomData, ops::Add};

use crate::{Edge, FieldOffset, PhantomEdge, PhantomField, PhantomLeaf};

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

impl<S, A, B> IsGreaterOrEqual for PhantomEdge<S, (A, B)>
where
    A: FieldOffset<S>,
    B: FieldOffset<S>,
{
    const OUTPUT: bool = A::OFFSET <= B::OFFSET;
}

pub struct Ordering<S, T>(PhantomData<(S, T)>);

pub trait OrderingWrapper<S> {
    type Ordering;
}

impl<S, B> Add<B> for Ordering<S, !> {
    type Output = B;

    fn add(self, _rhs: B) -> Self::Output {
        unreachable!()
    }
}

impl<S, T> Edge for Ordering<S, T> {}

impl<S, T> FieldOffset<S> for Ordering<S, T>
where
    T: FieldOffset<S>,
{
    const OFFSET: usize = T::OFFSET;
}

impl<S, S2, A, B, C> Add<C> for Ordering<S, PhantomEdge<S2, (A, B)>>
where
    A: Edge + FieldOffset<S>,
    B: Edge,
    C: FieldOffset<S>,
    PhantomEdge<S, (A, C)>: Order<{ <PhantomEdge<S, (A, C)> as IsGreaterOrEqual>::OUTPUT }>,
{
    type Output = PhantomEdge<
        S,
        (
            <PhantomEdge<S, (A, C)> as Order<
                { <PhantomEdge<S, (A, C)> as IsGreaterOrEqual>::OUTPUT },
            >>::Output,
            B,
        ),
    >;

    fn add(self, _rhs: C) -> Self::Output {
        unreachable!()
    }
}

impl<S, T, const I: usize, C> Add<C> for Ordering<S, PhantomField<S, T, I>>
where
    PhantomField<S, T, I>: FieldOffset<S>,
    C: FieldOffset<S>,
    PhantomEdge<S, (Ordering<S, PhantomField<S, T, I>>, C)>:
        Order<{ <PhantomEdge<S, (Self, C)> as IsGreaterOrEqual>::OUTPUT }>,
{
    type Output = <PhantomEdge<S, (Self, C)> as Order<
        { <PhantomEdge<S, (Self, C)> as IsGreaterOrEqual>::OUTPUT },
    >>::Output;

    fn add(self, _rhs: C) -> Self::Output {
        unreachable!()
    }
}

impl<T, S> OrderingWrapper<S> for T {
    type Ordering = PhantomLeaf<S, Self>;
}
