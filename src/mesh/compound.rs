use std::{marker::PhantomData, ops::Add};

use super::{
    edge::{Edge, PhantomEdge},
    end::End,
    leaf::PhantomLeaf,
};

/// newtype of `PhantomEdge<S, T>` that represents its the root of a struct
pub struct Compound<S, T>(PhantomData<(S, T)>);

pub trait CompoundWrapper<S> {
    /// Convert `Edge` to be wrapped as `Compound` or not
    type Compound;
}

pub trait Flatten {
    type Output;
}

impl<S, A, B> Flatten for PhantomEdge<S, (A, B)>
where
    A: CompoundWrapper<S, Compound: Add<<B as Flatten>::Output>>,
    B: Flatten,
{
    type Output = <<A as CompoundWrapper<S>>::Compound as Add<<B as Flatten>::Output>>::Output;
}

impl Flatten for End {
    type Output = End;
}

impl<S, B> Add<B> for Compound<S, End> {
    type Output = B;

    fn add(self, _rhs: B) -> Self::Output {
        unreachable!()
    }
}

impl<S, A, B> Add<B> for Compound<S, PhantomLeaf<S, A>>
where
    A: Edge,
    B: Edge,
{
    type Output = PhantomEdge<S, (A, B)>;

    fn add(self, _rhs: B) -> Self::Output {
        unreachable!()
    }
}

impl<S, S2, A, B, C> Add<C> for Compound<S, PhantomEdge<S2, (A, B)>>
where
    Compound<S, B>: Add<C>,
{
    type Output = PhantomEdge<S, (A, <Compound<S, B> as Add<C>>::Output)>;

    fn add(self, _rhs: C) -> Self::Output {
        unreachable!()
    }
}
