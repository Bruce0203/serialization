use typenum::Unsigned;

use super::{
    edge::{Edge, PhantomEdge},
    end::End,
    field::{FieldOffset, PhantomField},
    padding::Padding,
};

pub const UNSIZED: usize = usize::MAX;

pub trait Len {
    const SIZE: usize;
}

const fn field_size_of<T>() -> usize
where
    T: Edge<First: Len, Second: Len>,
{
    let a = <T as Edge>::First::SIZE;
    let b = <T as Edge>::Second::SIZE;
    if a == UNSIZED || b == UNSIZED {
        0
    } else {
        a + b
    }
}

impl<S, S2, S3, FrontOffset, B, C> Len
    for PhantomEdge<S, (Padding<S2, FrontOffset>, PhantomEdge<S3, (B, C)>)>
where
    Self: Edge<First: Len, Second: Len>,
{
    const SIZE: usize = field_size_of::<Self>();
}

impl<S, S2, FrontOffset> Len for PhantomEdge<S, (Padding<S, FrontOffset>, End<S2>)>
where
    Self: Edge<First: Len, Second: Len>,
{
    const SIZE: usize = field_size_of::<Self>();
}

impl<S, S2, A, B, const I: usize> Len for PhantomEdge<S, (PhantomField<S2, A, I>, B)>
where
    Self: Edge<First: Len, Second: Len>,
{
    const SIZE: usize = field_size_of::<Self>();
}
