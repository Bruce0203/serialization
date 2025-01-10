use std::marker::PhantomData;

use super::{
    actor::Actor,
    compound::{Compound, CompoundWrapper},
    edge::{Edge, PhantomEdge},
    len::Len,
    size::Size,
};

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

impl<S, T, const I: usize> Len for PhantomField<S, T, I>
where
    T: Len,
{
    const SIZE: usize = T::SIZE;
}

impl<S, T, const I: usize> Actor for PhantomField<S, T, I>
where
    T: Actor,
{
    fn run_at(index: usize) -> super::actor::Continuous {
        T::run_at(index)
    }

    fn run() {
        T::run()
    }
}

impl<S, T, const I: usize> Size for PhantomField<S, T, I>
where
    T: Size,
{
    type Size = T::Size;
}
