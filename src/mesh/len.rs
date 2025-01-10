use typenum::Unsigned;

use super::{
    edge::{Edge, PhantomEdge},
    end::End,
    field::{FieldOffset, PhantomField},
    padding::Padding,
    size::Size,
};

pub const UNSIZED: usize = usize::MAX;

pub trait Len {
    const SIZE: usize;
}

const fn field_size_of(a: usize, b: usize) -> usize {
    if a == UNSIZED || b == UNSIZED {
        0
    } else {
        a + b
    }
}

impl<S, S2, S3, FrontOffset, B, C> Len
    for PhantomEdge<S, (Padding<S2, FrontOffset>, PhantomEdge<S3, (B, C)>)>
where
    C: Len,
    B: FieldOffset<Offset: Unsigned> + Size<Size: Unsigned> + Len,
    FrontOffset: FieldOffset<Offset: Unsigned> + Size<Size: Unsigned> + Size<Size: Unsigned>,
{
    const SIZE: usize = {
        let is_unsized_front_anyway = <<FrontOffset as Size>::Size as Unsigned>::USIZE == UNSIZED;
        let a = <<FrontOffset as FieldOffset>::Offset as Unsigned>::USIZE;
        let a_size = <<FrontOffset as Size>::Size as Unsigned>::USIZE;
        let b = <B::Offset as Unsigned>::USIZE;
        if a_size + a == b {
            field_size_of(<B as Len>::SIZE, <C as Len>::SIZE)
        } else {
            0
        }
    };
}

impl<S, S2, FrontOffset> Len for PhantomEdge<S, (Padding<S, FrontOffset>, End<S2>)> {
    const SIZE: usize = 0;
}

impl<S, S2, A, B, const I: usize> Len for PhantomEdge<S, (PhantomField<S2, A, I>, B)>
where
    Self: Edge<First: Size<Size: Unsigned>, Second: Len>,
{
    const SIZE: usize = field_size_of(
        <<Self as Edge>::First as Size>::Size::USIZE,
        <<Self as Edge>::Second as Len>::SIZE,
    );
}
