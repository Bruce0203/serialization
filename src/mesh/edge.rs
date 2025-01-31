use std::{marker::PhantomData, ops::Add};

pub trait Edge<C> {
    /// Indicate leaf
    type First: Edge<C>;

    /// Indicate another edge
    type Second: Edge<C>;
}

/// Generic type `S` represents a struct containing a edges.
pub struct PhantomEdge<C, S, T>(pub(crate) PhantomData<(C, S, T)>);

impl<C, S, First, Second> Edge<C> for PhantomEdge<C, S, (First, Second)>
where
    First: Edge<C>,
    Second: Edge<C>,
{
    type First = First;

    type Second = Second;
}

impl<C, S, A, B, Rhs> Add<Rhs> for PhantomEdge<C, S, (A, B)> {
    type Output = PhantomEdge<C, S, (Rhs, PhantomEdge<C, S, (A, B)>)>;

    fn add(self, _rhs: Rhs) -> Self::Output {
        unreachable!()
    }
}
