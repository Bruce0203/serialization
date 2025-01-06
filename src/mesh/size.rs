use super::Edge;

pub trait Size: Edge {
    const SIZE: usize;
}

impl<T> Size for T
where
    T: Edge,
{
    default const SIZE: usize = {
        let a = T::First::SIZE;
        let b = T::Second::SIZE;
        if b == usize::MAX {
            0
        } else if a == usize::MAX {
            b
        } else {
            a + b
        }
    };
}

impl<T> Size for T
where
    T: Edge<First = (), Second = Self>,
{
    default const SIZE: usize = UNSIZED;
}

const UNSIZED: usize = usize::MAX;
