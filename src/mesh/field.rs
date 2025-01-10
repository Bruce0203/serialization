use std::marker::PhantomData;

use super::{actor::Actor, edge::Edge};

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
