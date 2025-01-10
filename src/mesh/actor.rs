use std::any::type_name;

use super::{edge::PhantomEdge, end::End, field::PhantomField, len::Len, padding::Padding};

pub trait Actor {
    fn run_at(_index: usize) -> Continuous;

    fn run();
}

pub enum Continuous {
    Continue,
    Break,
}

impl<S, S2, A, B, const I: usize> Actor for PhantomEdge<S, (PhantomField<S2, A, I>, B)>
where
    PhantomField<S2, A, I>: Actor + Len,
    B: Actor,
{
    fn run_at(mut index: usize) -> Continuous {
        if index == 0 {
            Self::run();
            return Continuous::Break;
        }
        index -= 1;
        if let Continuous::Continue = PhantomField::<S2, A, I>::run_at(index) {
            B::run_at(index)
        } else {
            Continuous::Break
        }
    }

    fn run() {
        println!(
            "field {} {}",
            <PhantomField<S2, A, I> as Len>::SIZE,
            type_name::<A>()
        );
    }
}

impl<S, S2, B, FrontOffset> Actor for PhantomEdge<S, (Padding<S2, FrontOffset>, B)>
where
    Self: Len,
    Padding<S2, FrontOffset>: Actor,
    B: Actor,
{
    fn run_at(mut index: usize) -> Continuous {
        if index == 0 {
            Self::run();
            return Continuous::Break;
        }
        index -= 1;
        if let Continuous::Continue = Padding::<S2, FrontOffset>::run_at(index) {
            B::run_at(index)
        } else {
            Continuous::Break
        }
    }

    fn run() {
        println!("padding {}", <Self as Len>::SIZE);
    }
}

impl<S> Actor for End<S> {
    fn run_at(_index: usize) -> Continuous {
        Continuous::Continue
    }

    fn run() {}
}

impl<S, FrontOffset> Actor for Padding<S, FrontOffset> {
    fn run_at(_index: usize) -> Continuous {
        Continuous::Continue
    }

    fn run() {}
}
