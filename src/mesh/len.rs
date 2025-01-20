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

impl<S, S2, const I: usize, B> Len for PhantomEdge<S, (ConstPadding<S2, I>, B)>
where
    B: Len,
{
    const SIZE: usize = { if I == 0 { B::SIZE } else { 0 } };
}

impl<S, S2, S3, FrontOffset, B, C> Len
    for PhantomEdge<S, (Padding<S2, FrontOffset>, PhantomEdge<S3, (B, C)>)>
where
    C: Len,
    B: FieldOffset<Offset: ToUInt<Output: Unsigned>> + Len,
    FrontOffset: FieldOffset<Offset: ToUInt<Output: Unsigned>> + Len,
    PhantomEdge<S3, (B, C)>: Len,
{
    const SIZE: usize = {
        let front = <<<FrontOffset as FieldOffset>::Offset as ToUInt>::Output as Unsigned>::USIZE;
        let front_size = <FrontOffset as Len>::SIZE;
        let back = <<B::Offset as ToUInt>::Output as Unsigned>::USIZE;
        //TODO Simplify condition as possible as you can...
        if front_size != UNSIZED && front_size + front == back {
            //TODO add size of front
            field_size_of(<B as Len>::SIZE, <C as Len>::SIZE)
        } else {
            0
        }
    };
}

//TODO try remove S3 and replace it to S
impl<S, S2, S3, FrontOffset> Len for PhantomEdge<S, (Padding<S3, FrontOffset>, End<S2>)>
where
    S2: Len + Size,
    FrontOffset: FieldOffset<Offset: ToUInt<Output: Unsigned>> + Len + Size,
{
    const SIZE: usize = <S2 as Size>::SIZE
        - (<<<FrontOffset as FieldOffset>::Offset as ToUInt>::Output as Unsigned>::USIZE
            + <FrontOffset as Size>::SIZE);
}

impl<S, A, B> Len for PhantomEdge<S, (Field<A>, B)>
where
    Self: Edge<First: Len, Second: Len>,
{
    const SIZE: usize = field_size_of(
        <<Self as Edge>::First as Len>::SIZE,
        <<Self as Edge>::Second as Len>::SIZE,
    );
}

impl<S, T, B, V> Len for PhantomEdge<S, (Vectored<T, V>, B)>
where
    Self: Edge<Second: Len>,
{
    const SIZE: usize = field_size_of(UNSIZED, <<Self as Edge>::Second as Len>::SIZE);
}

const fn field_size_of(a: usize, b: usize) -> usize {
    //TODO try remove b == UNSIZED
    if a == UNSIZED || b == UNSIZED {
        0
    } else {
        a + b
    }
}
