use core::primitive::usize;
use std::any::type_name;

use crate::{BinaryEncoder, CompositeEncoder, Encode};

use super::{
    edge::PhantomEdge,
    end::End,
    field::Field,
    len::Len,
    pad::{ConstPadding, Padding},
};

pub trait Actor<S, C> {
    fn run_at(action: &mut Ctx<S, C>, _index: usize) -> Continuous;

    fn run(action: &mut Ctx<S, C>);
}

#[derive(Debug)]
pub enum Continuous {
    Next,
    Done,
}

#[derive(Debug)]
pub enum Ctx<T, C> {
    Encode {
        src: *const T,
        coder: C,
    },
    Decode {
        src: *const T,
        dst: *mut T,
        coder: C,
    },
    Drop {
        ptr: *const T,
        coder: C,
    },
}

impl<S, C, A, B> Actor<S, C> for PhantomEdge<S, (Field<A>, B)>
where
    Self: Len,
    C: CompositeEncoder + BinaryEncoder,
    Field<A>: Actor<S, C>,
    B: Actor<S, C>,
    A: Encode,
    [(); <Self as Len>::SIZE]:,
{
    fn run_at(action: &mut Ctx<S, C>, mut index: usize) -> Continuous {
        if index == 0 {
            Self::run(action);
            return Continuous::Done;
        }
        index -= 1;
        match Field::<A>::run_at(action, index) {
            Continuous::Next => B::run_at(action, index),
            Continuous::Done => Continuous::Done,
        }
    }

    fn run(action: &mut Ctx<S, C>) {
        println!("field {} {}", <Self as Len>::SIZE, type_name::<A>());
        match action {
            Ctx::Encode { src, coder } => {
                if Self::SIZE == 0 {
                    *src = unsafe { src.byte_add(size_of::<A>()) };
                } else {
                    unsafe {
                        let slice = &*(*src as *const [u8; Self::SIZE]);
                        coder.encode_slice::<{ Self::SIZE }>(slice);
                        *src = src.byte_add(Self::SIZE);
                    }
                }
            }
            Ctx::Decode { src, dst, coder } => {}
            Ctx::Drop { ptr, coder } => {}
        }
    }
}

impl<S, S2, C, B, const I: usize> Actor<S, C> for PhantomEdge<S, (ConstPadding<S2, I>, B)>
where
    Self: Len,
    ConstPadding<S2, I>: Actor<S, C>,
    B: Actor<S, C>,
{
    fn run_at(action: &mut Ctx<S, C>, mut index: usize) -> Continuous {
        if index == 0 {
            Self::run(action);
            return Continuous::Done;
        }
        index -= 1;
        match ConstPadding::<S2, I>::run_at(action, index) {
            Continuous::Next => B::run_at(action, index),
            Continuous::Done => Continuous::Done,
        }
    }

    fn run(action: &mut Ctx<S, C>) {
        match action {
            Ctx::Encode { src, coder: _ } => {
                *src = unsafe { src.byte_add(I) };
            }
            Ctx::Decode {
                src: _,
                dst: _,
                coder: _,
            } => {}
            Ctx::Drop { ptr: _, coder: _ } => {}
        }
        println!("padding {} ", I);
    }
}

// 1 + 3 + 10 + 0 + 6 + 0 + 2 + 1 + 2 + 6 + 2 + 1 + 2 + 4 + 4
// field 1 model::_::__FieldToken<u8, 0>
// padding 3
// field 10 model::_::__FieldToken<u32, 0>
// padding 0
// field 6 model::_::__FieldToken<u32, 1>
// padding 0
// field 2 model::_::__FieldToken<u8, 0>
// padding 0
// field 1 model::_::__FieldToken<u8, 1>
// padding 0
// padding 2
// padding 0
// field 0 model::_::__FieldToken<alloc::vec::Vec<u8>, 2>
// padding 0
// field 6 model::_::__FieldToken<u32, 3>
// padding 0
// field 2 model::_::__FieldToken<u8, 0>
// padding 0
// field 1 model::_::__FieldToken<u8, 1>
// padding 0
// padding 2
// field 4 model::_::__FieldToken<u32, 5>
// padding 4

impl<S, C> Actor<S, C> for End<S> {
    fn run_at(action: &mut Ctx<S, C>, _index: usize) -> Continuous {
        Continuous::Next
    }

    fn run(_action: &mut Ctx<S, C>) {
        unreachable!()
    }
}

impl<S, C, FrontOffset> Actor<S, C> for Padding<S, FrontOffset> {
    fn run_at(_action: &mut Ctx<S, C>, _index: usize) -> Continuous {
        Continuous::Next
    }

    fn run(_action: &mut Ctx<S, C>) {
        unreachable!()
    }
}

impl<S, C, const I: usize> Actor<S, C> for ConstPadding<S, I> {
    fn run_at(_action: &mut Ctx<S, C>, _index: usize) -> Continuous {
        Continuous::Next
    }

    fn run(_action: &mut Ctx<S, C>) {
        unreachable!()
    }
}

impl<S, C, T> Actor<S, C> for Field<T> {
    fn run_at(_action: &mut Ctx<S, C>, _index: usize) -> Continuous {
        Continuous::Next
    }

    fn run(_action: &mut Ctx<S, C>) {
        unreachable!()
    }
}
