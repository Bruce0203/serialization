use std::marker::PhantomData;

use super::edge::Edge;

///Token for end
pub struct End<S>(PhantomData<S>);

impl<S> Edge for End<S> {
    type First = End<S>;

    type Second = End<S>;
}
