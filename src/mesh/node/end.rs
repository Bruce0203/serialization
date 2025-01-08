use crate::FieldOffset;

use super::Edge;

impl Edge for ! {
    type First = Self;

    type Second = Self;
}

impl<S> FieldOffset<S> for ! {
    const OFFSET: usize = 0;
}
