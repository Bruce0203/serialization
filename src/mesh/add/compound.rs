use std::{marker::PhantomData, ops::Add};

use typenum::U0;

use crate::{Edge, FieldOffset, PhantomEdge, PhantomField, PhantomLeaf};

/// newtype of `PhantomEdge<S, T>` that represents its the root of a struct
pub struct Compound<S, T>(PhantomData<(S, T)>);

pub trait CompoundWrapper<S> {
    /// Convert `Edge` to be wrapped as `Compound` or not
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

impl<S, S2, A, B, const I: usize> Add<B> for Compound<S, PhantomField<S2, A, I>>
where
    A: Edge,
    B: Edge,
    Compound<S, B>: Add<B>,
{
    type Output = PhantomEdge<S, (A, <Compound<S, B> as Add<B>>::Output)>;

    fn add(self, _rhs: B) -> Self::Output {
        unreachable!()
    }
}

impl<S, S2, T> FieldOffset<S> for Compound<S2, T> {
    type Offset = U0;

