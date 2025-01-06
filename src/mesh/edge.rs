use std::{marker::PhantomData, ops::Add};

use super::FieldOffset;

pub trait Edge {
    /// Indicate leaf
    type First: Edge = ();

    /// Indicate another edge
    type Second: Edge = End;
}

pub trait Node {}

/// Generic type `S` represents a struct containing a edges.
pub struct PhantomEdge<S, T>(PhantomData<(S, T)>);

/// newtype of `PhantomEdge<S, T>` that represents its the root of a struct
pub struct Compound<S, T>(PhantomData<(S, T)>);

/// Generic type `S` represents a struct containing a edges.
pub struct PhantomLeaf<S, T>(PhantomData<(S, T)>);

/// Unit type for end of the `Edge` chain
pub struct End;

impl<S> FieldOffset<S> for End {
    const OFFSET: usize = 0;
}

impl<S, T> Edge for PhantomLeaf<S, T>
where
    T: Edge,
{
    type First = T::First;

    type Second = T::Second;
}

impl<S, T, S2> CompoundWrapper<S2> for PhantomLeaf<S, T>
where
    T: CompoundWrapper<S2>,
{
    type Compound = T::Compound;
}

pub trait CompoundWrapper<S> {
    /// Convert `Edge` to be wrapped as `Compound` or just `Self`
    type Compound;
}

impl<T, S> CompoundWrapper<S> for T
where
    T: Node,
{
    type Compound = PhantomEdge<S, Self>;
}

impl Edge for End {
    //TODO ?
    type First = Self;
    //TODO ?
    type Second = Self;
}

impl<S, T> CompoundWrapper<S> for PhantomEdge<S, T>
where
    T: CompoundWrapper<S>,
{
    type Compound = T::Compound;
}

impl<S, First, Second> Edge for PhantomEdge<S, (First, Second)>
where
    First: Edge,
    Second: Edge,
{
    type First = First;

    type Second = Second;
}

impl<S, T> Edge for PhantomEdge<S, T>
where
    T: Edge,
{
    type First = <T as Edge>::First;

    type Second = <T as Edge>::Second;
}

impl<S, A, B> Add<B> for PhantomEdge<S, A> {
    type Output = PhantomEdge<S, (A, B)>;
    fn add(self, _rhs: B) -> Self::Output {
        unreachable!()
    }
}

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

impl<S, B> Add<B> for Compound<S, End> {
    type Output = B;

    fn add(self, _rhs: B) -> Self::Output {
        unreachable!()
    }
}

impl<S, T> Edge for Compound<S, T> {
    type First = ();

    type Second = Self;
}
