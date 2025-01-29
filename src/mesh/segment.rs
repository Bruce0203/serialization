use std::mem::{discriminant, transmute, MaybeUninit};

use crate::{Codec, CompositeDecoder, CompositeEncoder, Decode, Encode};

use super::{
    edge::{Edge, PhantomEdge},
    end::End,
    field::Field,
    flatten::Flatten,
    len::{Len, Size},
    padding::{ConstPadding, ConstifyPadding},
    prelude::{Vector, Vectored},
    r#enum::Enum,
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
        C,
        Second: Sorted<Output: ConstifyPadding<Output: Flatten<T, Output: SegmentWalker<T, C, H>>>>,
    >,
    H: SegmentCodec<C>,
{
    type Output = <<<<T as Edge<C>>::Second as Sorted>::Output as ConstifyPadding>::Output as Flatten<T>>::Output;
}

#[inline(never)]
pub fn walk_segment<T, C, H>(src: *const T, codec: &mut C) -> Result<(), H::Error>
where
    H: SegmentCodec<C>,
    T: Mesh<C, H>,
{
    <T as Mesh<C, H>>::Output::walk(src as *const _ as *mut u8, codec, None)
}

pub trait SegmentCodec<C> {
    type Error;

    fn handle_element<T: Encode + Decode>(
        element: &mut T,
        codec: &mut C,
    ) -> Result<(), Self::Error>;
    fn handle_cluster<const N: usize>(cluster: &mut [u8; N], codec: &mut C);
    fn handle_clusters<const N: usize>(clusters: &mut [[u8; N]], codec: &mut C);
}

pub struct SegmentEncoder;

impl<C> SegmentCodec<C> for SegmentEncoder
where
    C: CompositeEncoder,
{
    type Error = C::Error;

    fn handle_element<T: Encode>(element: &mut T, codec: &mut C) -> Result<(), Self::Error> {
        codec.encode_element(element)
    }

    fn handle_clusters<const N: usize>(clusters: &mut [[u8; N]], codec: &mut C) {
        codec.write_slice(clusters);
    }

    fn handle_cluster<const N: usize>(cluster: &mut [u8; N], codec: &mut C) {
        codec.write_array::<u8, N>(cluster);
    }
}

pub struct SegmentDecoder;

impl<C> SegmentCodec<C> for SegmentDecoder
where
    C: CompositeDecoder,
{
    type Error = C::Error;

    fn handle_element<T: Encode + Decode>(
        element: &mut T,
        codec: &mut C,
    ) -> Result<(), Self::Error> {
        C::decode_element(codec, unsafe {
            transmute::<_, &mut MaybeUninit<T>>(element)
        })
    }

    fn handle_cluster<const N: usize>(cluster: &mut [u8; N], codec: &mut C) {
        codec.read_array(unsafe { transmute::<_, &mut MaybeUninit<[u8; N]>>(cluster) })
    }

    fn handle_clusters<const N: usize>(clusters: &mut [[u8; N]], codec: &mut C) {
        codec.read_slice(unsafe { transmute::<_, &mut [MaybeUninit<[u8; N]>]>(clusters) });
    }
}

pub trait SegmentWalker<S, C, H>
where
    H: SegmentCodec<C>,
{
    fn walk(src: *mut u8, codec: &mut C, skip_len: Option<usize>) -> Result<(), H::Error>;
}

impl<S, A, B, C, H> SegmentWalker<S, C, H> for PhantomEdge<C, S, (Field<A>, B)>
where
    Self: Len,
    H: SegmentCodec<C>,
    A: Encode + Decode + Size,
    B: SegmentWalker<S, C, H>,
    [(); <Self as Len>::SIZE]:,
{
    fn walk(mut src: *mut u8, codec: &mut C, mut skip_len: Option<usize>) -> Result<(), H::Error> {
        if let Some(len) = skip_len {
            skip_len = Some(len - <A as Size>::SIZE);
        } else {
            skip_len = Some(<Self as Len>::SIZE);
            if <Self as Len>::SIZE == 0 {
                let segment = unsafe { transmute(src) };
                H::handle_element::<A>(segment, codec)?;
                src = src.wrapping_byte_add(<A as Size>::SIZE);
            } else {
                let segment = unsafe { transmute::<_, &mut [u8; <Self as Len>::SIZE]>(src) };
                H::handle_cluster::<{ <Self as Len>::SIZE }>(segment, codec);
                src = src.wrapping_byte_add(<Self as Len>::SIZE);
            }
        }
        B::walk(src, codec, skip_len)
    }
}

impl<S, S2, C, H, B, T> SegmentWalker<S, C, H> for PhantomEdge<C, S2, (Vectored<T>, B)>
where
    H: SegmentCodec<C>,
    B: SegmentWalker<S, C, H>,
    T: Vector<Item: Size + Mesh<C, H, Output: SegmentWalker<<T as Vector>::Item, C, H> + Len>>
        + Size,
    [(); <<T as Vector>::Item as Size>::SIZE]:,
    Vectored<T>: Decode,
{
    fn walk(mut src: *mut u8, codec: &mut C, _skip_len: Option<usize>) -> Result<(), H::Error> {
        H::handle_element(unsafe { transmute::<_, &mut Vectored<T>>(src) }, codec)?;
        let vector: &T = unsafe { transmute(src) };
        let clustered_len = <<<T as Vector>::Item as Mesh<C, H>>::Output as Len>::SIZE;
        let element_len = <<T as Vector>::Item as Size>::SIZE;
        if clustered_len == element_len {
            let segment = unsafe {
                core::slice::from_raw_parts_mut(
                    vector.as_ptr() as *mut [u8; <<T as Vector>::Item as Size>::SIZE],
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
                >>::walk(elem as *const _ as *mut u8, codec, None)?;
            }
        }
        src = src.wrapping_byte_add(<T as Size>::SIZE);
        B::walk(src, codec, None)
    }
}

impl<S, S2, C, H, B, T> SegmentWalker<S, C, H> for PhantomEdge<C, S2, (Enum<T>, B)>
where
    H: SegmentCodec<C>,
    B: SegmentWalker<S, C, H>,
    T: Size,
    C: Codec,
{
    fn walk(mut src: *mut u8, codec: &mut C, _skip_len: Option<usize>) -> Result<(), H::Error> {
        H::handle_element(unsafe { transmute::<_, &mut Enum<T>>(src) }, codec)?;
        let enum_value: MaybeUninit<T> = MaybeUninit::uninit();
        //let v = <T as EnumMatcher>::edge_of::<C, H>(unsafe { enum_value.assume_init_ref() });
        //stopship: enum discriminant를 decoding하는 함수를 만들어라
        //discriminant size를 그대로 decode/encode하라
        //decode [u8; size_of::<Discriminant<T>>()]을 디코딩하고
        //discriminant를 입력값으로 받고 VariantToken을 반환하라
        //discriminant matching한 함수의 반환값을 함수 포인터로 하는 것을 고려하라
        //또는 type Output을 associated type으로 가지고 있는 trait을 impl Type형태로 반환하고
        // <>::walk(elem as *const _ as *mut u8, codec, None)?;
        src = src.wrapping_byte_add(<T as Size>::SIZE);
        B::walk(src, codec, None)
    }
}

impl<S, S2, H, C, B, const I: usize> SegmentWalker<S, C, H>
    for PhantomEdge<C, S, (ConstPadding<C, S2, I>, B)>
where
    H: SegmentCodec<C>,
    B: SegmentWalker<S, C, H>,
{
    fn walk(mut src: *mut u8, codec: &mut C, mut skip_len: Option<usize>) -> Result<(), H::Error> {
        if I != 0 {
            skip_len = None;
        }
        src = src.wrapping_byte_add(I);
        B::walk(src, codec, skip_len)
    }
}

impl<S, S2, C, H> SegmentWalker<S, C, H> for End<C, S2>
where
    H: SegmentCodec<C>,
{
    fn walk(_src: *mut u8, _codec: &mut C, _skip_len: Option<usize>) -> Result<(), H::Error> {
        Ok(())
    }
}
