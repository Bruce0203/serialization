use std::mem::transmute;

use typenum::Const;

use crate::{
    prelude::{
        CompoundUnwrapper, CompoundWrapper, Edge, End, Field, FieldOffset, Len, PhantomEdge, Size,
        Vector, Vectored, UNSIZED,
    },
    Encode, Encoder,
};

impl<T, const N: usize> Encode for [T; N] {
    fn encode<E: Encoder>(&self, _encoder: &mut E) -> Result<(), E::Error> {
        todo!()
    }
}

impl<T, const N: usize> Vector for [T; N] {
    type Item = T;

    fn as_iter(&self) -> impl Iterator<Item = &Self::Item> {
        self.iter()
    }

    fn as_ptr(&self) -> *const Self::Item {
        unsafe { transmute(self) }
    }

    fn len(&self) -> usize {
        N
    }
}

const _: () = {
    impl<T, const N: usize> FieldOffset for [T; N] {
        type Offset = Const<0>;
    }

    impl<T, const N: usize> Edge for [T; N]
    where
        T: Edge,
    {
        type First = End<Self>;

        type Second = PhantomEdge<Self, (Field<Vectored<[T; N], [T; N]>>, End<Self>)>;
    }

    impl<T, const N: usize> Len for [T; N] {
        const SIZE: usize = UNSIZED;
    }

    impl<T, const N: usize> Size for [T; N] {
        const SIZE: usize = size_of::<Self>();
    }

    impl<S, T, const N: usize> CompoundWrapper<S> for [T; N]
    where
        T: Edge,
    {
        type Compound = <Self as CompoundUnwrapper<S>>::Output;
    }
};
