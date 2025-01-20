use std::{marker::PhantomData, ops::Add};

use super::{
    edge::{Edge, PhantomEdge},
    end::End,
};

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

impl<S, T> Add<End<S>> for PhantomLeaf<T> {
    type Output = PhantomEdge<S, (T, End<S>)>;

    fn add(self, _rhs: End<S>) -> Self::Output {
        unreachable!()
    }
}
