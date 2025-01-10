use std::marker::PhantomData;

use super::{
    compound::CompoundWrapper, edge::Edge, end::End, field::FieldOffset, leaf::PhantomLeaf,
    len::Len,
};

pub struct Padding<S, FrontOffset>(PhantomData<(S, FrontOffset)>);

impl<S, FrontOffset> FieldOffset for Padding<S, FrontOffset> {
    type Offset = FrontOffset;
}

impl<S, FrontOffset> Edge for Padding<S, FrontOffset> {
    type First = End<S>;

    type Second = End<S>;
}

impl<S, FrontOffset> CompoundWrapper<S> for Padding<S, FrontOffset> {
    type Compound = PhantomLeaf<S, Self>;
}

impl<S, FrontOffset> Len for Padding<S, FrontOffset> {
    const SIZE: usize = 0;
}
