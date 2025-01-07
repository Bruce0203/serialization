use std::{marker::PhantomData, ops::Add};

use crate::{Edge, Node, PhantomEdge, PhantomLeaf};

/// newtype of `PhantomEdge<S, T>` that represents its the root of a struct
pub struct Compound<S, T>(PhantomData<(S, T)>);

pub trait CompoundWrapper<S> {
    /// Convert `Edge` to be wrapped as `Compound` or just `Self`
    type Compound;
}

impl<S, B> Add<B> for Compound<S, !> {
    type Output = B;

    fn add(self, _rhs: B) -> Self::Output {
        unreachable!()
    }
}

impl<S, T> Edge for Compound<S, T> {}

impl<S, S2, A, B, C> Add<C> for Compound<S, PhantomEdge<S2, (A, B)>>
where
    A: Edge,
    B: Edge,
    Compound<S, B>: Add<C>,
{
    type Output = PhantomEdge<S, (A, <Compound<S, B> as Add<C>>::Output)>;

    fn add(self, _rhs: C) -> Self::Output {
        unreachable!()
    }
}

impl<T, S> CompoundWrapper<S> for T
where
    T: Node,
{
    type Compound = PhantomLeaf<S, Self>;
}
