use std::mem::MaybeUninit;

use typenum::Const;

use crate::{
    impl_field_token,
    prelude::{
        CompoundUnwrapper, CompoundWrapper, Edge, End, Field, FieldOffset, Len, PhantomEdge, Size,
        UNSIZED,
    },
    Decode, Decoder, Encode, Encoder,
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
    fn decode_in_place<D: Decoder>(
        _decoder: &mut D,
        _out: &mut MaybeUninit<Self>,
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
        //TODO the size of discriminant is different Codec
        type Offset = Const<1>;
    }

    impl<C, T> Edge<C> for Option<T>
    where
        T: Edge<C>,
    {
        type First = End<C, Self>;

        type Second = PhantomEdge<
            C,
            Self,
            (
                Field<__FieldToken<Option<T>, Option<T>, 0>>,
                PhantomEdge<C, Self, (Field<__FieldToken<Option<T>, T, 1>>, End<C, Self>)>,
            ),
        >;
    }

    impl<T> Len for Option<T> {
        const SIZE: usize = UNSIZED;
    }

    impl<T> Size for Option<T> {
        const SIZE: usize = size_of::<Self>();
    }

    impl<C, S, T> CompoundWrapper<C, S> for Option<T>
    where
        T: Edge<C>,
    {
        type Compound = <Self as CompoundUnwrapper<C, S>>::Output;
    }
};
