use super::{CompoundWrapper, Edge, PhantomLeaf};

macro_rules! impl_serializable {
    ($($type:ty),*) => {
        $(
        impl Edge for $type {}

        impl<S> CompoundWrapper<S> for $type {
            type Compound = PhantomLeaf<S, Self>;
        }
        )*
    };
}

impl_serializable!(u8, u32, Vec<u8>, ());
