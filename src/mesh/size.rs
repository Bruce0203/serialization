use super::edge::{Edge, PhantomEdge};

pub const UNSIZED: usize = usize::MAX;

pub trait Size {
    const SIZE: usize;
}

impl<S, A, B> Size for PhantomEdge<S, (A, B)>
where
    Self: Edge<First: Size, Second: Size>,
{
    const SIZE: usize = {
        let a = <Self as Edge>::First::SIZE;
        let b = <Self as Edge>::Second::SIZE;
        if a == UNSIZED || b == UNSIZED {
            0
        } else {
            a + b
        }
    };
}
