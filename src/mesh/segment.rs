use std::{
    any::type_name,
    marker::PhantomData,
    mem::{discriminant, transmute, transmute_copy, Discriminant, MaybeUninit},
    ptr::drop_in_place,
};

use typenum::{ToUInt, Unsigned};

use crate::{
    BufReadError, BufWriteError, Codec, CompositeDecoder, CompositeEncoder, Decode, DecodeError,
    Encode, EncodeError, EnumIdentifierToVariantIndex, EnumVariantDiscriminantId, EnumVariantIndex,
    EnumVariantStringId,
};

use super::{
    edge::{Edge, PhantomEdge},
    end::End,
    field::Field,
    flatten::Flatten,
    len::{Len, Size},
    padding::{ConstPadding, ConstifyPadding},
    prelude::{Instantiate, Vector, Vectored},
    r#enum::{Enum, EnumDiscriminantDecoder, Variant},
    sort::Sorted,
};

pub trait Mesh<C, H>: Sized
where
    H: SegmentCodec<C, Self>,
{
    type Output: SegmentWalker<C, H>;
}

impl<T, C, H> Mesh<C, H> for T
where
    T: Edge<
        C,
        Second: Sorted<Output: ConstifyPadding<Output: Flatten<T, Output: SegmentWalker<C, H>>>>,
    >,
    H: SegmentCodec<C, Self>,
{
    type Output = <<<<T as Edge<C>>::Second as Sorted>::Output as ConstifyPadding>::Output as Flatten<T>>::Output;
}

#[inline(never)]
pub fn walk_segment<T, C, H>(src: *const T, codec: &mut C) -> Result<(), H::Error>
where
    H: SegmentCodec<C, T>,
    T: Mesh<C, H>,
{
    <T as Mesh<C, H>>::Output::walk(src as *const _ as *mut u8, codec, None)
}

pub trait SegemntErrorKind<C> {
    type Error;
}

pub trait SegmentCodec<C, T>: SegemntErrorKind<C> {
    fn handle_element(element: &mut T, codec: &mut C) -> Result<(), Self::Error>;
    fn handle_cluster<const N: usize>(
        cluster: &mut [u8; N],
        codec: &mut C,
    ) -> Result<(), Self::Error>;
    fn handle_clusters<const N: usize>(clusters: &mut [[u8; N]], codec: &mut C);
    fn get_variant_index(src: &mut T, codec: &mut C) -> Result<EnumVariantIndex, Self::Error>
    where
        T: EnumDiscriminantDecoder<T>
            + EnumIdentifierToVariantIndex<EnumVariantStringId>
            + EnumIdentifierToVariantIndex<EnumVariantDiscriminantId<T>>,
        for<'a> &'a T: Into<EnumVariantStringId> + Into<EnumVariantDiscriminantId<T>>,
        [(); size_of::<Discriminant<T>>()]:;
}

impl<C> SegemntErrorKind<C> for SegmentEncoder
where
    C: CompositeEncoder,
{
    type Error = C::Error;
}

impl<C> SegemntErrorKind<C> for SegmentDecoder
where
    C: CompositeDecoder,
{
    type Error = C::Error;
}

pub struct SegmentEncoder;

impl<C, T> SegmentCodec<C, T> for SegmentEncoder
where
    C: CompositeEncoder,
    T: Encode,
{
    fn handle_element(
        element: &mut T,
        codec: &mut C,
    ) -> Result<(), <Self as SegemntErrorKind<C>>::Error> {
        codec.encode_element(element)
    }

    fn handle_clusters<const N: usize>(clusters: &mut [[u8; N]], codec: &mut C) {
        codec.write_slice(clusters);
    }

    fn handle_cluster<const N: usize>(
        cluster: &mut [u8; N],
        codec: &mut C,
    ) -> Result<(), Self::Error> {
        codec
            .write_array::<u8, N>(cluster)
            .map_err(|BufWriteError::EOF| EncodeError::not_enough_space_in_the_buffer())
    }

    fn get_variant_index(src: &mut T, codec: &mut C) -> Result<EnumVariantIndex, Self::Error>
    where
        T: EnumDiscriminantDecoder<T>
            + EnumIdentifierToVariantIndex<EnumVariantStringId>
            + EnumIdentifierToVariantIndex<EnumVariantDiscriminantId<T>>,
        for<'a> &'a T: Into<EnumVariantStringId> + Into<EnumVariantDiscriminantId<T>>,
        [(); size_of::<Discriminant<T>>()]:,
    {
        codec.encode_enum_identifier::<T>(src)
    }
}

pub struct SegmentDecoder;

