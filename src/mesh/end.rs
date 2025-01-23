use std::{marker::PhantomData, ops::Add};

use super::{
    edge::{Edge, PhantomEdge},
    len::Len,
};

///Token for end
pub struct End<C, S>(PhantomData<(C, S)>);

impl<C, S> Edge<C> for End<C, S> {
    type First = End<C, S>;

    type Second = End<C, S>;
}

impl<C, S, Rhs> Add<Rhs> for End<C, S> {
    type Output = PhantomEdge<C, S, (Rhs, End<C, S>)>;

    fn add(self, _rhs: Rhs) -> Self::Output {
        unreachable!()
    }
}

impl<C, S> Len for End<C, S> {
    const SIZE: usize = 0;
}
