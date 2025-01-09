use super::{edge::Edge, size::Size};

///Token for end
pub struct End;

impl Edge for End {
    type First = End;

    type Second = End;
}

impl Size for End {
    const SIZE: usize = 0;
}
