use super::{Edge, PhantomEdge};

const UNSIZED: usize = usize::MAX;

pub trait Size: Edge {
    const SIZE: usize;
}

impl<S, A, B> Size for PhantomEdge<S, (A, B)>
where
    Self: Edge<First: Size, Second: Size>,
{
    const SIZE: usize = {
        let a = Self::First::SIZE;
        let b = Self::Second::SIZE;
        if b == usize::MAX {
            0
        } else if a == usize::MAX {
            b
        } else {
            a + b
        }
    };
}
