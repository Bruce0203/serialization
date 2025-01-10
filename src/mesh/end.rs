use std::marker::PhantomData;

use super::{edge::Edge, field::FieldOffset, len::Len, size::Size};

///Token for end
pub struct End<S>(PhantomData<S>);

impl<S> Edge for End<S> {
    type First = End<S>;

    type Second = End<S>;
}

impl<S> Size for End<S>
where
    S: Size,
{
    type Size = typenum::U0;
}

impl<S> FieldOffset for End<S>
where
    S: Size,
{
    type Offset = S::Size;
}

impl<S> Len for End<S> {
    const SIZE: usize = 0;
}
