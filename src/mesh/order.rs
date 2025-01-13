use std::{marker::PhantomData, ops::Add};

use typenum::{B0, B1, IsLess, ToUInt};

use super::{
    edge::{Edge, PhantomEdge},
    end::End,
    field::{Field, FieldOffset },
    leaf::PhantomLeaf,
};

pub trait Sorted {
    type Output;
}

impl<S, A, B> Sorted for PhantomEdge<S, (A, B)>
where
    B: Sorted,
    PhantomOrder<S, B::Output>: Add<A>
{
    type Output = <PhantomOrder<S, <B as Sorted>::Output> as Add<A>>::Output;
}

impl<S> Sorted for End<S> {
    type Output = End<S>;
}

pub trait Order<T> {
    type Output;
}

pub struct PhantomOrder<S, T>(PhantomData<(S, T)>);

impl<S, A, B> Order<B1> for PhantomEdge<S, (A, B)> {
    type Output = PhantomEdge<S, (A, B)>;
}

impl<S, A, B> Order<B0> for PhantomEdge<S, (A, B)> {
    type Output = PhantomEdge<S, (B, A)>;
}

impl<S, S2, B> Add<B> for PhantomOrder<S, End<S2>> {
    type Output = PhantomEdge<S, (B, End<S2>)>;

    fn add(self, _rhs: B) -> Self::Output {
        unreachable!()
    }
}

impl<S, A, B> Add<B> for PhantomOrder<S, PhantomLeaf< A>>
where
    A: FieldOffset<Offset: ToUInt>,
    B: FieldOffset<Offset: ToUInt>,
    <<A as FieldOffset>::Offset as ToUInt>::Output:
        IsLess<<<B as FieldOffset>::Offset as ToUInt>::Output>,
    PhantomEdge<S, (A, B)>: Order<
        <<<A as FieldOffset>::Offset as ToUInt>::Output as IsLess<
            <<B as FieldOffset>::Offset as ToUInt>::Output,
        >>::Output,
    >,
{
    type Output = <PhantomEdge<S, (A, B)> as Order<
        <<A::Offset as ToUInt>::Output as IsLess<<B::Offset as ToUInt>::Output>>::Output,
    >>::Output;

    fn add(self, _rhs: B) -> Self::Output {
        unreachable!()
    }
}

impl<S, A, B> Add<B> for PhantomOrder<S, Field<A>>
where
    B: FieldOffset<Offset: ToUInt>,
    Field<A>: FieldOffset<Offset: ToUInt>,
    <<Field<A> as FieldOffset>::Offset as ToUInt>::Output:
        IsLess<<<B as FieldOffset>::Offset as ToUInt>::Output>,
    PhantomEdge<S, (Field<A>, B)>: Order<
        <<<Field<A> as FieldOffset>::Offset as ToUInt>::Output as IsLess<
            <<B as FieldOffset>::Offset as ToUInt>::Output,
        >>::Output,
    >,
{
    type Output = <PhantomEdge<S, (Field<A>, B)> as Order<
        <<<Field<A> as FieldOffset>::Offset as ToUInt>::Output as IsLess<
            <B::Offset as ToUInt>::Output,
        >>::Output,
    >>::Output;

    fn add(self, _rhs: B) -> Self::Output {
        unreachable!()
    }
}

impl<S, A, B, C> Add<C> for PhantomOrder<S, PhantomEdge<S, (A, B)>>
where
    A: FieldOffset<Offset: ToUInt>,
    C: FieldOffset<Offset: ToUInt>,
    <<A as FieldOffset>::Offset as ToUInt>::Output:
        IsLess<<<C as FieldOffset>::Offset as ToUInt>::Output>,
    PhantomEdge<S, (A, C)>: Order<
            <<<A as FieldOffset>::Offset as ToUInt>::Output as IsLess<
                <<C as FieldOffset>::Offset as ToUInt>::Output,
            >>::Output,
            Output: Edge,
        >,
    PhantomOrder<S, B>: Add<
        <<PhantomEdge<S, (A, C)> as Order<
            <<<A as FieldOffset>::Offset as ToUInt>::Output as IsLess<
                <<C as FieldOffset>::Offset as ToUInt>::Output,
            >>::Output,
        >>::Output as Edge>::Second,
    >,
{
    type Output =
        PhantomEdge<S, (
            <<PhantomEdge<S, (A, C)> as Order<<<A::Offset as ToUInt>::Output as IsLess<<C::Offset as ToUInt>::Output>>::Output>>::Output as Edge>::First, 
            <PhantomOrder<S,  B> 
            as Add<<<PhantomEdge<S, (A, C)> as Order<<<A::Offset as ToUInt>::Output as IsLess<<C::Offset as ToUInt>::Output>>::Output>>::Output as Edge>::Second>>::Output
        )>;

    fn add(self, _rhs: C) -> Self::Output {
        unreachable!()
    }
}
