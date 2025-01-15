use std::{marker::PhantomData, ops::Add};

use super::{
    edge::{Edge, PhantomEdge},
    end::End,
    field::FieldOffset,
    flatten::CompoundWrapper,
    len::Len,
};

pub struct Padding<S, FrontOffset>(PhantomData<(S, FrontOffset)>);

impl<S, FrontOffset> FieldOffset for Padding<S, FrontOffset>
where
    FrontOffset: FieldOffset,
{
    type Offset = FrontOffset::Offset;
}

impl<S, FrontOffset> Edge for Padding<S, FrontOffset> {
    type First = End<S>;

    type Second = End<S>;
}

impl<S, S2, FrontOffset> CompoundWrapper<S> for Padding<S2, FrontOffset> {
    type Compound = Padding<S, FrontOffset>;
}

impl<S, FrontOffset, Rhs> Add<Rhs> for Padding<S, FrontOffset> {
    type Output = PhantomEdge<S, (Padding<S, FrontOffset>, Rhs)>;

    fn add(self, _rhs: Rhs) -> Self::Output {
        unreachable!()
    }
}

impl<S, FrontOffset> Len for Padding<S, FrontOffset> {
    const SIZE: usize = 0;
}
