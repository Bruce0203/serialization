use std::mem::{transmute, MaybeUninit};

use typenum::Const;

use crate::{
    impl_field_token,
    prelude::{
        CompoundUnwrapper, CompoundWrapper, Edge, End, Field, FieldOffset, Len, PhantomEdge, Size,
        Vector, Vectored, UNSIZED,
    },
    Decode, Decoder, Encode, Encoder,
};

impl<'a> Encode for &'a str {
    fn encode<E: crate::Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        encoder.encode_str(self)
    }
}

impl<'a> Decode for &'a str {
    fn decode_in_place<D: crate::Decoder>(
        decoder: &mut D,
        out: &mut MaybeUninit<Self>,
    ) -> Result<(), D::Error> {
        decoder.decode_str(out)
    }
}

impl Encode for String {
    fn encode<E: Encoder>(&self, _encoder: &mut E) -> Result<(), E::Error> {
        #[cfg(debug_assertions)]
        println!("HI vec<T> encoding!");
        Ok(())
    }
}

//TODO try remove impl block
impl Decode for String {
    fn decode_in_place<D: Decoder>(
        decoder: &mut D,
        out: &mut MaybeUninit<Self>,
    ) -> Result<(), D::Error> {
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

    fn set_len(&mut self, len: usize) {
        let mut s = unsafe { transmute::<_, &mut MaybeUninit<String>>(self) };
        *s = MaybeUninit::new(String::with_capacity(len));
        unsafe { s.assume_init_mut().as_mut_vec().set_len(len) };
    }
}

const _: () = {
    impl_field_token!();

    impl FieldOffset for __FieldToken<String, String, 0> {
        // type Offset = Const<{ <u8 as Size>::SIZE }>;
        type Offset = Const<0>;
    }

    impl<C> Edge<C> for String {
        type First = End<C, Self>;

        type Second =
            PhantomEdge<C, Self, (Vectored<__FieldToken<String, String, 0>>, End<C, Self>)>;
    }

    impl Len for String {
        const SIZE: usize = UNSIZED;
    }

    impl Size for String {
        const SIZE: usize = size_of::<Self>();
    }

    impl<C, S> CompoundWrapper<C, S> for String {
        type Compound = <Self as CompoundUnwrapper<C, S>>::Output;
    }
};
