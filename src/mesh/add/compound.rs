use std::{marker::PhantomData, ops::Add};

use typenum::U0;

use crate::{Edge, FieldOffset, PhantomEdge, PhantomLeaf};

/// newtype of `PhantomEdge<S, T>` that represents its the root of a struct
pub struct Compound<S, T>(PhantomData<(S, T)>);

pub trait CompoundWrapper<S> {
    /// Convert `Edge` to be wrapped as `Compound` or not
    type Compound;
}

pub trait Flatten {
    type Output;
}

impl<S, T> Flatten for Compound<S, T>
where
    T: Edge,
    Compound<S, <T as Edge>::First>: Add<<T as Edge>::Second>,
{
    type Output = <Compound<S, T::First> as Add<T::Second>>::Output;
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

impl<S, S2, T> FieldOffset<S> for Compound<S2, T> {
    type Offset = U0;
}
