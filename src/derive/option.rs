use std::{hint::unreachable_unchecked, mem::MaybeUninit};

use crate::{Decode, Decoder, Encode, Encoder, __impl_enum_mesh, __impl_enum_variant_mesh};

__impl_enum_mesh!(
    {T},
    (Option), {T}, (None, Some), (0, 1), (0, 1), (unit, parentheses),
    impl {T,} ();
);

__impl_enum_variant_mesh!(
    {T},
    unit,
    (Option), {T}, None, 0,
    impl {T,} ();
);
__impl_enum_variant_mesh!(

    {T},
    parentheses,
    (Option), {T}, Some, 1,
    impl {T,} ();
    v0 => {T}
);

impl<T> Encode for Option<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        unsafe { unreachable_unchecked() }
    }
}

impl<T> Decode for Option<T> {
    fn decode_in_place<D: Decoder>(
        _decoder: &mut D,
        _out: &mut MaybeUninit<Self>,
    ) -> Result<(), D::Error> {
        unsafe { unreachable_unchecked() }
    }
}
