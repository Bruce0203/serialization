use std::{
    marker::PhantomData,
    mem::{discriminant, transmute, transmute_copy, Discriminant, MaybeUninit},
};

use crate::{
    Codec, CompositeDecoder, CompositeEncoder, Decode, DecodeError, Encode, EncodeError,
    EnumVariantIndex,
};

use super::{
    edge::{Edge, PhantomEdge},
    end::End,
    field::Field,
    flatten::Flatten,
    len::{Len, Size},
    padding::{ConstPadding, ConstifyPadding},
    prelude::{Instantiate, Vector, Vectored},
    r#enum::{Enum, Variant},
    sort::Sorted,
};

pub trait Mesh<C, H>: Sized
where
    H: SegmentCodec<C>,
{
    type Output: SegmentWalker<C, H>;
}

impl<T, C, H> Mesh<C, H> for T
where
    T: Edge<
        C,
        Second: Sorted<Output: ConstifyPadding<Output: Flatten<T, Output: SegmentWalker<C, H>>>>,
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
    fn get_variant_index<T>(src: &T, codec: &mut C) -> Result<EnumVariantIndex, Self::Error>
    where
        //TODO consider removing this
        for<'a> &'a T: Into<EnumVariantIndex>,
        [(); size_of::<Discriminant<T>>()]:;
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

    fn get_variant_index<T>(src: &T, codec: &mut C) -> Result<EnumVariantIndex, Self::Error>
    where
        for<'a> &'a T: Into<EnumVariantIndex>,
        [(); size_of::<Discriminant<T>>()]:,
    {
        let id = src.into();
        codec.encode_enum_identifier::<T>(&id)?;
        Ok(id)
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

    fn get_variant_index<T>(src: &T, codec: &mut C) -> Result<EnumVariantIndex, Self::Error>
    where
        for<'a> &'a T: Into<EnumVariantIndex>,
        [(); size_of::<Discriminant<T>>()]:,
    {
        codec.decode_enum_variant::<T>()
    }
}

pub trait SegmentWalker<C, H>
where
    Self: Instantiate,
    H: SegmentCodec<C>,
{
    fn walk(src: *mut u8, codec: &mut C, skip_len: Option<usize>) -> Result<(), H::Error>;
}

impl<S, A, B, C, H> SegmentWalker<C, H> for PhantomEdge<C, S, (Field<A>, B)>
where
    Self: Len,
    H: SegmentCodec<C>,
    A: Encode + Decode + Size,
    B: SegmentWalker<C, H>,
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

impl<S2, C, H, B, T> SegmentWalker<C, H> for PhantomEdge<C, S2, (Vectored<T>, B)>
where
    H: SegmentCodec<C>,
    B: SegmentWalker<C, H>,
    T: Vector<Item: Size + Mesh<C, H, Output: SegmentWalker<C, H> + Len>> + Size,
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
                <<<T as Vector>::Item as Mesh<C, H>>::Output as SegmentWalker<C, H>>::walk(
                    elem as *const _ as *mut u8,
                    codec,
                    None,
                )?;
            }
        }
        src = src.wrapping_byte_add(<T as Size>::SIZE);
        B::walk(src, codec, None)
    }
}

impl<S2, C, H, B, T, V> SegmentWalker<C, H> for PhantomEdge<C, S2, (Enum<T, V>, B)>
where
    H: SegmentCodec<C>,
    B: SegmentWalker<C, H>,
    T: Size,
    for<'a> &'a T: Into<EnumVariantIndex>,
    C: Codec,
    V: Edge<C, Second: SegmentWalker<C, H>>,
    [(); size_of::<Discriminant<T>>()]:,
{
    fn walk(mut src: *mut u8, codec: &mut C, skip_len: Option<usize>) -> Result<(), H::Error> {
        H::handle_element(unsafe { transmute::<_, &mut Enum<T, V>>(src) }, codec)?;
        let variant_index = H::get_variant_index::<T>(unsafe { transmute(src) }, codec)?;
        // let variant_index = T::index_by_identifier(id).map_err(|EnumIdentifierToVariantIndexError::InvalidIdentifier|  );
        <<V as Edge<C>>::Second as SegmentWalker<C, H>>::walk(src, codec, Some(variant_index.0))?;
        src = src.wrapping_byte_add(<T as Size>::SIZE);
        B::walk(src, codec, skip_len)
    }
}

impl<S, S2, H, C, B, const I: usize> SegmentWalker<C, H>
    for PhantomEdge<C, S, (ConstPadding<C, S2, I>, B)>
where
    H: SegmentCodec<C>,
    B: SegmentWalker<C, H>,
{
    fn walk(mut src: *mut u8, codec: &mut C, mut skip_len: Option<usize>) -> Result<(), H::Error> {
        //TODO try remove ..
        if I != 0 {
            skip_len = None;
        }
        src = src.wrapping_byte_add(I);
        B::walk(src, codec, skip_len)
    }
}

impl<H, C, S, T, B, const I: usize> SegmentWalker<C, H> for PhantomEdge<C, S, (Variant<T, I>, B)>
where
    C: Codec,
    H: SegmentCodec<C>,
    T: Mesh<C, H, Output: SegmentWalker<C, H>>,
    B: SegmentWalker<C, H>,
{
    fn walk(src: *mut u8, codec: &mut C, mut skip_len: Option<usize>) -> Result<(), H::Error> {
        if let Some(0) = skip_len {
            <<T as Mesh<C, H>>::Output as SegmentWalker<C, H>>::walk(src, codec, skip_len);
            Ok(())
        } else if let Some(skip_len_value) = skip_len {
            skip_len = Some(skip_len_value - 1);
            B::walk(src, codec, skip_len)
        } else {
            Ok(())
        }
    }
}

impl<S2, C, H> SegmentWalker<C, H> for End<C, S2>
where
    H: SegmentCodec<C>,
{
    fn walk(_src: *mut u8, _codec: &mut C, _skip_len: Option<usize>) -> Result<(), H::Error> {
        Ok(())
    }
}