impl<C, T> SegmentCodec<C, T> for SegmentDecoder
where
    C: CompositeDecoder,
    T: Decode,
{
    fn handle_element(element: &mut T, codec: &mut C) -> Result<(), Self::Error> {
        C::decode_element(codec, unsafe {
            transmute::<_, &mut MaybeUninit<T>>(element)
        })
    }

    fn handle_cluster<const N: usize>(
        cluster: &mut [u8; N],
        codec: &mut C,
    ) -> Result<(), Self::Error> {
        codec
            .read_array(unsafe { transmute::<_, &mut MaybeUninit<[u8; N]>>(cluster) })
            .map_err(|BufReadError::EOF| DecodeError::not_enough_bytes_in_the_buffer())
    }

    fn handle_clusters<const N: usize>(clusters: &mut [[u8; N]], codec: &mut C) {
        codec.read_slice(unsafe { transmute::<_, &mut [MaybeUninit<[u8; N]>]>(clusters) });
    }

    fn get_variant_index(src: &mut T, codec: &mut C) -> Result<EnumVariantIndex, Self::Error>
    where
        T: EnumDiscriminantDecoder<T>
            + EnumIdentifierToVariantIndex<EnumVariantStringId>
            + EnumIdentifierToVariantIndex<EnumVariantDiscriminantId<T>>,
        for<'a> &'a T: Into<EnumVariantStringId> + Into<EnumVariantDiscriminantId<T>>,
        [(); size_of::<Discriminant<T>>()]:,
    {
        let src: &mut MaybeUninit<T> = unsafe { transmute(src) };
        let variant_index = codec.decode_enum_identifier::<T>(src)?;
        T::decode_enum_discriminant(&variant_index, src);
        Ok(variant_index)
    }
}

pub trait SegmentWalker<C, H>
where
    Self: Instantiate,
    H: SegemntErrorKind<C>,
{
    fn walk(src: *mut u8, codec: &mut C, skip_len: Option<usize>) -> Result<(), H::Error>;
}

const fn adjust_to_word(value: usize) -> usize {
    //TODO support 32bit system
    if value <= 2 {
        value
    } else if value <= 4 {
        4
    } else if value <= 8 {
        8
    } else if value <= 16 {
        16
    } else {
        value
    }
}

impl<S, A, B, C, H> SegmentWalker<C, H> for PhantomEdge<C, S, (Field<A>, B)>
where
    Self: Len,
    H: SegmentCodec<C, A>,
    A: Encode + Decode + Size,
    B: SegmentWalker<C, H>,
    [(); adjust_to_word(<Self as Len>::SIZE)]:,
{
    fn walk(mut src: *mut u8, codec: &mut C, mut skip_len: Option<usize>) -> Result<(), H::Error> {
        let origin_src = src;
        if let Some(len) = skip_len {
            skip_len = Some(len - <A as Size>::SIZE);
        } else {
            skip_len = Some(<Self as Len>::SIZE);
            if <Self as Len>::SIZE == 0 {
                let segment = unsafe { transmute(origin_src) };
                H::handle_element(segment, codec)?;
                src = origin_src.wrapping_byte_add(<A as Size>::SIZE);
            } else {
                let segment = unsafe {
                    transmute::<_, &mut [u8; adjust_to_word(<Self as Len>::SIZE)]>(origin_src)
                };
                H::handle_cluster::<{ adjust_to_word(<Self as Len>::SIZE) }>(segment, codec);
                src = origin_src.wrapping_byte_add(<Self as Len>::SIZE);
            }
        }
        match B::walk(src, codec, skip_len) {
            Ok(()) => Ok(()),
            Err(err) => {
                unsafe { drop_in_place::<A>(transmute(origin_src)) };
                return Err(err);
            }
        }
    }
}

impl<S, S2, H, C, B, N> SegmentWalker<C, H> for PhantomEdge<C, S, (ConstPadding<C, S2, N>, B)>
where
    H: SegmentCodec<C, ()>,
    B: SegmentWalker<C, H>,
    N: ToUInt<Output: Unsigned>,
{
    fn walk(mut src: *mut u8, codec: &mut C, mut skip_len: Option<usize>) -> Result<(), H::Error> {
        //TODO try remove ..
        if <<N as ToUInt>::Output>::USIZE != 0 {
            skip_len = None;
        }
        src = src.wrapping_byte_add(<<N as ToUInt>::Output>::USIZE);
        B::walk(src, codec, skip_len)
    }
}

