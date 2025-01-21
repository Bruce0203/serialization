use typenum::Const;

use crate::{
    impl_enum_mesh, impl_field_token, impl_mesh, offset_of, offset_of_enum,
    prelude::{
        CompoundUnwrapper, CompoundWrapper, Edge, End, Field, FieldOffset, Len, PhantomEdge, Size,
        Vector, Vectored, UNSIZED,
    },
    Encode, Encoder,
};

impl<A, B> Encode for (A, B)
where
    A: Encode,
    B: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        A::encode(&self.0, encoder)?;
        B::encode(&self.1, encoder)?;
        Ok(())
    }
}
const _: () = {
    impl_field_token!();

    impl<A> FieldOffset for __FieldToken<A, 0> {
        type Offset = Const<{0}>;
    }

    impl<B> FieldOffset for __FieldToken<B, 1> {
        type Offset = Const<0>;
    }

    impl<A, B> Edge for (A, B)
    where
        A: Edge,
        B: Edge,
    {
        type First = End<Self>;

        type Second = PhantomEdge<
            Self,
            (
                Field<__FieldToken<A, 0>>,
                PhantomEdge<Self, (Field<__FieldToken<B, 1>>, End<Self>)>,
            ),
        >;
    }
};
