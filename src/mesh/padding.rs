use std::{marker::PhantomData, ops::Add};

use typenum::{ToUInt, Unsigned};

use super::{
    edge::{Edge, PhantomEdge},
    end::End,
    field::{Field, FieldOffset},
    flatten::CompoundWrapper,
    len::{Len, Size},
    prelude::Vectored,
    r#enum::{Enum, Variant},
};

pub struct Padding<C, S, FrontOffset>(PhantomData<(C, S, FrontOffset)>);

pub struct ConstPadding<C, S, const N: usize>(PhantomData<(C, S)>);

impl<C, S, FrontOffset> FieldOffset for Padding<C, S, FrontOffset>
where
    FrontOffset: FieldOffset,
{
    type Offset = FrontOffset::Offset;
}

impl<C, S, const I: usize> FieldOffset for ConstPadding<C, S, I> {
    ///FieldOffset for Padding was just for ordering when meshup macro
    type Offset = typenum::Const<0>;
}

impl<C, S, FrontOffset> Edge<C> for Padding<C, S, FrontOffset> {
    type First = End<C, S>;

    type Second = End<C, S>;
}

impl<C, S, const I: usize> Edge<C> for ConstPadding<C, S, I> {
    type First = End<C, S>;

    type Second = End<C, S>;
}

//Do not remove separation of S, and S2
impl<C, S, S2, FrontOffset> CompoundWrapper<C, S> for Padding<C, S2, FrontOffset> {
    type Compound = Padding<C, S, FrontOffset>;
}

//Do not remove separation of S, and S2
impl<C, S, S2, const I: usize> CompoundWrapper<C, S> for ConstPadding<C, S2, I> {
    type Compound = ConstPadding<C, S, I>;
}

impl<C, S, FrontOffset, Rhs> Add<Rhs> for Padding<C, S, FrontOffset> {
    type Output = PhantomEdge<C, S, (Padding<C, S, FrontOffset>, Rhs)>;

    fn add(self, _rhs: Rhs) -> Self::Output {
        unreachable!()
    }
}

impl<C, S, const I: usize, Rhs> Add<Rhs> for ConstPadding<C, S, I> {
    type Output = PhantomEdge<C, S, (ConstPadding<C, S, I>, Rhs)>;

    fn add(self, _rhs: Rhs) -> Self::Output {
        unreachable!()
    }
}

impl<C, S, FrontOffset> Len for Padding<C, S, FrontOffset> {
    const SIZE: usize = 0;
}

impl<C, S, const I: usize> Len for ConstPadding<C, S, I> {
    const SIZE: usize = 0;
}

/// Replace `Padding` to `ConstPadding`
pub trait ConstifyPadding {
    type Output;
}

//TODO try remove S3 (becareful)
impl<Codec, S, S2, S3, FrontOffset, B, C> ConstifyPadding
    for PhantomEdge<
        Codec,
        S,
        (
            Padding<Codec, S2, FrontOffset>,
            PhantomEdge<Codec, S3, (Field<B>, C)>,
        ),
    >
where
    Self: Len,
    FrontOffset: FieldOffset<Offset: ToUInt<Output: Unsigned>> + Size,
    Field<B>: FieldOffset<Offset: ToUInt<Output: Unsigned>>,
    [(); padding_of::<FrontOffset, Field<B>>()]:,
    PhantomEdge<Codec, S, (Field<B>, C)>: ConstifyPadding,
{
    type Output = PhantomEdge<
        Codec,
        S,
        (
            ConstPadding<Codec, S, { padding_of::<FrontOffset, Field<B>>() }>,
            //TODO watch out! there is S3
            <PhantomEdge<Codec, S, (Field<B>, C)> as ConstifyPadding>::Output,
        ),
    >;
}

//TODO make macro for Field, Vectored, Enum, Variant
impl<C, S, A, B> ConstifyPadding for PhantomEdge<C, S, (Field<A>, B)>
where
    B: ConstifyPadding,
{
    type Output = PhantomEdge<C, S, (Field<A>, <B as ConstifyPadding>::Output)>;
}

impl<C, S, A, B> ConstifyPadding for PhantomEdge<C, S, (Vectored<A>, B)>
where
    B: ConstifyPadding,
{
    type Output = PhantomEdge<C, S, (Vectored<A>, <B as ConstifyPadding>::Output)>;
}

impl<C, S, A, B, V> ConstifyPadding for PhantomEdge<C, S, (Enum<A, V>, B)>
where
    B: ConstifyPadding,
{
    type Output = PhantomEdge<C, S, (Enum<A, V>, <B as ConstifyPadding>::Output)>;
}

impl<C, S, A, B, const I: usize> ConstifyPadding for PhantomEdge<C, S, (Variant<A, I>, B)>
where
    B: ConstifyPadding,
{
    type Output = PhantomEdge<C, S, (Variant<A, I>, <B as ConstifyPadding>::Output)>;
}

impl<C, S> ConstifyPadding for End<C, S> {
    type Output = End<C, S>;
}

impl<C, S, S2, S3, FrontOffset> ConstifyPadding
    for PhantomEdge<C, S, (Padding<C, S2, FrontOffset>, End<C, S3>)>
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
        C,
        S,
        (
            ConstPadding<
                C,
                S,
                {
                    <S3 as Size>::SIZE - (
                        <<<FrontOffset as FieldOffset>::Offset as ToUInt>::Output as Unsigned>::USIZE 
                        + <FrontOffset as Size>::SIZE
                    )
                },
            >,
            End<C, S3>,
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
