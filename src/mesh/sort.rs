use std::{marker::PhantomData, ops::Add};

use typenum::{B0, B1, IsLess, ToUInt};

use super::{
    edge::{Edge, PhantomEdge}, end::End, r#enum::Enum, field::{Field, FieldOffset}, leaf::PhantomLeaf, prelude::Vectored
};

pub trait Sorted {
    type Output;
}

impl<C, S, A, B> Sorted for PhantomEdge<C, S, (A, B)>
where
    B: Sorted,
    PhantomOrder<C, S, B::Output>: Add<A>,
{
    type Output = <PhantomOrder<C, S, <B as Sorted>::Output> as Add<A>>::Output;
}

impl<C, S> Sorted for End<C, S> {
    type Output = End<C, S>;
}

pub trait Order<T> {
    type Output;
}

pub struct PhantomOrder<C, S, T>(PhantomData<(C, S, T)>);

impl<C, S, A, B> Order<B1> for PhantomEdge<C, S, (A, B)> {
    type Output = PhantomEdge<C, S, (A, B)>;
}

impl<C, S, A, B> Order<B0> for PhantomEdge<C, S, (A, B)> {
    type Output = PhantomEdge<C, S, (B, A)>;
}

impl<C, S, S2, B> Add<B> for PhantomOrder<C, S, End<C, S2>> {
    type Output = PhantomEdge<C, S, (B, End<C, S2>)>;

    fn add(self, _rhs: B) -> Self::Output {
        unreachable!()
    }
}

impl<C, S, A, B> Add<B> for PhantomOrder<C, S, PhantomLeaf<A>>
where
    A: FieldOffset<Offset: ToUInt>,
    B: FieldOffset<Offset: ToUInt>,
    <<A as FieldOffset>::Offset as ToUInt>::Output:
        IsLess<<<B as FieldOffset>::Offset as ToUInt>::Output>,
    PhantomEdge<C, S, (A, B)>: Order<
        <<<A as FieldOffset>::Offset as ToUInt>::Output as IsLess<
            <<B as FieldOffset>::Offset as ToUInt>::Output,
        >>::Output,
    >,
{
    type Output = <PhantomEdge<C, S, (A, B)> as Order<
        <<A::Offset as ToUInt>::Output as IsLess<<B::Offset as ToUInt>::Output>>::Output,
    >>::Output;

    fn add(self, _rhs: B) -> Self::Output {
        unreachable!()
    }
}

impl<C, S, A, B> Add<B> for PhantomOrder<C, S, Field<A>>
where
    B: FieldOffset<Offset: ToUInt>,
    Field<A>: FieldOffset<Offset: ToUInt>,
    <<Field<A> as FieldOffset>::Offset as ToUInt>::Output:
        IsLess<<<B as FieldOffset>::Offset as ToUInt>::Output>,
    PhantomEdge<C, S, (Field<A>, B)>: Order<
        <<<Field<A> as FieldOffset>::Offset as ToUInt>::Output as IsLess<
            <<B as FieldOffset>::Offset as ToUInt>::Output,
        >>::Output,
    >,
{
    type Output = <PhantomEdge<C, S, (Field<A>, B)> as Order<
        <<<Field<A> as FieldOffset>::Offset as ToUInt>::Output as IsLess<
            <B::Offset as ToUInt>::Output,
        >>::Output,
    >>::Output;

    fn add(self, _rhs: B) -> Self::Output {
        unreachable!()
    }
}

impl<C, S, A, B> Add<B> for PhantomOrder<C, S, Vectored<A>>
where
    B: FieldOffset<Offset: ToUInt>,
    Vectored<A>: FieldOffset<Offset: ToUInt>,
    <<Vectored<A> as FieldOffset>::Offset as ToUInt>::Output:
        IsLess<<<B as FieldOffset>::Offset as ToUInt>::Output>,
    PhantomEdge<C, S, (Vectored<A>, B)>: Order<
        <<<Vectored<A> as FieldOffset>::Offset as ToUInt>::Output as IsLess<
            <<B as FieldOffset>::Offset as ToUInt>::Output,
        >>::Output,
    >,
{
    type Output = <PhantomEdge<C, S, (Vectored<A>, B)> as Order<
        <<<Vectored<A> as FieldOffset>::Offset as ToUInt>::Output as IsLess<
            <B::Offset as ToUInt>::Output,
        >>::Output,
    >>::Output;

    fn add(self, _rhs: B) -> Self::Output {
        unreachable!()
    }
}

impl<C, S, A, B> Add<B> for PhantomOrder<C, S, Enum<A>>
where
    B: FieldOffset<Offset: ToUInt>,
    Enum<A>: FieldOffset<Offset: ToUInt>,
    <<Enum<A> as FieldOffset>::Offset as ToUInt>::Output:
        IsLess<<<B as FieldOffset>::Offset as ToUInt>::Output>,
    PhantomEdge<C, S, (Enum<A>, B)>: Order<
        <<<Enum<A> as FieldOffset>::Offset as ToUInt>::Output as IsLess<
            <<B as FieldOffset>::Offset as ToUInt>::Output,
        >>::Output,
    >,
{
    type Output = <PhantomEdge<C, S, (Enum<A>, B)> as Order<
        <<<Enum<A> as FieldOffset>::Offset as ToUInt>::Output as IsLess<
            <B::Offset as ToUInt>::Output,
        >>::Output,
    >>::Output;

    fn add(self, _rhs: B) -> Self::Output {
        unreachable!()
    }
}


impl<Codec, S, A, B, C> Add<C> for PhantomOrder<Codec, S, PhantomEdge<Codec, S, (A, B)>>
where
    A: FieldOffset<Offset: ToUInt>,
    C: FieldOffset<Offset: ToUInt>,
    <<A as FieldOffset>::Offset as ToUInt>::Output:
        IsLess<<<C as FieldOffset>::Offset as ToUInt>::Output>,
    PhantomEdge<Codec, S, (A, C)>: Order<
            <<<A as FieldOffset>::Offset as ToUInt>::Output as IsLess<
                <<C as FieldOffset>::Offset as ToUInt>::Output,
            >>::Output,
            Output: Edge<Codec>,
        >,
    PhantomOrder<Codec, S, B>: Add<
        <<PhantomEdge<Codec, S, (A, C)> as Order<
            <<<A as FieldOffset>::Offset as ToUInt>::Output as IsLess<
                <<C as FieldOffset>::Offset as ToUInt>::Output,
            >>::Output,
        >>::Output as Edge<Codec>>::Second,
    >,
{
    type Output =
        PhantomEdge<Codec, S, (
            <<PhantomEdge<Codec, S, (A, C)> as Order<<<A::Offset as ToUInt>::Output as IsLess<<C::Offset as ToUInt>::Output>>::Output>>::Output as Edge<Codec>>::First, 
            <PhantomOrder<Codec, S, B> 
            as Add<<<PhantomEdge<Codec, S, (A, C)> as Order<<<A::Offset as ToUInt>::Output as IsLess<<C::Offset as ToUInt>::Output>>::Output>>::Output as Edge<Codec>>::Second>>::Output
        )>;

    fn add(self, _rhs: C) -> Self::Output {
        unreachable!()
    }
}
