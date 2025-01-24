use std::mem::{transmute, MaybeUninit};

use typenum::Const;

use crate::{
    prelude::{
        CompoundUnwrapper, CompoundWrapper, Edge, End, Field, FieldOffset, Len, PhantomEdge, Size,
        Vector, Vectored, UNSIZED,
    },
    Decode, Decoder, Encode, Encoder,
};

impl<T, const N: usize> Decode for [T; N] {
    fn decode_in_place<D: Decoder>(
        decoder: &mut D,
        out: &mut MaybeUninit<Self>,
    ) -> Result<(), D::Error> {
        todo!()
    }
}

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

    fn set_len(&mut self, len: usize) {}
}

const _: () = {
    impl<T, const N: usize> FieldOffset for [T; N] {
        type Offset = Const<0>;
    }

    impl<C, T, const N: usize> Edge<C> for [T; N]
    where
        T: Edge<C>,
    {
        type First = End<C, Self>;

        type Second = PhantomEdge<C, Self, (Field<Vectored<[T; N]>>, End<C, Self>)>;
    }

    impl<T, const N: usize> Len for [T; N] {
        const SIZE: usize = UNSIZED;
    }

    impl<T, const N: usize> Size for [T; N] {
        const SIZE: usize = size_of::<Self>();
    }

    impl<C, S, T, const N: usize> CompoundWrapper<C, S> for [T; N]
    where
        T: Edge<C>,
    {
        type Compound = <Self as CompoundUnwrapper<C, S>>::Output;
    }
};
