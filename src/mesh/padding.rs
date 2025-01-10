use std::marker::PhantomData;

use super::{
    compound::CompoundWrapper, edge::Edge, end::End, field::FieldOffset, leaf::PhantomLeaf,
    len::Len, size::Size,
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

impl<S, FrontOffset> CompoundWrapper<S> for Padding<S, FrontOffset> {
    type Compound = PhantomLeaf<S, Self>;
}

///TODO remove (try it)
impl<S, FrontOffset> Size for Padding<S, FrontOffset> {
    type Size = typenum::U0;
}
