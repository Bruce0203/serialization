use std::{any::type_name, mem::transmute};

use crate::{CompositeEncoder, Encode};

use super::super::{
    edge::PhantomEdge,
    end::End,
    field::Field,
    len::{Len, Size},
    mesh::Mesh,
    pad::ConstPadding,
    prelude::{Vector, Vectored},
};

pub trait EncodeActor<S, C>
where
    C: CompositeEncoder,
{
    fn run(src: &S, codec: &mut C, skip_acc: usize, vectored_amount: usize)
        -> Result<(), C::Error>;
}

impl<S, C, A, B> EncodeActor<S, C> for PhantomEdge<S, (Field<A>, B)>
where
    Self: Len,
    A: Encode + Size,
    B: EncodeActor<S, C>,
    C: CompositeEncoder,
    S: Size,
    [(); <Self as Len>::SIZE]:,
{
    fn run(
        mut src: &S,
        codec: &mut C,
        mut skip_acc: usize,
        vectored_amount: usize,
    ) -> Result<(), C::Error> {
        #[cfg(debug_assertions)]
        println!("field {:?} {}", <Self as Len>::SIZE, type_name::<A>());
        if skip_acc == 0 {
            skip_acc = <Self as Len>::SIZE;
            {
                let mut src = src;
                for _ in 0..vectored_amount {
                    if <Self as Len>::SIZE == 0 {
                        unsafe { codec.encode_element::<A>(transmute(src))? };
                    } else {
                        unsafe { codec.encode_array::<{ <Self as Len>::SIZE }>(transmute(src)) };
                    }
                    src = unsafe { &*(src as *const S).byte_add(<S as Size>::SIZE) };
                }
            }
            if <Self as Len>::SIZE == 0 {
                src = unsafe { &*(src as *const S).byte_add(<A as Size>::SIZE) };
            } else {
                src = unsafe { &*(src as *const S).byte_add(<Self as Len>::SIZE) };
            }
        } else {
            skip_acc -= <A as Size>::SIZE;
        }
        B::run(src, codec, skip_acc, vectored_amount)
    }
}

impl<S, C, B, T, V> EncodeActor<S, C> for PhantomEdge<S, (Vectored<T, V>, B)>
where
    C: CompositeEncoder,
    B: EncodeActor<S, C>,
    T: Vector<Item: Size + Mesh<C, Output: EncodeActor<<T as Vector>::Item, C> + Len>> + Size,
    S: Size,
    [(); <<T as Vector>::Item as Size>::SIZE]:,
{
    fn run(
        mut src: &S,
        codec: &mut C,
        _skip_acc: usize,
        vectored_amount: usize,
    ) -> Result<(), C::Error> {
        let skip_acc = 0;
        src = unsafe { &*(src as *const S).byte_sub(1) };
        let vec = unsafe { transmute::<_, &T>(src) };
        {
            let mut src = vec.as_ptr();
            for _ in 0..vec.len() {
                <<<T as Vector>::Item as Mesh<C>>::Output as EncodeActor<
                    <T as Vector>::Item,
                    C,
                >>::run(unsafe { transmute(src) }, codec, skip_acc, 1)?;
                src = unsafe { src.byte_add(<<T as Vector>::Item as Size>::SIZE) };
            }
        }
        src = unsafe { &*(src as *const S).byte_add(<T as Size>::SIZE) };
        B::run(src, codec, skip_acc, vectored_amount)
    }
}

impl<S, S2, C, B, const I: usize> EncodeActor<S, C> for PhantomEdge<S, (ConstPadding<S2, I>, B)>
where
    C: CompositeEncoder,
    Self: Len,
    B: EncodeActor<S, C>,
{
    fn run(
        mut src: &S,
        codec: &mut C,
        mut skip_acc: usize,
        vectored_amount: usize,
    ) -> Result<(), C::Error> {
        //TODO try remove skip_acc != 0 or not!
        if skip_acc != 0 && I != 0 {
            skip_acc = 0;
        }
        src = unsafe { &*(src as *const S).byte_add(I) };
        B::run(src, codec, skip_acc, vectored_amount)
    }
}

impl<S, S2, C> EncodeActor<S, C> for End<S2>
where
    C: CompositeEncoder,
{
    fn run(
        _src: &S,
        _codec: &mut C,
        _skip_acc: usize,
        _vectored_amount: usize,
    ) -> Result<(), C::Error> {
        Ok(())
    }
}
