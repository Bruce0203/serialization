use crate::{CompositeEncoder, Encoder};

use super::{
    actor::{Continuous, EncodeActor},
    edge::Edge,
    flatten::Flatten,
    pad::ConstifyPadding,
    sort::Sorted,
};

pub trait Mesh<C> {
    type Output;
}

impl<T, C> Mesh<C> for T
where
    C: CompositeEncoder,
    T: Edge<Second: Sorted<Output: ConstifyPadding<Output: Flatten<T, Output: EncodeActor<T, C>>>>>,
{
    type Output = <<<<T as Edge>::Second as Sorted>::Output as ConstifyPadding>::Output as Flatten<T>>::Output;
}

pub fn encode_with_encoder<T, C>(src: &T, enc: &mut C) -> Result<(), C::Error>
where
    C: CompositeEncoder,
    T: Mesh<C, Output: EncodeActor<T, C>>,
{
    let mut i = 0;
    let mut skip_acc = 0;
    let src = &mut (src as *const T);
    loop {
        match T::Output::run_at(src, enc, &mut skip_acc, i) {
            Continuous::Next => break,
            Continuous::Done(result) => result?,
        }
        i += 1;
    }
    Ok(())
}
