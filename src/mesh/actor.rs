use core::primitive::usize;
use std::any::type_name;

use super::{edge::PhantomEdge, end::End, field::Field, len::Len, padding::Padding};

pub trait Actor {
    fn run_at(_index: usize) -> Continuous;

    fn run();
}

#[derive(Debug)]
pub enum Continuous {
    Next,
    Done,
}

impl<S, A, B> Actor for PhantomEdge<S, (Field<A>, B)>
where
    Self: Len,
    Field<A>: Actor,
    B: Actor,
{
    fn run_at(mut index: usize) -> Continuous {
        if index == 0 {
            Self::run();
            return Continuous::Done;
        }
        index -= 1;
        if let Continuous::Next = Field::<A>::run_at(index) {
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

    fn run() {
        //TODO try remove
        unreachable!()
    }
}

impl<S, FrontOffset> Actor for Padding<S, FrontOffset> {
    fn run_at(_index: usize) -> Continuous {
        Continuous::Next
    }

    fn run() {
        unreachable!()
    }
}

impl<T> Actor for Field<T> {
    fn run_at(_index: usize) -> Continuous {
        Continuous::Next
    }

    fn run() {
        unreachable!()
    }
}
