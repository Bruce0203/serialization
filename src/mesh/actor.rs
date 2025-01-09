use std::any::type_name;

use super::{edge::PhantomEdge, end::End, size::Size};

pub trait Actor {
    fn run_at(_index: usize) -> Continuous;

    fn run();
}

pub enum Continuous {
    Continue,
    Break,
}

impl<S, A, B> Actor for PhantomEdge<S, (A, B)>
where
    Self: Size,
    A: Actor,
    B: Actor,
{
    fn run_at(index: usize) -> Continuous {
        #[cfg(debug_assertions)]
        println!(
            "size({}) = {}",
            Self::SIZE,
            crate::trim!(type_name::<Self>())
        );
        if index == 0 {
            Self::run();
            return Continuous::Break;
        }
        let index = index - 1;
        if let Continuous::Continue = A::run_at(index) {
            B::run_at(index)
        } else {
            Continuous::Break
        }
    }

    fn run() {}
}

impl Actor for End {
    fn run_at(_index: usize) -> Continuous {
        Continuous::Continue
    }

    fn run() {}
}
