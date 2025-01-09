use super::{CompoundWrapper, Edge, PhantomField, PhantomLeaf};

macro_rules! impl_serializable {
    ($($type:ty),*) => {
        $(
        impl Edge for $type {}

        impl<S> CompoundWrapper<S> for $type {
            type Compound = PhantomLeaf<S, Self>;
        }

        impl<S, const I: usize> CompoundWrapper<S> for PhantomField<S, $type, I> {
            type Compound = PhantomLeaf<S, $type>;
        }
        )*
    };
}

impl_serializable!(u8, u32, Vec<u8>, ());
