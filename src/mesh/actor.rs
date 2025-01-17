use std::{any::type_name, mem::transmute};

use crate::{CompositeEncoder, Encode};

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
    fn run_at(src: &mut &S, codec: &mut C, skip_acc: &mut usize, _index: usize) -> Continuous<C>;

    fn run(src: &mut &S, codec: &mut C) -> Result<(), C::Error>;
}

#[derive(Debug)]
pub enum Continuous<C>
where
    C: CompositeEncoder,
{
    Next,
    Done(Result<(), C::Error>),
}

fn add_to_src<S>(src: &mut &S, add: usize) {
    *src = unsafe { &*((*src) as *const S).byte_add(add) };
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
    fn run_at(
        src: &mut &S,
        codec: &mut C,
        skip_acc: &mut usize,
        mut index: usize,
    ) -> Continuous<C> {
        if index == 0 {
            #[cfg(debug_assertions)]
            println!("{}", skip_acc);
            if *skip_acc == 0 {
                *skip_acc = Self::SIZE;
                let result = Self::run(src, codec);
                return Continuous::Done(result);
            } else {
                *skip_acc -= size_of::<A>();
                #[cfg(debug_assertions)]
                println!("HHHHH");
                return Continuous::Done(Ok(()));
            }
        }
        index -= 1;
        match Field::<A>::run_at(src, codec, skip_acc, index) {
            Continuous::Next => B::run_at(src, codec, skip_acc, index),
            Continuous::Done(result) => Continuous::Done(result),
        }
    }

    fn run(src: &mut &S, codec: &mut C) -> Result<(), C::Error> {
        #[cfg(debug_assertions)]
        println!("field {} {}", <Self as Len>::SIZE, type_name::<A>());
        if Self::SIZE == 0 {
            // /unsafe { codec.encode_element::<A>(transmute(*src))? };
            add_to_src(src, size_of::<A>())
        } else {
            // unsafe { codec.encode_slice::<{ Self::SIZE }>(transmute(*src)) };
            add_to_src(src, Self::SIZE);
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
        src: &mut &S,
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
            #[cfg(debug_assertions)]
            println!("padding {} ", I);
            add_to_src(src, I);
            return Continuous::Done(Ok(()));
        }
        index -= 1;
        match ConstPadding::<S2, I>::run_at(src, codec, skip_acc, index) {
            Continuous::Next => B::run_at(src, codec, skip_acc, index),
            Continuous::Done(result) => Continuous::Done(result),
        }
    }

    fn run(src: &mut &S, _codec: &mut C) -> Result<(), C::Error> {
        unreachable!()
    }
}

impl<S, C> EncodeActor<S, C> for End<S>
where
    C: CompositeEncoder,
{
    fn run_at(src: &mut &S, codec: &mut C, skip_acc: &mut usize, _index: usize) -> Continuous<C> {
        Continuous::Next
    }

    fn run(src: &mut &S, codec: &mut C) -> Result<(), C::Error> {
        unreachable!()
    }
}

impl<S, C, FrontOffset> EncodeActor<S, C> for Padding<S, FrontOffset>
where
    C: CompositeEncoder,
{
    fn run_at(src: &mut &S, codec: &mut C, skip_acc: &mut usize, _index: usize) -> Continuous<C> {
        Continuous::Next
    }

    fn run(src: &mut &S, codec: &mut C) -> Result<(), C::Error> {
        unreachable!()
    }
}

impl<S, C, const I: usize> EncodeActor<S, C> for ConstPadding<S, I>
where
    C: CompositeEncoder,
{
    fn run_at(src: &mut &S, codec: &mut C, skip_acc: &mut usize, _index: usize) -> Continuous<C> {
        Continuous::Next
    }

    fn run(src: &mut &S, codec: &mut C) -> Result<(), C::Error> {
        unreachable!()
    }
}

impl<S, C, T> EncodeActor<S, C> for Field<T>
where
    C: CompositeEncoder,
{
    fn run_at(src: &mut &S, codec: &mut C, skip_acc: &mut usize, _index: usize) -> Continuous<C> {
        Continuous::Next
    }

    fn run(src: &mut &S, codec: &mut C) -> Result<(), C::Error> {
        unreachable!()
    }
}
