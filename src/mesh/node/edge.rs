use std::marker::PhantomData;

pub trait Edge {
    /// Indicate leaf
    type First: Edge = ();

    /// Indicate another edge
    type Second: Edge = ();
}

/// Generic type `S` represents a struct containing a edges.
pub struct PhantomEdge<S, T>(PhantomData<(S, T)>);

impl<S, First, Second> Edge for PhantomEdge<S, (First, Second)>
where
    First: Edge,
    Second: Edge,
{
    type First = First;

    type Second = Second;
}
