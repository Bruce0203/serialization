use std::{marker::PhantomData, ops::Add};

use crate::{Edge, FieldOffset, Node, PhantomEdge, PhantomLeaf};

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
    const OUTPUT: bool = true; //A::OFFSET <= B::OFFSET;
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

impl<S, S2, A, B, C> Add<C> for Ordering<S, PhantomEdge<S2, (A, B)>>
where
    A: Edge,
    B: Edge,
    Ordering<S, B>: Add<C>,
{
    type Output = PhantomEdge<S, (A, <Ordering<S, B> as Add<C>>::Output)>;

    fn add(self, _rhs: C) -> Self::Output {
        unreachable!()
    }
}

impl<T, S> OrderingWrapper<S> for T
where
    T: Node,
{
    type Ordering = PhantomLeaf<S, Self>;
}
