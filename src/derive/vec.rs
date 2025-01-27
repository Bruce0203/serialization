use std::mem::{transmute, MaybeUninit};

use typenum::Const;

use crate::prelude::{
    CompoundUnwrapper, CompoundWrapper, Edge, End, Field, FieldOffset, Len, PhantomEdge, Size,
    Vector, Vectored, UNSIZED,
};
use crate::{impl_field_token, Decode, Encode, Encoder};

impl<T> Encode for Vec<T>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, _encoder: &mut E) -> Result<(), E::Error> {
        #[cfg(debug_assertions)]
        println!("HI vec<T> encoding!");
        Ok(())
    }
}

impl<T> Decode for Vec<T> {
    fn decode_in_place<D: crate::Decoder>(
        decoder: &mut D,
        out: &mut MaybeUninit<Self>,
    ) -> Result<(), D::Error> {
        todo!()
    }
}

impl<T> Vector for Vec<T> {
    type Item = T;

    fn as_iter(&self) -> impl Iterator<Item = &Self::Item> {
        self.iter()
    }

    fn as_ptr(&self) -> *const Self::Item {
        self.as_ptr()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn set_len(&mut self, len: usize) {
        let mut vec = unsafe { transmute::<_, &mut MaybeUninit<Vec<T>>>(self) };
        *vec = MaybeUninit::new(Vec::with_capacity(len));
        unsafe { vec.assume_init_mut().set_len(len) };
    }
}

const _: () = {
    impl_field_token!();

    impl<T> FieldOffset for __FieldToken<Vec<T>, Vec<T>, 0> {
        type Offset = Const<0>;
    }

    impl<C, T> Edge<C> for Vec<T>
    where
        T: Edge<C>,
    {
        type First = End<C, Self>;

        type Second =
            PhantomEdge<C, Self, (Vectored<__FieldToken<Vec<T>, Vec<T>, 0>>, End<C, Self>)>;
    }

    impl<T> Len for Vec<T>
    where
        T: Len,
    {
        const SIZE: usize = UNSIZED;
    }

    impl<T> Size for Vec<T>
    where
        T: Size,
    {
        const SIZE: usize = size_of::<Self>();
    }

    impl<C, S, T> CompoundWrapper<C, S> for Vec<T>
    where
        T: Edge<C>,
    {
        type Compound = <Self as CompoundUnwrapper<C, S>>::Output;
    }
};
