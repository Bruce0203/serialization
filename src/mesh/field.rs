use std::marker::PhantomData;

use super::edge::Edge;

pub trait FieldOffset {
    type Offset;
}

pub struct PhantomField<S, T, const I: usize>(PhantomData<(S, T)>);

impl<S, T, const I: usize> Edge for PhantomField<S, T, I>
where
    T: Edge,
{
    type First = T::First;

    type Second = T::Second;
}
