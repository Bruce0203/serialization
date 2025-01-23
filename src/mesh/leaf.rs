use std::{marker::PhantomData, ops::Add};

use super::{
    edge::{Edge, PhantomEdge},
    end::End,
};

/// Generic type `S` represents a struct containing a edges.
pub struct PhantomLeaf<T>(PhantomData<T>);

impl<C, T> Edge<C> for PhantomLeaf<T>
where
    T: Edge<C>,
{
    type First = T::First;

    type Second = T::Second;
}

impl<C, S, A, B, T> Add<PhantomEdge<C, S, (A, B)>> for PhantomLeaf<T> {
    type Output = PhantomEdge<C, S, (T, PhantomEdge<C, S, (A, B)>)>;

    fn add(self, _rhs: PhantomEdge<C, S, (A, B)>) -> Self::Output {
        unreachable!()
    }
}

impl<C, S, T> Add<End<C, S>> for PhantomLeaf<T> {
    type Output = PhantomEdge<C, S, (T, End<C, S>)>;

    fn add(self, _rhs: End<C, S>) -> Self::Output {
        unreachable!()
    }
}
