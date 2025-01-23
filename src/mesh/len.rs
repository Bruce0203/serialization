use typenum::{ToUInt, Unsigned};

use super::{
    edge::{Edge, PhantomEdge},
    end::End,
    field::{Field, FieldOffset},
    pad::{ConstPadding, Padding},
    prelude::Vectored,
};

pub const UNSIZED: usize = usize::MAX;

pub trait Size {
    const SIZE: usize;
}

pub trait Len {
    const SIZE: usize;
}

impl<C, S, S2, const I: usize, B> Len for PhantomEdge<C, S, (ConstPadding<C, S2, I>, B)>
where
    B: Len,
{
    const SIZE: usize = {
        if I == 0 {
            B::SIZE
        } else {
            0
        }
    };
}

impl<Codec, S, S2, S3, FrontOffset, B, C> Len
    for PhantomEdge<Codec, S, (Padding<Codec, S2, FrontOffset>, PhantomEdge<Codec, S3, (B, C)>)>
where
    C: Len,
    B: FieldOffset<Offset: ToUInt<Output: Unsigned>> + Len,
    FrontOffset: FieldOffset<Offset: ToUInt<Output: Unsigned>> + Len,
    PhantomEdge<Codec, S3, (B, C)>: Len,
{
    const SIZE: usize = {
        let front = <<<FrontOffset as FieldOffset>::Offset as ToUInt>::Output as Unsigned>::USIZE;
        let front_size = <FrontOffset as Len>::SIZE;
        let back = <<B::Offset as ToUInt>::Output as Unsigned>::USIZE;
        if front_size != UNSIZED && front_size + front == back {
            // field_size_of(<B as Len>::SIZE, <C as Len>::SIZE);
            <PhantomEdge<Codec, S3, (B, C)> as Len>::SIZE
        } else {
            0
        }
    };
}

//TODO try remove S3 and replace it to S
impl<Codec, S, S2, S3, FrontOffset> Len
    for PhantomEdge<Codec, S, (Padding<Codec, S3, FrontOffset>, End<Codec, S2>)>
where
    S2: Len + Size,
    FrontOffset: FieldOffset<Offset: ToUInt<Output: Unsigned>> + Len + Size,
{
    const SIZE: usize = <S2 as Size>::SIZE
        - (<<<FrontOffset as FieldOffset>::Offset as ToUInt>::Output as Unsigned>::USIZE
            + <FrontOffset as Size>::SIZE);
}

impl<C, S, A, B> Len for PhantomEdge<C, S, (Field<A>, B)>
where
    Self: Edge<C, First: Len, Second: Len>,
{
    const SIZE: usize = field_size_of(
        <<Self as Edge<C>>::First as Len>::SIZE,
        <<Self as Edge<C>>::Second as Len>::SIZE,
    );
}

impl<C, S, T, B> Len for PhantomEdge<C, S, (Vectored<T>, B)>
where
    Self: Edge<C, Second: Len>,
{
    const SIZE: usize = field_size_of(UNSIZED, <<Self as Edge<C>>::Second as Len>::SIZE);
}

const fn field_size_of(a: usize, b: usize) -> usize {
    if a == UNSIZED {
        0
    } else {
        a + b
    }
}
