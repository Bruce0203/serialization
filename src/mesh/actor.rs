use std::{any::type_name, mem::transmute};

use crate::{CompositeEncoder, Encode};

use super::{
    edge::PhantomEdge,
    end::End,
    field::Field,
    len::{Len, Size},
    pad::ConstPadding,
};

pub trait EncodeActor<S, C>
where
    C: CompositeEncoder,
{
    fn run(src: &S, codec: &mut C, skip_acc: usize) -> Result<(), C::Error>;
}

impl<S, C, A, B> EncodeActor<S, C> for PhantomEdge<S, (Field<A>, B)>
where
    Self: Len,
    A: Encode + Size,
    B: EncodeActor<S, C>,
    C: CompositeEncoder,
    [(); <Self as Len>::SIZE]:,
{
    fn run(mut src: &S, codec: &mut C, mut skip_acc: usize) -> Result<(), C::Error> {
        println!("field {:?} {}", <Self as Len>::SIZE, type_name::<A>());
        if skip_acc == 0 {
            skip_acc = <Self as Len>::SIZE;
            if <Self as Len>::SIZE == 0 {
                unsafe { codec.encode_element::<A>(transmute(src))? };
                src = unsafe { &*(src as *const S).byte_add(<A as Size>::SIZE) };
            } else {
                unsafe { codec.encode_slice::<{ <Self as Len>::SIZE }>(transmute(src)) };
                src = unsafe { &*(src as *const S).byte_add(<Self as Len>::SIZE) };
            }
        } else {
            skip_acc -= <A as Size>::SIZE;
        }
        B::run(src, codec, skip_acc)
    }
}

impl<S, S2, C, B, const I: usize> EncodeActor<S, C> for PhantomEdge<S, (ConstPadding<S2, I>, B)>
where
    C: CompositeEncoder,
    Self: Len,
    B: EncodeActor<S, C>,
{
    fn run(mut src: &S, codec: &mut C, mut skip_acc: usize) -> Result<(), C::Error> {
        if skip_acc != 0 && I != 0 {
            skip_acc = 0;
        }
        src = unsafe { &*(src as *const S).byte_add(I) };
        B::run(src, codec, skip_acc)
    }
}

impl<S, C> EncodeActor<S, C> for End<S>
where
    C: CompositeEncoder,
{
    fn run(_src: &S, _codec: &mut C, _skip_acc: usize) -> Result<(), C::Error> {
        Ok(())
    }
}
