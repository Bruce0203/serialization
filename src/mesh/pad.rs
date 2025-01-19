use std::{marker::PhantomData, ops::Add};

use typenum::{ToUInt, Unsigned};

use super::{
    edge::{Edge, PhantomEdge},
    end::End,
    field::{Field, FieldOffset},
    flatten::CompoundWrapper,
    len::{Len, Size},
};

pub struct Padding<S, FrontOffset>(PhantomData<(S, FrontOffset)>);

pub struct ConstPadding<S, const N: usize>(PhantomData<S>);

impl<S, FrontOffset> FieldOffset for Padding<S, FrontOffset>
where
    FrontOffset: FieldOffset,
{
    type Offset = FrontOffset::Offset;
}

impl<S, const I: usize> FieldOffset for ConstPadding<S, I> {
    ///FieldOffset for Padding was just for ordering when meshup macro
    type Offset = typenum::Const<0>;
}

impl<S, FrontOffset> Edge for Padding<S, FrontOffset> {
    type First = End<S>;

    type Second = End<S>;
}

impl<S, const I: usize> Edge for ConstPadding<S, I> {
    type First = End<S>;

    type Second = End<S>;
}

//Do not remove separation of S, and S2
impl<S, S2, FrontOffset> CompoundWrapper<S> for Padding<S2, FrontOffset> {
    type Compound = Padding<S, FrontOffset>;
}

//Do not remove separation of S, and S2
impl<S, S2, const I: usize> CompoundWrapper<S> for ConstPadding<S2, I> {
    type Compound = ConstPadding<S, I>;
}

impl<S, FrontOffset, Rhs> Add<Rhs> for Padding<S, FrontOffset> {
    type Output = PhantomEdge<S, (Padding<S, FrontOffset>, Rhs)>;

    fn add(self, _rhs: Rhs) -> Self::Output {
        unreachable!()
    }
}

impl<S, const I: usize, Rhs> Add<Rhs> for ConstPadding<S, I> {
    type Output = PhantomEdge<S, (ConstPadding<S, I>, Rhs)>;

    fn add(self, _rhs: Rhs) -> Self::Output {
        unreachable!()
    }
}

impl<S, FrontOffset> Len for Padding<S, FrontOffset> {
    const SIZE: usize = 0;
}

impl<S, const I: usize> Len for ConstPadding<S, I> {
    const SIZE: usize = 0;
}

/// Replace `Padding` to `ConstPadding`
pub trait ConstifyPadding {
    type Output;
}

//TODO try remove S3 (becareful)
impl<S, S2, S3, FrontOffset, B, C> ConstifyPadding
    for PhantomEdge<S, (Padding<S2, FrontOffset>, PhantomEdge<S3, (Field<B>, C)>)>
where
    Self: Len,
    FrontOffset: FieldOffset<Offset: ToUInt<Output: Unsigned>> + Size,
    Field<B>: FieldOffset<Offset: ToUInt<Output: Unsigned>>,
    [(); padding_of::<FrontOffset, Field<B>>()]:,
    PhantomEdge<S, (Field<B>, C)>: ConstifyPadding,
{
    type Output = PhantomEdge<
        S,
        (
            ConstPadding<S, { padding_of::<FrontOffset, Field<B>>() }>,
            //TODO watch out! there is S3
            <PhantomEdge<S, (Field<B>, C)> as ConstifyPadding>::Output,
        ),
    >;
}

impl<S, A, B> ConstifyPadding for PhantomEdge<S, (Field<A>, B)>
where
    B: ConstifyPadding,
{
    type Output = PhantomEdge<S, (Field<A>, <B as ConstifyPadding>::Output)>;
}

impl<S> ConstifyPadding for End<S> {
    type Output = End<S>;
}

impl<S, S2, S3, FrontOffset> ConstifyPadding for PhantomEdge<S, (Padding<S2, FrontOffset>, End<S3>)>
where
    S3: Size,
    FrontOffset: FieldOffset<Offset: ToUInt<Output: Unsigned>> + Size,
    [(); {
        <S3 as Size>::SIZE
            - (<<<FrontOffset as FieldOffset>::Offset as ToUInt>::Output as Unsigned>::USIZE
                + <FrontOffset as Size>::SIZE)
    }]:,
{
    type Output = PhantomEdge<
        S,
        (
            ConstPadding<
                S,
                {
                    <S3 as Size>::SIZE - (
                        <<<FrontOffset as FieldOffset>::Offset as ToUInt>::Output as Unsigned>::USIZE 
                        + <FrontOffset as Size>::SIZE
                    )
                },
            >,
            End<S3>,
        ),
    >;
}

pub const fn padding_of<FrontOffset, B>() -> usize
where
    FrontOffset: FieldOffset<Offset: ToUInt<Output: Unsigned>> + Size,
    B: FieldOffset<Offset: ToUInt<Output: Unsigned>>,
{
    let front = <<<FrontOffset as FieldOffset>::Offset as ToUInt>::Output as Unsigned>::USIZE;
    let front_size = <FrontOffset as Size>::SIZE;
    let back = <<B::Offset as ToUInt>::Output as Unsigned>::USIZE;
    back - (front_size + front)
}
