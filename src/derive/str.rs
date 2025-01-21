use typenum::Const;

use crate::{
    impl_field_token,
    prelude::{
        CompoundUnwrapper, CompoundWrapper, Edge, End, Field, FieldOffset, Len, PhantomEdge, Size,
        Vector, Vectored, UNSIZED,
    },
    Encode, Encoder,
};

// impl Encode for String {
//     fn encode<E: crate::Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
//         encoder.encode_u8(&(self.len() as u8))?;
//         encoder.encode_bytes(self.as_bytes())?;
//         Ok(())
//     }
// }

impl Encode for &'static str {
    fn encode<E: crate::Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        encoder.encode_str(self)
    }
}

impl Encode for String {
    fn encode<E: Encoder>(&self, _encoder: &mut E) -> Result<(), E::Error> {
        #[cfg(debug_assertions)]
        println!("HI vec<T> encoding!");
        Ok(())
    }
}

impl Vector for String {
    type Item = u8;

    fn as_iter(&self) -> impl Iterator<Item = &Self::Item> {
        self.as_bytes().iter()
    }

    fn as_ptr(&self) -> *const Self::Item {
        self.as_bytes().as_ptr()
    }

    fn len(&self) -> usize {
        self.len()
    }
}

const _: () = {
    impl_field_token!();

    impl FieldOffset for __FieldToken<u8, 0> {
        type Offset = Const<0>;
    }

    impl FieldOffset for __FieldToken<u8, 1> {
        type Offset = Const<{ <u8 as Size>::SIZE }>;
    }

    impl Edge for String {
        type First = End<Self>;

        type Second = PhantomEdge<Self, (Vectored<Self, __FieldToken<u8, 1>>, End<Self>)>;
    }

    impl Len for String {
        const SIZE: usize = UNSIZED;
    }

    impl Size for String {
        const SIZE: usize = size_of::<Self>();
    }

    impl<S> CompoundWrapper<S> for String {
        type Compound = <Self as CompoundUnwrapper<S>>::Output;
    }
};
