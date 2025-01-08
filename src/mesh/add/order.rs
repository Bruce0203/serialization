use std::{marker::PhantomData, ops::Add};

use typenum::{B0, B1, IsLess};

use crate::{Edge, FieldOffset, PhantomEdge, PhantomField, PhantomLeaf};

pub trait Order<T> {
    type Output;
}

impl<S, A, B> Order<B1> for PhantomEdge<S, (A, B)> {
    type Output = PhantomEdge<S, (A, B)>;
}

impl<S, A, B> Order<B0> for PhantomEdge<S, (A, B)> {
    type Output = PhantomEdge<S, (B, A)>;
}

pub struct Ordering<S, T>(PhantomData<(S, T)>);

pub trait OrderingWrapper<S> {
    type Ordering;
}

impl<S, B> Add<B> for Ordering<S, !> {
    type Output = PhantomLeaf<S, B>;

    fn add(self, _rhs: B) -> Self::Output {
        unreachable!()
    }
}

impl<S, T> Edge for Ordering<S, T> {}

impl<S, T> FieldOffset<S> for Ordering<S, T>
where
    T: FieldOffset<S>,
{
    type Offset = T::Offset;
}

impl<S, A, B> Add<B> for Ordering<S, PhantomLeaf<S, A>>
where
    A: FieldOffset<S>,
    B: FieldOffset<S>,
    <A as FieldOffset<S>>::Offset: IsLess<<B as FieldOffset<S>>::Offset>,
    PhantomEdge<S, (A, B)>:
        Order<<<A as FieldOffset<S>>::Offset as IsLess<<B as FieldOffset<S>>::Offset>>::Output>,
{
    type Output =
        <PhantomEdge<S, (A, B)> as Order<<A::Offset as IsLess<B::Offset>>::Output>>::Output;

    fn add(self, _rhs: B) -> Self::Output {
        todo!()
    }
}

impl<S, A, B, C> Add<C> for Ordering<S, PhantomEdge<S, (A, B)>>
where
    A: FieldOffset<S>,
    B: FieldOffset<S> + Edge<Second: FieldOffset<S>>,
    C: FieldOffset<S>,
    <A as FieldOffset<S>>::Offset: IsLess<<C as FieldOffset<S>>::Offset>,
    PhantomEdge<S, (A, C)>:
        Order<<<A as FieldOffset<S>>::Offset as IsLess<<C as FieldOffset<S>>::Offset>>::Output>,
    <PhantomEdge<S, (A, C)> as Order<
        <<A as FieldOffset<S>>::Offset as IsLess<<C as FieldOffset<S>>::Offset>>::Output,
    >>::Output: Edge,
    <<PhantomEdge<S, (A, C)> as Order<
        <<A as FieldOffset<S>>::Offset as IsLess<<C as FieldOffset<S>>::Offset>>::Output,
    >>::Output as Edge>::Second: FieldOffset<S>,
    PhantomEdge<
        S,
        (
            <<PhantomEdge<S, (A, C)> as Order<
                <<A as FieldOffset<S>>::Offset as IsLess<<C as FieldOffset<S>>::Offset>>::Output,
            >>::Output as Edge>::Second,
            <B as Edge>::First,
        ),
    >: Order<
        <<<<PhantomEdge<S, (A, C)> as Order<
            <<A as FieldOffset<S>>::Offset as IsLess<<C as FieldOffset<S>>::Offset>>::Output,
        >>::Output as Edge>::Second as FieldOffset<S>>::Offset as IsLess<
            <<B as Edge>::Second as FieldOffset<S>>::Offset,
        >>::Output,
    >,
    <<<PhantomEdge<S, (A, C)> as Order<
        <<A as FieldOffset<S>>::Offset as IsLess<<C as FieldOffset<S>>::Offset>>::Output,
    >>::Output as Edge>::Second as FieldOffset<S>>::Offset:
        IsLess<<<B as Edge>::Second as FieldOffset<S>>::Offset>,
{
    type Output =
        PhantomEdge<
            S,
            (
                <<PhantomEdge<S, (A, C)> as Order<
                    <A::Offset as IsLess<C::Offset>>::Output,
                >>::Output as Edge>::First,
                <PhantomEdge<
                    S,
                    //TODO currently B is PhantomEdge but calc offset
                    (
                        <<PhantomEdge<S, (A, C)> as Order<
                            <A::Offset as IsLess<C::Offset>>::Output,
                        >>::Output as Edge>::Second,
                        B::First,
                    ),
                > as Order<
                    <<<<PhantomEdge<S, (A, C)> as Order<
                        <A::Offset as IsLess<C::Offset>>::Output,
                    >>::Output as Edge>::Second as FieldOffset<S>>::Offset as IsLess<
                        <B::Second as FieldOffset<S>>::Offset,
                    >>::Output,
                >>::Output,
            ),
        >;

    fn add(self, _rhs: C) -> Self::Output {
        unreachable!()
    }
}

impl<T, S> OrderingWrapper<S> for T {
    type Ordering = PhantomLeaf<S, Self>;
}
//A, B, C
//
//A > C
//C, A, B
//B < C
//
//(B, (A, C Not Orderd)::Second)
//
//
//1  2  3
//A  B  C
//
//
//1  3  2
//A  C  B
//
//2  3  1
//C  A  B
//
//
