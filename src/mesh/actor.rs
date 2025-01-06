use std::any::type_name;

use crate::trim;

use super::{Edge, End};

pub trait Actor {
    fn run_at(index: usize) -> Result<(), ()>;
    fn run();
}

impl<T> Actor for T
where
    T: Edge,
{
    default fn run_at(index: usize) -> Result<(), ()> {
        println!("{}", trim!(type_name::<T>()));
        if index == 0 {
            T::run();
            return Err(());
        }
        let index = index - 1;
        T::First::run_at(index)?;
        T::Second::run_at(index)?;
        Ok(())
    }

    default fn run() {}
}

impl Actor for () {
    default fn run_at(_index: usize) -> Result<(), ()> {
        Ok(())
    }

    default fn run() {
        unreachable!()
    }
}

impl Actor for End {
    default fn run_at(_index: usize) -> Result<(), ()> {
        Ok(())
    }

    default fn run() {
        unreachable!()
    }
}
