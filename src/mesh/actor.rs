use core::primitive::usize;
use std::any::type_name;

use super::{edge::PhantomEdge, end::End, field::PhantomField, len::Len, padding::Padding};

pub trait Actor {
    fn run_at(_index: usize) -> Continuous;

    fn run();
}

pub enum Continuous {
    Next,
    Done,
}

impl<S, S2, A, B, const I: usize> Actor for PhantomEdge<S, (PhantomField<S2, A, I>, B)>
where
    Self: Len,
    PhantomField<S2, A, I>: Actor,
    B: Actor,
{
    fn run_at(mut index: usize) -> Continuous {
        if index == 0 {
            Self::run();
            return Continuous::Done;
        }
        index -= 1;
        if let Continuous::Next = PhantomField::<S2, A, I>::run_at(index) {
            B::run_at(index)
        } else {
            Continuous::Done
        }
    }

    fn run() {
        println!("field {} {}", <Self as Len>::SIZE, type_name::<A>());
    }
}

impl<S, S2, B, FrontOffset> Actor for PhantomEdge<S, (Padding<S2, FrontOffset>, B)>
where
    Self: Len,
    Padding<S2, FrontOffset>: Actor,
    B: Actor,
{
    fn run_at(index: usize) -> Continuous {
        if let Continuous::Next = Padding::<S2, FrontOffset>::run_at(index) {
            B::run_at(index)
        } else {
            Continuous::Done
        }
    }

    fn run() {
        unreachable!()
    }
}

impl<S> Actor for End<S> {
    fn run_at(_index: usize) -> Continuous {
        Continuous::Next
    }

    fn run() {}
}

impl<S, FrontOffset> Actor for Padding<S, FrontOffset> {
    fn run_at(_index: usize) -> Continuous {
        Continuous::Next
    }

    fn run() {
        unreachable!()
    }
}

impl<S, T, const I: usize> Actor for PhantomField<S, T, I> {
    fn run_at(_index: usize) -> Continuous {
        Continuous::Next
    }

    fn run() {
        unreachable!()
    }
}