impl<S2, C, H, B, T> SegmentWalker<C, H> for PhantomEdge<C, S2, (Vectored<T>, B)>
where
    H: SegmentCodec<C, T> + SegmentCodec<C, Vectored<T>> + SegmentCodec<C, <T as Vector>::Item>,
    B: SegmentWalker<C, H>,
    T: Vector<Item: Size + Mesh<C, H, Output: SegmentWalker<C, H> + Len>> + Size,
    [(); <<T as Vector>::Item as Size>::SIZE]:,
{
    fn walk(mut src: *mut u8, codec: &mut C, _skip_len: Option<usize>) -> Result<(), H::Error> {
        let origin_src = src;
        H::handle_element(
            unsafe { transmute::<_, &mut Vectored<T>>(origin_src) },
            codec,
        )?;
        let vector: &mut T = unsafe { transmute(origin_src) };
        let clustered_len = <<<T as Vector>::Item as Mesh<C, H>>::Output as Len>::SIZE;
        let element_len = <<T as Vector>::Item as Size>::SIZE;
        if clustered_len == element_len {
            let segment = unsafe {
                core::slice::from_raw_parts_mut(
                    vector.as_ptr() as *mut [u8; <<T as Vector>::Item as Size>::SIZE],
                    vector.len(),
                )
            };
            <H as SegmentCodec<C, Vectored<T>>>::handle_clusters(segment, codec);
        } else {
            let mut vec_ptr = vector.as_mut_ptr();
            let end = vec_ptr.wrapping_add(vector.len());
            loop {
                if vec_ptr == end {
                    break;
                }

                let elem = vec_ptr;
                match <<<T as Vector>::Item as Mesh<C, H>>::Output as SegmentWalker<C, H>>::walk(
                    elem as *const _ as *mut u8,
                    codec,
                    None,
                ) {
                    Ok(()) => Ok(()),
                    Err(err) => {
                        let end = vector.as_mut_ptr();
                        loop {
                            if vec_ptr == end {
                                break;
                            }
                            unsafe { drop_in_place((vec_ptr)) }
                            vec_ptr.wrapping_sub(1);
                        }
                        Err(err)
                    }
                }?;
                vec_ptr = vec_ptr.wrapping_add(1);
            }
        }
        //TODO documentize why
        H::handle_element(unsafe { transmute::<_, &mut T>(origin_src) }, codec)?;
        src = origin_src.wrapping_byte_add(<T as Size>::SIZE);
        match B::walk(src, codec, None) {
            Ok(()) => Ok(()),
            Err(err) => {
                unsafe {
                    drop_in_place::<T>(transmute(origin_src));
                };
                return Err(err);
            }
        }
    }
}

impl<S2, C, H, B, T, V> SegmentWalker<C, H> for PhantomEdge<C, S2, (Enum<T, V>, B)>
where
    H: SegmentCodec<C, T>,
    B: SegmentWalker<C, H>,
    T: Size,
    T: EnumDiscriminantDecoder<T>
        + EnumIdentifierToVariantIndex<EnumVariantStringId>
        + EnumIdentifierToVariantIndex<EnumVariantDiscriminantId<T>>,
    for<'a> &'a T: Into<EnumVariantStringId> + Into<EnumVariantDiscriminantId<T>>,
    C: Codec,
    V: Edge<C, Second: ConstifyPadding<Output: SegmentWalker<C, H>>>,
    [(); size_of::<Discriminant<T>>()]:,
{
    fn walk(mut src: *mut u8, codec: &mut C, skip_len: Option<usize>) -> Result<(), H::Error> {
        let origin_src = src;
        // H::handle_element(unsafe { transmute::<_, &mut Enum<T, V>>(src) }, codec)?;
        let variant_index = H::get_variant_index(unsafe { transmute(origin_src) }, codec)?;
        <<<V as Edge<C>>::Second as ConstifyPadding>::Output as SegmentWalker<C, H>>::walk(
            origin_src,
            codec,
            Some(variant_index.0),
        )?;
        src = origin_src.wrapping_byte_add(<T as Size>::SIZE);
        match B::walk(src, codec, skip_len) {
            Ok(()) => Ok(()),
            Err(err) => {
                unsafe { drop_in_place::<T>(transmute(origin_src)) };
                return Err(err);
            }
        }
    }
}

impl<H, C, S, T, B, const I: usize> SegmentWalker<C, H> for PhantomEdge<C, S, (Variant<T, I>, B)>
where
    C: Codec,
    H: SegmentCodec<C, T>,
    T: Mesh<C, H, Output: SegmentWalker<C, H>>,
    B: SegmentWalker<C, H>,
{
    fn walk(src: *mut u8, codec: &mut C, mut skip_len: Option<usize>) -> Result<(), H::Error> {
        if let Some(0) = skip_len {
            <<T as Mesh<C, H>>::Output as SegmentWalker<C, H>>::walk(src, codec, None)
        } else if let Some(skip_len_value) = skip_len {
            skip_len = Some(skip_len_value - 1);
            B::walk(src, codec, skip_len)
        } else {
            /// skip_len is never None when walking enum variant
            unreachable!()
        }
    }
}

impl<S2, C, H> SegmentWalker<C, H> for End<C, S2>
where
    H: SegmentCodec<C, S2>,
{
    fn walk(_src: *mut u8, _codec: &mut C, _skip_len: Option<usize>) -> Result<(), H::Error> {
        Ok(())
    }
}
