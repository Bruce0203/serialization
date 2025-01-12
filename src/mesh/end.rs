use std::{marker::PhantomData, ops::Add};

use super::edge::{Edge, PhantomEdge};

///Token for end
pub struct End<S>(PhantomData<S>);

impl<S> Edge for End<S> {
    type First = End<S>;

    type Second = End<S>;
}

impl<S, Rhs> Add<Rhs> for End<S> {
    type Output = PhantomEdge<S, (Rhs, End<S>)>;

    fn add(self, _rhs: Rhs) -> Self::Output {
        unreachable!()
    }
}
