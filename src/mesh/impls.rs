use super::{
    actor::{Actor, Continuous},
    compound::CompoundWrapper,
    edge::Edge,
    end::End,
    leaf::PhantomLeaf,
    size::{Size, UNSIZED},
};

macro_rules! impl_serializable {
    ($($type:ty),*) => {
        $(
        impl Edge for $type {
            type First = End;
            type Second = End;
        }

        impl Actor for $type {
            fn run_at(_index: usize) -> Continuous {
                Continuous::Continue
            }

            fn run() {}
        }


        impl<S> CompoundWrapper<S> for $type {
            type Compound = PhantomLeaf<S, Self>;
        }
        )*
    };
}

macro_rules! impl_primitives {
    ($($type:ty),*) => {
        impl_serializable!($($type),*);

        $(impl Size for $type {
            const SIZE: usize = size_of::<Self>();
        })*
    };
}

macro_rules! impl_non_primitives {
    ($($type:ty),*) => {
        impl_serializable!($($type),*);

        $(impl Size for $type {
            const SIZE: usize = UNSIZED;
        })*
    };
}

impl_primitives!(u8, u32, ());
impl_non_primitives!(Vec<u8>);
