use std::mem::MaybeUninit;

use typenum::Const;

use crate::{
    impl_field_token,
    prelude::{
        CompoundUnwrapper, CompoundWrapper, Edge, End, Field, FieldOffset, Len, PhantomEdge, Size,
        UNSIZED,
    },
    Decode, Encode, Encoder,
};

impl<T> Encode for Option<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        if self.is_some() {
            encoder.encode_some()
        } else {
            encoder.encode_none()
        }
    }
}

impl<T> Decode for Option<T> {
    fn decode_in_place<D: crate::Decoder>(
        decoder: &mut D,
        out: &mut MaybeUninit<Self>,
    ) -> Result<(), D::Error> {
        todo!()
    }
}

const _: () = {
    impl_field_token!();

    impl<T> FieldOffset for __FieldToken<Option<T>, Option<T>, 0> {
        type Offset = Const<0>;
    }

    impl<T> FieldOffset for __FieldToken<Option<T>, T, 1> {
        // type Offset = Const<{ <u8 as Size>::SIZE }>;
        type Offset = Const<1>;
    }

    impl<T> Edge for Option<T>
    where
        T: Edge,
    {
        type First = End<Self>;

        type Second = PhantomEdge<
            Self,
            (
                Field<__FieldToken<Option<T>, Option<T>, 0>>,
                PhantomEdge<Self, (Field<__FieldToken<Option<T>, T, 1>>, End<Self>)>,
            ),
        >;
    }

    impl<T> Len for Option<T> {
        const SIZE: usize = UNSIZED;
    }

    impl<T> Size for Option<T> {
        const SIZE: usize = size_of::<Self>();
    }

    impl<S, T> CompoundWrapper<S> for Option<T>
    where
        T: Edge,
    {
        type Compound = <Self as CompoundUnwrapper<S>>::Output;
    }
};
