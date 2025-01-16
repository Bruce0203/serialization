use super::{actor::Actor, edge::Edge, flatten::Flatten, pad::ConstifyPadding, sort::Sorted};

pub trait Mesh<C> {
    type Output;
}

impl<T, C> Mesh<C> for T
where
    T: Edge<Second: Sorted<Output: ConstifyPadding<Output: Flatten<T, Output: Actor<T, C>>>>>,
{
    type Output = <<<<T as Edge>::Second as Sorted>::Output as ConstifyPadding>::Output as Flatten<T>>::Output;
}
