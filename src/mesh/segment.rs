use std::mem::transmute;

use crate::{CompositeDecoder, CompositeEncoder, Decode, Encode};

use super::{
    edge::PhantomEdge,
    end::End,
    field::Field,
    len::{Len, Size},
    mesh::Mesh,
    pad::ConstPadding,
    prelude::{Vector, Vectored},
};

pub trait SegmentHandler<C> {
    type Error;

    fn handle_element<T: Encode + Decode>(element: &T, codec: &mut C) -> Result<(), Self::Error>;
    fn handle_cluster<const N: usize>(cluster: &[u8; N], codec: &mut C);
    fn handle_clusters<const N: usize>(clsuters: &[[u8; N]], codec: &mut C);
}

pub struct SegmentEncoder;

impl<C> SegmentHandler<C> for SegmentEncoder
where
    C: CompositeEncoder,
{
    type Error = C::Error;

    fn handle_element<T: Encode>(element: &T, codec: &mut C) -> Result<(), Self::Error> {
        codec.encode_element(element)
    }

    fn handle_clusters<const N: usize>(clusters: &[[u8; N]], codec: &mut C) {
        codec
            .write_slice(unsafe { core::slice::from_raw_parts(clusters.as_ptr(), clusters.len()) });
    }

    fn handle_cluster<const N: usize>(cluster: &[u8; N], codec: &mut C) {
        codec.write_array(cluster);
    }
}

pub trait SegmentWalker<S, C, H>
where
    H: SegmentHandler<C>,
{
    // try change src type to raw pointer
    fn walk(src: &S, codec: &mut C, skip_acc: usize) -> Result<(), H::Error>;
}

impl<S, A, B, C, H> SegmentWalker<S, C, H> for PhantomEdge<S, (Field<A>, B)>
where
    //TODO try remove something that you can
    Self: Len,
    H: SegmentHandler<C>,
    A: Encode + Decode + Size,
    B: SegmentWalker<S, C, H>,
    [(); <Self as Len>::SIZE]:,
{
    fn walk(mut src: &S, codec: &mut C, mut skip_acc: usize) -> Result<(), H::Error> {
        if skip_acc == 0 {
            skip_acc = <Self as Len>::SIZE;
            if <Self as Len>::SIZE == 0 {
                let segment = unsafe { transmute(src) };
                H::handle_element::<A>(segment, codec)?;
                // unsafe { codec.encode_element::<A>(transmute(src))? };
                src = unsafe { &*(src as *const S).wrapping_byte_add(<A as Size>::SIZE) };
            } else {
                let segment = unsafe { transmute::<_, &[u8; { <Self as Len>::SIZE }]>(src) };
                H::handle_cluster::<{ <Self as Len>::SIZE }>(segment, codec);
                //unsafe { codec.write_array::<u8, { <Self as Len>::SIZE }>(transmute(src)) };
                src = unsafe { &*(src as *const S).wrapping_byte_add(<Self as Len>::SIZE) };
            }
        } else {
            skip_acc -= <A as Size>::SIZE;
        }
        B::walk(src, codec, skip_acc)
    }
}

impl<S, C, H, B, T, V> SegmentWalker<S, C, H> for PhantomEdge<S, (Vectored<T, V>, B)>
where
    H: SegmentHandler<C>,
    B: SegmentWalker<S, C, H>,
    T: Vector<Item: Size + Mesh<C, Output: SegmentWalker<<T as Vector>::Item, C, H> + Len>> + Size,
    [(); <<T as Vector>::Item as Size>::SIZE]:,
{
    fn walk(mut src: &S, codec: &mut C, _skip_acc: usize) -> Result<(), H::Error> {
        let skip_acc = 0;
        let vector: &T = unsafe { transmute(src) };
        let clustered_len = <<<T as Vector>::Item as Mesh<C>>::Output as Len>::SIZE;
        let element_len = <<T as Vector>::Item as Size>::SIZE;
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
                <<<T as Vector>::Item as Mesh<C>>::Output as SegmentWalker<
                    <T as Vector>::Item,
                    C,
                    H,
                >>::walk(elem, codec, skip_acc)?;
            }
        }
        src = unsafe { &*(src as *const S).wrapping_byte_add(<T as Size>::SIZE) };
        B::walk(src, codec, skip_acc)
    }
}

impl<S, S2, H, C, B, const I: usize> SegmentWalker<S, C, H>
    for PhantomEdge<S, (ConstPadding<S2, I>, B)>
where
    H: SegmentHandler<C>,
    B: SegmentWalker<S, C, H>,
{
    fn walk(mut src: &S, codec: &mut C, mut skip_acc: usize) -> Result<(), H::Error> {
        if skip_acc != 0 && I != 0 {
            skip_acc = 0;
        }
        src = unsafe { &*(src as *const S).wrapping_byte_add(I) };
        B::walk(src, codec, skip_acc)
    }
}

impl<S, S2, C, H> SegmentWalker<S, C, H> for End<S2>
where
    H: SegmentHandler<C>,
{
    fn walk(_src: &S, _codec: &mut C, _skip_acc: usize) -> Result<(), H::Error> {
        Ok(())
    }
}
