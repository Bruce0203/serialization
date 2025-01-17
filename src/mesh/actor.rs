use core::primitive::usize;
use std::{any::type_name, iter::Skip, mem::transmute};

use crate::{CompositeDecoder, CompositeEncoder, Encode};

use super::{
    edge::PhantomEdge,
    end::End,
    field::Field,
    len::Len,
    pad::{ConstPadding, Padding},
};

pub trait EncodeActor<S, C>
where
    C: CompositeEncoder,
{
    fn run_at<'a>(src: &S, codec: &mut C, skip_acc: usize, _index: usize) -> Continuous<'a, S, C>;

    fn run(src: &S, codec: &mut C) -> Result<(), C::Error>;
}

#[derive(Debug)]
pub enum Continuous<'a, S, C>
where
    C: CompositeEncoder,
{
    Next { src: &'a S, skip_acc: usize },
    Done(Result<(), C::Error>),
}

impl<S, C, A, B> EncodeActor<S, C> for PhantomEdge<S, (Field<A>, B)>
where
    Self: Len,
    C: CompositeEncoder,
    Field<A>: EncodeActor<S, C>,
    B: EncodeActor<S, C>,
    A: Encode,
    [(); <Self as Len>::SIZE]:,
{
    fn run_at<'a>(
        src: &S,
        codec: &mut C,
        mut skip_acc: usize,
        mut index: usize,
    ) -> Continuous<'a, S, C> {
        if index == 0 {
            #[cfg(debug_assertions)]
            println!("{}", skip_acc);
            if skip_acc == 0 {
                skip_acc = Self::SIZE;
                let result = Self::run(src, codec);
                return Continuous::Done(result);
            } else {
                skip_acc -= size_of::<A>();
                #[cfg(debug_assertions)]
                println!("HHHHH");
                return Continuous::Done(Ok(()));
            }
        }
        index -= 1;
        match Field::<A>::run_at(src, codec, skip_acc, index) {
            Continuous::Next { src, skip_acc } => B::run_at(src, codec, skip_acc, index),
            Continuous::Done(result) => Continuous::Done(result),
        }
    }

    fn run(src: &mut *const S, codec: &mut C) -> Result<(), C::Error> {
        #[cfg(debug_assertions)]
        println!("field {} {}", <Self as Len>::SIZE, type_name::<A>());
        if Self::SIZE == 0 {
            // unsafe {
            //     let src: &A = transmute(*src);
            //     codec.encode_element(src)?;
            // }
            *src = unsafe { (*src).byte_add(size_of::<A>()) };
        } else {
            unsafe {
                let slice = (*src) as *const _ as *const [u8; Self::SIZE];
                codec.encode_slice::<{ Self::SIZE }>(&*slice);
                *src = (*src).byte_add(Self::SIZE);
            }
        }
        Ok(())
    }
}

impl<S, S2, C, B, const I: usize> EncodeActor<S, C> for PhantomEdge<S, (ConstPadding<S2, I>, B)>
where
    C: CompositeEncoder,
    Self: Len,
    ConstPadding<S2, I>: EncodeActor<S, C>,
    B: EncodeActor<S, C>,
{
    fn run_at(
        src: &mut *const S,
        codec: &mut C,
        skip_acc: &mut usize,
        mut index: usize,
    ) -> Continuous<C> {
        if index == 0 {
            if *skip_acc != 0 && I != 0 {
                *skip_acc = 0;
            }
            #[cfg(debug_assertions)]
            println!("skip_acc = {}", skip_acc);
            let result = Self::run(src, codec);
            return Continuous::Done(result);
        }
        index -= 1;
        match ConstPadding::<S2, I>::run_at(src, codec, skip_acc, index) {
            Continuous::Next => B::run_at(src, codec, skip_acc, index),
            Continuous::Done(result) => Continuous::Done(result),
        }
    }

    fn run(src: &mut *const S, _codec: &mut C) -> Result<(), C::Error> {
        #[cfg(debug_assertions)]
        println!("padding {} ", I);
        *src = unsafe { (*src).byte_add(I) };
        Ok(())
    }
}

impl<S, C> EncodeActor<S, C> for End<S>
where
    C: CompositeEncoder,
{
    fn run_at(
        src: &mut *const S,
        codec: &mut C,
        skip_acc: &mut usize,
        _index: usize,
    ) -> Continuous<C> {
        Continuous::Next
    }

    fn run(src: &mut *const S, codec: &mut C) -> Result<(), C::Error> {
        unreachable!()
    }
}

impl<S, C, FrontOffset> EncodeActor<S, C> for Padding<S, FrontOffset>
where
    C: CompositeEncoder,
{
    fn run_at(
        src: &mut *const S,
        codec: &mut C,
        skip_acc: &mut usize,
        _index: usize,
    ) -> Continuous<C> {
        Continuous::Next
    }

    fn run(src: &mut *const S, codec: &mut C) -> Result<(), C::Error> {
        unreachable!()
    }
}

impl<S, C, const I: usize> EncodeActor<S, C> for ConstPadding<S, I>
where
    C: CompositeEncoder,
{
    fn run_at(
        src: &mut *const S,
        codec: &mut C,
        skip_acc: &mut usize,
        _index: usize,
    ) -> Continuous<C> {
        Continuous::Next
    }

    fn run(src: &mut *const S, codec: &mut C) -> Result<(), C::Error> {
        unreachable!()
    }
}

impl<S, C, T> EncodeActor<S, C> for Field<T>
where
    C: CompositeEncoder,
{
    fn run_at(
        src: &mut *const S,
        codec: &mut C,
        skip_acc: &mut usize,
        _index: usize,
    ) -> Continuous<C> {
        Continuous::Next
    }

    fn run(src: &mut *const S, codec: &mut C) -> Result<(), C::Error> {
        unreachable!()
    }
}
