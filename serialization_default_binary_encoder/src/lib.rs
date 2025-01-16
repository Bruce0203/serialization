use serialization::__private::{Actor, Ctx, Edge, Flatten, Sorted};

mod coder;
pub use coder::*;

pub fn encode<'a, T>(src: &T, dst: &mut [u8])
where
    T: Serializable<Coder<*mut u8>, Output: Actor<T, Coder<*mut u8>>>,
{
    let coder = Coder(dst.as_mut_ptr());
    encode_with_encoder(src, coder);
}

fn encode_with_encoder<T, C>(src: &T, enc: C)
where
    T: Serializable<C, Output: Actor<T, C>>,
{
    let mut ctx = Ctx::Encode { src, coder: enc };
    for i in 0..100 {
        T::Output::run_at(&mut ctx, i);
    }
}
pub trait Serializable<C> {
    type Output;
}

impl<T, C> Serializable<C> for T
where
    T: Edge<Second: Sorted<Output: Flatten<T, Output: Actor<T, C>>>>,
{
    type Output = <<<T as Edge>::Second as Sorted>::Output as Flatten<T>>::Output;
}
