use std::{marker::PhantomData, ops::Add};

use crate::{FieldOffset, IsGreaterOrEqual, Order};

use super::{Edge, PhantomEdge};

/// Generic type `S` represents a struct containing a edges.
pub struct PhantomLeaf<S, T>(PhantomData<(S, T)>);

impl<S, T> Edge for PhantomLeaf<S, T>
where
    T: Edge,
{
    type First = T::First;

    type Second = T::Second;
}

impl<S, A, B> Add<B> for PhantomLeaf<S, A>
where
    A: FieldOffset<S>,
    B: FieldOffset<S>,
    PhantomEdge<S, (A, B)>: Order<{ <PhantomEdge<S, (A, B)> as IsGreaterOrEqual>::OUTPUT }>,
{
    type Output = <PhantomEdge<S, (A, B)> as Order<
        { <PhantomEdge<S, (A, B)> as IsGreaterOrEqual>::OUTPUT },
    >>::Output;

    fn add(self, _rhs: B) -> Self::Output {
        unreachable!()
    }
}

impl<S, S2, T> FieldOffset<S> for PhantomLeaf<S2, T>
where
    T: FieldOffset<S>,
{
    const OFFSET: usize = T::OFFSET;
}
