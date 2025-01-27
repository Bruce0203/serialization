use std::{
    mem::{transmute, MaybeUninit},
    ops::Add,
};

use crate::{Decode, Decoder, Encode, Encoder};

use super::{
    edge::{Edge, PhantomEdge},
    end::End,
    field::FieldOffset,
    flatten::CompoundWrapper,
    len::{Len, Size},
    prelude::Vectored,
};

pub struct Enum<T>(MaybeUninit<T>);

impl<T> Encode for Enum<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        todo!()
    }
}

impl<T> Decode for Enum<T> {
    fn decode_in_place<D: Decoder>(
        decoder: &mut D,
        out: &mut MaybeUninit<Self>,
    ) -> Result<(), D::Error> {
        todo!()
    }
}

impl<C, S, T> Add<End<C, S>> for Enum<T> {
    type Output = PhantomEdge<C, S, (Enum<T>, End<C, S>)>;

    fn add(self, rhs: End<C, S>) -> Self::Output {
        unreachable!()
    }
}

impl<C, S, B, T> Add<PhantomEdge<C, S, B>> for Enum<T> {
    type Output = PhantomEdge<C, S, (Enum<T>, PhantomEdge<C, S, B>)>;

    fn add(self, rhs: PhantomEdge<C, S, B>) -> Self::Output {
        unreachable!()
    }
}

impl<T> FieldOffset for Enum<T>
where
    T: FieldOffset,
{
    type Offset = T::Offset;
}

impl<C, T> Edge<C> for Enum<T> {
    type First = End<C, Self>;

    type Second = End<C, Self>;
}

impl<T> Len for Enum<T>
where
    T: Size,
{
    const SIZE: usize = T::SIZE;
}

impl<C, S, T> CompoundWrapper<C, S> for Enum<T> {
    type Compound = Enum<T>;
}

fn get_memory_index_of_enum_discriminant<T>(value: &T) -> Option<usize>
where
    T: Sized,
    [(); size_of::<T>()]:,
{
    let punchboard: &[u8; size_of::<T>()] = unsafe { transmute(value) };
    for i in punchboard {
        if *i != 0 {}
    }
    todo!()
}

#[cfg(test)]
mod tests {
    use std::{any::type_name, mem::transmute};

    use super::get_memory_index_of_enum_discriminant;

    #[test]
    fn asdf() {
        println!("size = {:?}", unsafe {
            transmute::<_, [u8; 2]>(Option::<Result<bool, bool>>::Some(Err(false)))
        });
        type T = Option<Option<Option<bool>>>;
        println!("opt = {}", unsafe {
            transmute::<_, u8>(Some(Some(Option::<bool>::None)).unwrap())
        });
        println!("size = {}", size_of::<T>());
        test(Some(Some(Some(true))));
        test(Some(Some(Option::<bool>::None)));
        test(Result::<bool, bool>::Ok(true));
    }

    fn test<T>(value: T)
    where
        [(); size_of::<T>()]:,
    {
        println!("{}", type_name::<T>());
        get_memory_index_of_enum_discriminant::<T>(&value);
    }
}
