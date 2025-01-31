use std::{marker::PhantomData, ops::Add};

use super::{
    edge::{Edge, PhantomEdge},
    end::End,
    padding::ConstifyPadding,
    sort::Sorted,
};

/// newtype of `PhantomEdge<S, T>` that represents its the root of a struct
pub struct Compound<S, T>(PhantomData<(S, T)>);

pub trait CompoundWrapper<C, S> {
    /// Convert `Edge` to be wrapped as `Compound` or not
    type Compound;
}

pub trait Flatten<S> {
    type Output;
}

pub trait CompoundUnwrapper<C, S> {
    type Output;
}

impl<C, S, T> CompoundUnwrapper<C, S> for T
where
    T: Edge<C, Second: Sorted<Output: ConstifyPadding>>,
{
    type Output =
        Compound<S, <<<T as Edge<C>>::Second as Sorted>::Output as ConstifyPadding>::Output>;
}

impl<C, S, S2, A, B> Flatten<S> for PhantomEdge<C, S2, (A, B)>
where
    A: CompoundWrapper<C, S, Compound: Add<<B as Flatten<S>>::Output>>,
    B: Flatten<S>,
{
    type Output =
        <<A as CompoundWrapper<C, S>>::Compound as Add<<B as Flatten<S>>::Output>>::Output;
}

impl<C, S, S2> Flatten<S> for End<C, S2> {
    type Output = End<C, S>;
}

impl<C, S, S2, B> Add<B> for Compound<S, End<C, S2>> {
    type Output = B;

    fn add(self, _rhs: B) -> Self::Output {
        unreachable!()
    }
}

impl<Codec, S, S2, A, B, C> Add<C> for Compound<S, PhantomEdge<Codec, S2, (A, B)>>
where
    A: CompoundWrapper<Codec, S>,
    Compound<S, B>: Add<C>,
    <A as CompoundWrapper<Codec, S>>::Compound: Add<<Compound<S, B> as Add<C>>::Output>,
{
    type Output = <<A as CompoundWrapper<Codec, S>>::Compound as Add<
        <Compound<S, B> as Add<C>>::Output,
    >>::Output;

    fn add(self, _rhs: C) -> Self::Output {
        unreachable!()
    }
}
