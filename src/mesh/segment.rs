use std::{marker::PhantomData, mem::transmute};

use crate::{CompositeDecoder, CompositeEncoder, Decode, Encode};

use super::{
    edge::{Edge, PhantomEdge},
    end::End,
    field::Field,
    flatten::Flatten,
    len::{Len, Size},
    pad::{ConstPadding, ConstifyPadding},
    prelude::{Vector, Vectored},
    sort::Sorted,
};

pub trait Mesh<C, H>: Sized
where
    H: SegmentCodec<C>,
{
    type Output: SegmentWalker<Self, C, H>;
}

impl<T, C, H> Mesh<C, H> for T
where
    T: Edge<
        Second: Sorted<Output: ConstifyPadding<Output: Flatten<T, Output: SegmentWalker<T, C, H>>>>,
    >,
    H: SegmentCodec<C>,
{
    type Output = <<<<T as Edge>::Second as Sorted>::Output as ConstifyPadding>::Output as Flatten<T>>::Output;
}

pub fn encode<T, C>(src: &T, codec: &mut C) -> Result<(), C::Error>
where
    T: Mesh<C, SegmentEncoder>,
    C: CompositeEncoder,
{
    walk_segment::<T, C, SegmentEncoder>(src, codec)?;
    Ok(())
}

//TODO try remove inline never
#[inline(never)]
pub fn walk_segment<T, C, H>(src: &T, codec: &mut C) -> Result<(), H::Error>
where
    H: SegmentCodec<C>,
    C: CompositeEncoder,
    T: Mesh<C, H>,
{
    <T as Mesh<C, H>>::Output::walk(src, codec, None)
}

pub trait SegmentCodec<C> {
    type Error;

    fn handle_element<T: Encode + Decode>(element: &T, codec: &mut C) -> Result<(), Self::Error>;
    fn handle_cluster<const N: usize>(cluster: &[u8; N], codec: &mut C);
    fn handle_clusters<const N: usize>(clsuters: &[[u8; N]], codec: &mut C);
}

pub struct SegmentEncoder;

impl<C> SegmentCodec<C> for SegmentEncoder
where
    C: CompositeEncoder,
{
    type Error = C::Error;

    fn handle_element<T: Encode>(element: &T, codec: &mut C) -> Result<(), Self::Error> {
        codec.encode_element(element)
    }

    fn handle_clusters<const N: usize>(clusters: &[[u8; N]], codec: &mut C) {
        codec.write_slice(clusters);
    }

    fn handle_cluster<const N: usize>(cluster: &[u8; N], codec: &mut C) {
        codec.write_array(cluster);
    }
}

pub struct SegmentDecoder;

impl<C> SegmentCodec<C> for SegmentDecoder
where
    C: CompositeDecoder,
{
    type Error = C::Error;

    fn handle_element<T: Encode + Decode>(element: &T, codec: &mut C) -> Result<(), Self::Error> {
        todo!()
    }

    fn handle_cluster<const N: usize>(cluster: &[u8; N], codec: &mut C) {
        todo!()
    }

    fn handle_clusters<const N: usize>(clsuters: &[[u8; N]], codec: &mut C) {
        todo!()
    }
}

pub trait SegmentWalker<S, C, H>
where
    H: SegmentCodec<C>,
{
    // try change src type to raw pointer
    fn walk(src: &S, codec: &mut C, skip_len: Option<usize>) -> Result<(), H::Error>;
}

impl<S, A, B, C, H> SegmentWalker<S, C, H> for PhantomEdge<S, (Field<A>, B)>
where
    Self: Len,
    H: SegmentCodec<C>,
    A: Encode + Decode + Size,
    B: SegmentWalker<S, C, H>,
    [(); <Self as Len>::SIZE]:,
{
    fn walk(mut src: &S, codec: &mut C, mut skip_len: Option<usize>) -> Result<(), H::Error> {
        if let Some(len) = skip_len {
            skip_len = Some(len - <A as Size>::SIZE);
        } else {
            skip_len = Some(<Self as Len>::SIZE);
            if <Self as Len>::SIZE == 0 {
                let segment = unsafe { transmute(src) };
                H::handle_element::<A>(segment, codec)?;
                src = unsafe { &*(src as *const S).wrapping_byte_add(<A as Size>::SIZE) };
            } else {
                let segment = unsafe { transmute::<_, &[u8; <Self as Len>::SIZE]>(src) };
                H::handle_cluster::<{ <Self as Len>::SIZE }>(segment, codec);
                src = unsafe { &*(src as *const S).wrapping_byte_add(<Self as Len>::SIZE) };
            }
        }
        B::walk(src, codec, skip_len)
    }
}

impl<S, S2, C, H, B, T> SegmentWalker<S, C, H> for PhantomEdge<S2, (Vectored<T>, B)>
where
    H: SegmentCodec<C>,
    B: SegmentWalker<S, C, H>,
    T: Vector<Item: Size + Mesh<C, H, Output: SegmentWalker<<T as Vector>::Item, C, H> + Len>>
        + Size,
    [(); <<T as Vector>::Item as Size>::SIZE]:,
    Vectored<T>: Decode,
{
    fn walk(mut src: &S, codec: &mut C, _skip_len: Option<usize>) -> Result<(), H::Error> {
        let vector: &T = unsafe { transmute(src) };
        let clustered_len = <<<T as Vector>::Item as Mesh<C, H>>::Output as Len>::SIZE;
        let element_len = <<T as Vector>::Item as Size>::SIZE;
        H::handle_element(unsafe { transmute::<_, &Vectored<T>>(src) }, codec)?;
        if clustered_len == element_len {
            let segment = unsafe {
                core::slice::from_raw_parts(
                    vector.as_ptr() as *const [u8; <<T as Vector>::Item as Size>::SIZE],
                    vector.len(),
                )
            };
            H::handle_clusters(segment, codec);
        } else {
            let iter = vector.as_iter();
            for elem in iter {
                <<<T as Vector>::Item as Mesh<C, H>>::Output as SegmentWalker<
                    <T as Vector>::Item,
                    C,
                    H,
                >>::walk(elem, codec, None)?;
            }
        }
        src = unsafe { &*(src as *const S).wrapping_byte_add(<T as Size>::SIZE) };
        B::walk(src, codec, None)
    }
}

impl<S, S2, H, C, B, const I: usize> SegmentWalker<S, C, H>
    for PhantomEdge<S, (ConstPadding<S2, I>, B)>
where
    H: SegmentCodec<C>,
    B: SegmentWalker<S, C, H>,
{
    fn walk(mut src: &S, codec: &mut C, mut skip_len: Option<usize>) -> Result<(), H::Error> {
        if I != 0 {
            skip_len = None;
        }
        src = unsafe { &*(src as *const S).wrapping_byte_add(I) };
        B::walk(src, codec, skip_len)
    }
}

impl<S, S2, C, H> SegmentWalker<S, C, H> for End<S2>
where
    H: SegmentCodec<C>,
{
    fn walk(_src: &S, _codec: &mut C, _skip_len: Option<usize>) -> Result<(), H::Error> {
        Ok(())
    }
}
