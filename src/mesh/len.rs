use typenum::{ToUInt, Unsigned};

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
    B: FieldOffset<Offset: ToUInt<Output: Unsigned>> + Len,
    FrontOffset: FieldOffset<Offset: ToUInt<Output: Unsigned>> + Len,
{
    const SIZE: usize = {
        let a = <<<FrontOffset as FieldOffset>::Offset as ToUInt>::Output as Unsigned>::USIZE;
        let a_size = <FrontOffset as Len>::SIZE;
        let b = <<B::Offset as ToUInt>::Output as Unsigned>::USIZE;
        if a_size != UNSIZED && a_size + a == b {
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
    Self: Edge<First: Len, Second: Len>,
{
    const SIZE: usize = field_size_of(
        <<Self as Edge>::First as Len>::SIZE,
        <<Self as Edge>::Second as Len>::SIZE,
    );
}
