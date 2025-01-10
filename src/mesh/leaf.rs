use std::{marker::PhantomData, ops::Add};

use super::edge::{Edge, PhantomEdge};

/// Generic type `S` represents a struct containing a edges.
pub struct PhantomLeaf<S, T>(PhantomData<(S, T)>);

impl<S, T> Edge for PhantomLeaf<S, T>
where
    T: Edge,
{
    type First = T::First;

    type Second = T::Second;
}

impl<S, A, B> Add<B> for PhantomLeaf<S, A> {
    type Output = PhantomEdge<S, (A, B)>;

    fn add(self, _rhs: B) -> Self::Output {
        unreachable!()
    }
}