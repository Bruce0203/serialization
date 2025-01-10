use std::{marker::PhantomData, ops::Add};

use typenum::{B0, B1, IsLess};

use super::{
    edge::{Edge, PhantomEdge},
    end::End,
    field::{FieldOffset, PhantomField},
    leaf::PhantomLeaf,
};

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

impl<S, A, B> Add<B> for PhantomOrder<S, PhantomLeaf<S, A>>
where
    A: FieldOffset,
    B: FieldOffset,
    <A as FieldOffset>::Offset: IsLess<<B as FieldOffset>::Offset>,
    PhantomEdge<S, (A, B)>:
        Order<<<A as FieldOffset>::Offset as IsLess<<B as FieldOffset>::Offset>>::Output>,
{
    type Output =
        <PhantomEdge<S, (A, B)> as Order<<A::Offset as IsLess<B::Offset>>::Output>>::Output;

    fn add(self, _rhs: B) -> Self::Output {
        unreachable!()
    }
}

impl<S, A, B, const I: usize> Add<B> for PhantomOrder<S, PhantomField<S, A, I>>
where
    B: FieldOffset,
    PhantomField<S, A, I>: FieldOffset,
    <PhantomField<S, A, I> as FieldOffset>::Offset: IsLess<<B as FieldOffset>::Offset>,
    PhantomEdge<S, (PhantomField<S, A, I>, B)>:
        Order<
            <<PhantomField<S, A, I> as FieldOffset>::Offset as IsLess<
                <B as FieldOffset>::Offset,
            >>::Output,
        >,
{
    type Output = <PhantomEdge<S, (PhantomField<S, A, I>, B)> as Order<
        <<PhantomField<S, A, I> as FieldOffset>::Offset as IsLess<B::Offset>>::Output,
    >>::Output;

    fn add(self, _rhs: B) -> Self::Output {
        unreachable!()
    }
}

impl<S, A, B, C> Add<C> for PhantomOrder<S, PhantomEdge<S, (A, B)>>
where
    A: FieldOffset,
    C: FieldOffset,
    <A as FieldOffset>::Offset: IsLess<<C as FieldOffset>::Offset>,
    PhantomEdge<S, (A, C)>: Order<
            <<A as FieldOffset>::Offset as IsLess<<C as FieldOffset>::Offset>>::Output,
            Output: Edge,
        >,
    PhantomOrder<S, B>: Add<
        <<PhantomEdge<S, (A, C)> as Order<
            <<A as FieldOffset>::Offset as IsLess<<C as FieldOffset>::Offset>>::Output,
        >>::Output as Edge>::Second,
    >,
{
    type Output =
        PhantomEdge<S, (<<PhantomEdge<S, (A, C)> as Order<<A::Offset as IsLess<C::Offset>>::Output>>::Output as Edge>::First, <PhantomOrder<S,  B> as Add<<<PhantomEdge<S, (A, C)> as Order<<A::Offset as IsLess<C::Offset>>::Output>>::Output as Edge>::Second>>::Output)>;

    fn add(self, _rhs: C) -> Self::Output {
        unreachable!()
    }
}
