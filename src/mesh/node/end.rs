use typenum::{U, U0};

use crate::FieldOffset;

use super::Edge;

impl Edge for ! {
    type First = Self;

    type Second = Self;
}

impl<S> FieldOffset<S> for () {
    type Offset = U<{ i64::MAX as usize + 1 }>;
}
