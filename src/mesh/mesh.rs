use crate::CompositeEncoder;

use super::{
    edge::Edge,
    flatten::Flatten,
    pad::ConstifyPadding,
    segment::{SegmentHandler, SegmentWalker},
    sort::Sorted,
};

pub trait Mesh<C> {
    type Output;
}

impl<T, C> Mesh<C> for T
where
    C: CompositeEncoder,
    T: Edge<Second: Sorted<Output: ConstifyPadding<Output: Flatten<T>>>>,
{
    type Output = <<<<T as Edge>::Second as Sorted>::Output as ConstifyPadding>::Output as Flatten<T>>::Output;
}

//TODO try remove inline never
#[inline(never)]
pub fn walk_segment<T, C, H>(mut src: &T, codec: &mut C) -> Result<(), H::Error>
where
    H: SegmentHandler<C>,
    C: CompositeEncoder,
    T: Mesh<C, Output: SegmentWalker<T, C, H>>,
{
    let src = &mut src;
    const SKIP_ACC: usize = 0;
    <T as Mesh<C>>::Output::walk(src, codec, SKIP_ACC)
}
