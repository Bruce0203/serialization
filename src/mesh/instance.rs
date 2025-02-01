use std::marker::PhantomData;

use super::{edge::PhantomEdge, end::End};

//TODO try remove and try compile
pub trait Instantiate {
    fn instance() -> Self;
}

impl<C, S, T> Instantiate for PhantomEdge<C, S, T> {
    fn instance() -> Self {
        Self(PhantomData)
    }
}

impl<C, S> Instantiate for End<C, S> {
    fn instance() -> Self {
        Self(PhantomData)
    }
}
