use std::{marker::PhantomData, ops::Add};

use super::edge::{Edge, PhantomEdge};

/// Generic type `S` represents a struct containing a edges.
pub struct PhantomLeaf<T>(PhantomData<T>);

impl<T> Edge for PhantomLeaf<T>
where
    T: Edge,
{
    type First = T::First;

    type Second = T::Second;
}

impl<S, A, B, T> Add<PhantomEdge<S, (A, B)>> for PhantomLeaf<T> {
    type Output = PhantomEdge<S, (T, PhantomEdge<S, (A, B)>)>;

    fn add(self, _rhs: PhantomEdge<S, (A, B)>) -> Self::Output {
        unreachable!()
    }
}
