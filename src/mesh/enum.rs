use std::{
    mem::{discriminant, transmute, Discriminant, MaybeUninit},
    num::NonZeroUsize,
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
        let discriminant = discriminant(self);
        Ok(())
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

fn get_discriminant_index_count<T>(value: &T, default: u8) -> usize
where
    T: Sized,
    [(); size_of::<T>()]:,
{
    let punchboard = unsafe { transmute::<&T, &[u8; size_of::<T>()]>(value) };
    punchboard
        .into_iter()
        .fold(0, |acc, byte| if *byte != default { acc + 1 } else { acc })
}

pub(crate) const unsafe fn const_transmute<A, B>(a: A) -> B {
    #[repr(C)]
    union Union<A, B> {
        a: std::mem::ManuallyDrop<A>,
        b: std::mem::ManuallyDrop<B>,
    }

    let a = std::mem::ManuallyDrop::new(a);
    std::mem::ManuallyDrop::into_inner(Union { a }.b)
}

#[cfg(test)]
mod tests {
    use std::{
        any::type_name,
        hint::black_box,
        intrinsics::transmute_unchecked,
        mem::{discriminant, transmute, transmute_copy, MaybeUninit},
        num::NonZeroUsize,
    };

    use crate::prelude::const_transmute;

    use super::get_discriminant_index_count;

    macro_rules! test {
        ($T:ty, $fill:ident, $value:expr) => {{
            let mut punchboard = [false; size_of::<$T>()];
            let mut last_chunk: Option<[u8; size_of::<$T>()]> = None;
            println!("{:?}", type_name::<$T>());
            for default in u8::MIN..=u8::MAX {
                let chunk = unsafe {
                    let $fill = [default; size_of::<$T>()];
                    transmute::<$T, [u8; size_of::<$T>()]>($value)
                };
                //println!("default = {default}, {chunk:?}");
                if let Some(last_chunk) = last_chunk {
                    for i in 0..size_of::<$T>() {
                        if chunk[i] == last_chunk[i] {
                            punchboard[i] = true;
                        }
                    }
                }
                last_chunk = Some(chunk);
            }
            let index_count = punchboard
                .into_iter()
                .fold(0, |acc, v| if v { acc + 1 } else { acc });
            println!("result = {index_count:?}");
            println!("{:?}", punchboard);
            index_count
        }};
    }

    #[test]
    fn asdf() {
        test!(
            Option<Option<Option<bool>>>, // 0
            default,
            Some(const_transmute(default))
        );
        test!(
            Option<Option<Option<bool>>>, // 0
            default,
            None
        );
        test!(
            Result<bool, u32>, // 7
            default,
            Ok(transmute_copy(&default))
        );
        test!(
            Result<&str, usize>, // 0
            default,
            Ok(const_transmute(default))
        );
        test!(
            Result<&str, usize>,
            default,
            Err(const_transmute(default))
        );
        test!(Option<&str>, default, None);
        test!(Option<&str>, default, Some(const_transmute(default)));
        // test!(Result<&str, usize>, default, Ok(const_transmute(default))); // 0
    }

    #[test]
    fn asdf2() {
        type T = Result<bool, u32>;
        let _trashes = [123_u8; 100000];
        black_box(&_trashes);
        drop(_trashes);
        let v: [u8; 100] = unsafe { MaybeUninit::uninit().assume_init() };
        println!("trashes = {:?}", v);
        let v: T = Ok(true);
        let mut v: [u8; size_of::<T>()] = unsafe { transmute(v) };
        println!("boolu8 = {:?}", v);
        v[6] = 124;
        println!("boolu8 = {:?}", v);
        let v: T = unsafe { transmute(v) };
        println!("boolu8 = {:?}", v);
    }
}
