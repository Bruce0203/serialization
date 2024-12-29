#![feature(derive_clone_copy)]
#![feature(derive_eq)]
#![feature(fmt_helpers_for_derive)]
#![feature(coverage_attribute)]
#![feature(core_intrinsics)]
#![feature(structural_match)]
#![feature(panic_internals)]
////////
#![feature(prelude_import)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(specialization)]
#![allow(warnings)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;

struct A1;
impl serialization::__private::Encode for A1 {
    fn encode<__E: serialization::__private::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> core::result::Result<(), __E::Error> {
        let A1 = self;
        {
            serialization::__private::encode_struct(self, encoder)
        }
    }
}
impl serialization::__private::Decode for A1 {
    fn decode_in_place<__D: serialization::__private::Decoder>(
        decoder: &mut __D,
        out: &mut core::mem::MaybeUninit<Self>,
    ) -> Result<(), __D::Error> {
        {
            serialization::__private::decode_struct(decoder, out)
        }
    }
}
impl const serialization::__private::SerialDescriptor for A1 {
    const SIZES_LEN: usize = 0;
    fn serial_sizes<__S: const serialization::__private::PrimitiveTypeSizeChecker>(
    ) -> serialization::__private::fastbuf::Buffer<
        [serialization::__private::SerialSize; { Self::SIZES_LEN }],
    > {
        pub use serialization::__private::fastbuf::WriteBuf;
        serialization::__private::order_sizes_by_repr_and_calc_offset::<
            Self,
            __S,
            { Self::SIZES_LEN },
        >(&[])
    }
}
impl serialization::__private::FieldPathDrop for A1 {
    fn drop_fields(
        value: &mut core::mem::MaybeUninit<Self>,
        fields: serialization::__private::FieldPath,
    ) {
        let fields = fields.as_slice();
        let A1 = unsafe { value.assume_init_mut() };
    }
}
impl const serialization::__private::FieldPathFinder for A1 {
    fn find_encode<'__a, E: serialization::__private::Encoder>(
        mut path: serialization::__private::FieldPath,
    ) -> &'__a dyn serialization::__private::CompositableEncode<E>
    where
        Self: '__a,
    {
        if path.remaining() == 0 {
            return &serialization::__private::CompositableWrapper::<()>(core::marker::PhantomData);
        }
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
    fn find_decode<'__a, D: serialization::__private::Decoder>(
        mut path: serialization::__private::FieldPath,
    ) -> &'__a dyn serialization::__private::CompositableDecode<D>
    where
        Self: '__a,
    {
        if path.remaining() == 0 {
            return &serialization::__private::CompositableWrapper::<()>(core::marker::PhantomData);
        }
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
    fn calc_offset(mut path: serialization::__private::FieldPath) -> usize {
        if path.remaining() == 0 {
            return 0;
        }
        let value = core::mem::MaybeUninit::<Self>::uninit();
        let value = unsafe { value.assume_init_ref() };
        let A1 = value;
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for A1 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "A1")
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for A1 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for A1 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for A1 {
    #[inline]
    fn eq(&self, other: &A1) -> bool {
        true
    }
}
struct A2 {}
impl serialization::__private::Encode for A2 {
    fn encode<__E: serialization::__private::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> core::result::Result<(), __E::Error> {
        let A2 {} = self;
        {
            serialization::__private::encode_struct(self, encoder)
        }
    }
}
impl serialization::__private::Decode for A2 {
    fn decode_in_place<__D: serialization::__private::Decoder>(
        decoder: &mut __D,
        out: &mut core::mem::MaybeUninit<Self>,
    ) -> Result<(), __D::Error> {
        {
            serialization::__private::decode_struct(decoder, out)
        }
    }
}
impl const serialization::__private::SerialDescriptor for A2 {
    const SIZES_LEN: usize = 0;
    fn serial_sizes<__S: const serialization::__private::PrimitiveTypeSizeChecker>(
    ) -> serialization::__private::fastbuf::Buffer<
        [serialization::__private::SerialSize; { Self::SIZES_LEN }],
    > {
        pub use serialization::__private::fastbuf::WriteBuf;
        serialization::__private::order_sizes_by_repr_and_calc_offset::<
            Self,
            __S,
            { Self::SIZES_LEN },
        >(&[])
    }
}
impl serialization::__private::FieldPathDrop for A2 {
    fn drop_fields(
        value: &mut core::mem::MaybeUninit<Self>,
        fields: serialization::__private::FieldPath,
    ) {
        let fields = fields.as_slice();
        let A2 {} = unsafe { value.assume_init_mut() };
    }
}
impl const serialization::__private::FieldPathFinder for A2 {
    fn find_encode<'__a, E: serialization::__private::Encoder>(
        mut path: serialization::__private::FieldPath,
    ) -> &'__a dyn serialization::__private::CompositableEncode<E>
    where
        Self: '__a,
    {
        if path.remaining() == 0 {
            return &serialization::__private::CompositableWrapper::<()>(core::marker::PhantomData);
        }
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
    fn find_decode<'__a, D: serialization::__private::Decoder>(
        mut path: serialization::__private::FieldPath,
    ) -> &'__a dyn serialization::__private::CompositableDecode<D>
    where
        Self: '__a,
    {
        if path.remaining() == 0 {
            return &serialization::__private::CompositableWrapper::<()>(core::marker::PhantomData);
        }
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
    fn calc_offset(mut path: serialization::__private::FieldPath) -> usize {
        if path.remaining() == 0 {
            return 0;
        }
        let value = core::mem::MaybeUninit::<Self>::uninit();
        let value = unsafe { value.assume_init_ref() };
        let A2 {} = value;
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for A2 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "A2")
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for A2 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for A2 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for A2 {
    #[inline]
    fn eq(&self, other: &A2) -> bool {
        true
    }
}
struct A3();
impl serialization::__private::Encode for A3 {
    fn encode<__E: serialization::__private::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> core::result::Result<(), __E::Error> {
        let A3 {} = self;
        {
            serialization::__private::encode_struct(self, encoder)
        }
    }
}
impl serialization::__private::Decode for A3 {
    fn decode_in_place<__D: serialization::__private::Decoder>(
        decoder: &mut __D,
        out: &mut core::mem::MaybeUninit<Self>,
    ) -> Result<(), __D::Error> {
        {
            serialization::__private::decode_struct(decoder, out)
        }
    }
}
impl const serialization::__private::SerialDescriptor for A3 {
    const SIZES_LEN: usize = 0;
    fn serial_sizes<__S: const serialization::__private::PrimitiveTypeSizeChecker>(
    ) -> serialization::__private::fastbuf::Buffer<
        [serialization::__private::SerialSize; { Self::SIZES_LEN }],
    > {
        pub use serialization::__private::fastbuf::WriteBuf;
        serialization::__private::order_sizes_by_repr_and_calc_offset::<
            Self,
            __S,
            { Self::SIZES_LEN },
        >(&[])
    }
}
impl serialization::__private::FieldPathDrop for A3 {
    fn drop_fields(
        value: &mut core::mem::MaybeUninit<Self>,
        fields: serialization::__private::FieldPath,
    ) {
        let fields = fields.as_slice();
        let A3 {} = unsafe { value.assume_init_mut() };
    }
}
impl const serialization::__private::FieldPathFinder for A3 {
    fn find_encode<'__a, E: serialization::__private::Encoder>(
        mut path: serialization::__private::FieldPath,
    ) -> &'__a dyn serialization::__private::CompositableEncode<E>
    where
        Self: '__a,
    {
        if path.remaining() == 0 {
            return &serialization::__private::CompositableWrapper::<()>(core::marker::PhantomData);
        }
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
    fn find_decode<'__a, D: serialization::__private::Decoder>(
        mut path: serialization::__private::FieldPath,
    ) -> &'__a dyn serialization::__private::CompositableDecode<D>
    where
        Self: '__a,
    {
        if path.remaining() == 0 {
            return &serialization::__private::CompositableWrapper::<()>(core::marker::PhantomData);
        }
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
    fn calc_offset(mut path: serialization::__private::FieldPath) -> usize {
        if path.remaining() == 0 {
            return 0;
        }
        let value = core::mem::MaybeUninit::<Self>::uninit();
        let value = unsafe { value.assume_init_ref() };
        let A3 {} = value;
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for A3 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "A3")
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for A3 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for A3 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for A3 {
    #[inline]
    fn eq(&self, other: &A3) -> bool {
        true
    }
}
struct A4(i32);
impl serialization::__private::Encode for A4 {
    fn encode<__E: serialization::__private::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> core::result::Result<(), __E::Error> {
        let A4(f0) = self;
        {
            serialization::__private::encode_struct(self, encoder)
        }
    }
}
impl serialization::__private::Decode for A4 {
    fn decode_in_place<__D: serialization::__private::Decoder>(
        decoder: &mut __D,
        out: &mut core::mem::MaybeUninit<Self>,
    ) -> Result<(), __D::Error> {
        {
            serialization::__private::decode_struct(decoder, out)
        }
    }
}
impl const serialization::__private::SerialDescriptor for A4 {
    const SIZES_LEN: usize = 0 + <i32 as serialization::__private::SerialDescriptor>::SIZES_LEN;
    fn serial_sizes<__S: const serialization::__private::PrimitiveTypeSizeChecker>(
    ) -> serialization::__private::fastbuf::Buffer<
        [serialization::__private::SerialSize; { Self::SIZES_LEN }],
    > {
        pub use serialization::__private::fastbuf::WriteBuf;
        serialization::__private::order_sizes_by_repr_and_calc_offset::<
            Self,
            __S,
            { Self::SIZES_LEN },
        >(&[
            <i32 as serialization::__private::SerialDescriptor>::serial_sizes::<__S>().as_slice(),
        ])
    }
}
impl serialization::__private::FieldPathDrop for A4 {
    fn drop_fields(
        value: &mut core::mem::MaybeUninit<Self>,
        fields: serialization::__private::FieldPath,
    ) {
        let fields = fields.as_slice();
        let A4(f0) = unsafe { value.assume_init_mut() };
        if fields.contains(&(0usize as serialization::__private::FieldIndex)) {
            unsafe {
                (&mut *((f0) as *mut _ as *mut core::mem::MaybeUninit<u32>)).assume_init_drop();
            }
        }
    }
}
impl const serialization::__private::FieldPathFinder for A4 {
    fn find_encode<'__a, E: serialization::__private::Encoder>(
        mut path: serialization::__private::FieldPath,
    ) -> &'__a dyn serialization::__private::CompositableEncode<E>
    where
        Self: '__a,
    {
        if path.remaining() == 0 {
            return &serialization::__private::CompositableWrapper::<()>(core::marker::PhantomData);
        }
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            0usize => <i32>::find_encode(path),
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
    fn find_decode<'__a, D: serialization::__private::Decoder>(
        mut path: serialization::__private::FieldPath,
    ) -> &'__a dyn serialization::__private::CompositableDecode<D>
    where
        Self: '__a,
    {
        if path.remaining() == 0 {
            return &serialization::__private::CompositableWrapper::<()>(core::marker::PhantomData);
        }
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            0usize => <i32>::find_decode(path),
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
    fn calc_offset(mut path: serialization::__private::FieldPath) -> usize {
        if path.remaining() == 0 {
            return 0;
        }
        let value = core::mem::MaybeUninit::<Self>::uninit();
        let value = unsafe { value.assume_init_ref() };
        let A4(f0) = value;
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            0usize => {
                serialization::__private::offset_of(value, f0)
                    + <i32 as serialization::__private::FieldPathFinder>::calc_offset(path)
            }
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for A4 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "A4", &&self.0)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for A4 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<i32>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for A4 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for A4 {
    #[inline]
    fn eq(&self, other: &A4) -> bool {
        self.0 == other.0
    }
}
struct A5 {
    v: i32,
}
impl serialization::__private::Encode for A5 {
    fn encode<__E: serialization::__private::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> core::result::Result<(), __E::Error> {
        let A5 { v } = self;
        {
            serialization::__private::encode_struct(self, encoder)
        }
    }
}
impl serialization::__private::Decode for A5 {
    fn decode_in_place<__D: serialization::__private::Decoder>(
        decoder: &mut __D,
        out: &mut core::mem::MaybeUninit<Self>,
    ) -> Result<(), __D::Error> {
        {
            serialization::__private::decode_struct(decoder, out)
        }
    }
}
impl const serialization::__private::SerialDescriptor for A5 {
    const SIZES_LEN: usize = 0 + <i32 as serialization::__private::SerialDescriptor>::SIZES_LEN;
    fn serial_sizes<__S: const serialization::__private::PrimitiveTypeSizeChecker>(
    ) -> serialization::__private::fastbuf::Buffer<
        [serialization::__private::SerialSize; { Self::SIZES_LEN }],
    > {
        pub use serialization::__private::fastbuf::WriteBuf;
        serialization::__private::order_sizes_by_repr_and_calc_offset::<
            Self,
            __S,
            { Self::SIZES_LEN },
        >(&[
            <i32 as serialization::__private::SerialDescriptor>::serial_sizes::<__S>().as_slice(),
        ])
    }
}
impl serialization::__private::FieldPathDrop for A5 {
    fn drop_fields(
        value: &mut core::mem::MaybeUninit<Self>,
        fields: serialization::__private::FieldPath,
    ) {
        let fields = fields.as_slice();
        let A5 { v } = unsafe { value.assume_init_mut() };
        if fields.contains(&(0usize as serialization::__private::FieldIndex)) {
            unsafe {
                (&mut *((v) as *mut _ as *mut core::mem::MaybeUninit<u32>)).assume_init_drop();
            }
        }
    }
}
impl const serialization::__private::FieldPathFinder for A5 {
    fn find_encode<'__a, E: serialization::__private::Encoder>(
        mut path: serialization::__private::FieldPath,
    ) -> &'__a dyn serialization::__private::CompositableEncode<E>
    where
        Self: '__a,
    {
        if path.remaining() == 0 {
            return &serialization::__private::CompositableWrapper::<()>(core::marker::PhantomData);
        }
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            0usize => <i32>::find_encode(path),
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
    fn find_decode<'__a, D: serialization::__private::Decoder>(
        mut path: serialization::__private::FieldPath,
    ) -> &'__a dyn serialization::__private::CompositableDecode<D>
    where
        Self: '__a,
    {
        if path.remaining() == 0 {
            return &serialization::__private::CompositableWrapper::<()>(core::marker::PhantomData);
        }
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            0usize => <i32>::find_decode(path),
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
    fn calc_offset(mut path: serialization::__private::FieldPath) -> usize {
        if path.remaining() == 0 {
            return 0;
        }
        let value = core::mem::MaybeUninit::<Self>::uninit();
        let value = unsafe { value.assume_init_ref() };
        let A5 { v } = value;
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            0usize => {
                serialization::__private::offset_of(value, v)
                    + <i32 as serialization::__private::FieldPathFinder>::calc_offset(path)
            }
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for A5 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "A5", "v", &&self.v)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for A5 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<i32>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for A5 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for A5 {
    #[inline]
    fn eq(&self, other: &A5) -> bool {
        self.v == other.v
    }
}
struct A6 {
    v1: i32,
    v2: u16,
}
impl serialization::__private::Encode for A6 {
    fn encode<__E: serialization::__private::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> core::result::Result<(), __E::Error> {
        let A6 { v1, v2 } = self;
        {
            serialization::__private::encode_struct(self, encoder)
        }
    }
}
impl serialization::__private::Decode for A6 {
    fn decode_in_place<__D: serialization::__private::Decoder>(
        decoder: &mut __D,
        out: &mut core::mem::MaybeUninit<Self>,
    ) -> Result<(), __D::Error> {
        {
            serialization::__private::decode_struct(decoder, out)
        }
    }
}
impl const serialization::__private::SerialDescriptor for A6 {
    const SIZES_LEN: usize = 0
        + <i32 as serialization::__private::SerialDescriptor>::SIZES_LEN
        + <u16 as serialization::__private::SerialDescriptor>::SIZES_LEN;
    fn serial_sizes<__S: const serialization::__private::PrimitiveTypeSizeChecker>(
    ) -> serialization::__private::fastbuf::Buffer<
        [serialization::__private::SerialSize; { Self::SIZES_LEN }],
    > {
        pub use serialization::__private::fastbuf::WriteBuf;
        serialization::__private::order_sizes_by_repr_and_calc_offset::<
            Self,
            __S,
            { Self::SIZES_LEN },
        >(&[
            <i32 as serialization::__private::SerialDescriptor>::serial_sizes::<__S>().as_slice(),
            <u16 as serialization::__private::SerialDescriptor>::serial_sizes::<__S>().as_slice(),
        ])
    }
}
impl serialization::__private::FieldPathDrop for A6 {
    fn drop_fields(
        value: &mut core::mem::MaybeUninit<Self>,
        fields: serialization::__private::FieldPath,
    ) {
        let fields = fields.as_slice();
        let A6 { v1, v2 } = unsafe { value.assume_init_mut() };
        if fields.contains(&(0usize as serialization::__private::FieldIndex)) {
            unsafe {
                (&mut *((v1) as *mut _ as *mut core::mem::MaybeUninit<u32>)).assume_init_drop();
            }
        }
        if fields.contains(&(1usize as serialization::__private::FieldIndex)) {
            unsafe {
                (&mut *((v2) as *mut _ as *mut core::mem::MaybeUninit<u32>)).assume_init_drop();
            }
        }
    }
}
impl const serialization::__private::FieldPathFinder for A6 {
    fn find_encode<'__a, E: serialization::__private::Encoder>(
        mut path: serialization::__private::FieldPath,
    ) -> &'__a dyn serialization::__private::CompositableEncode<E>
    where
        Self: '__a,
    {
        if path.remaining() == 0 {
            return &serialization::__private::CompositableWrapper::<()>(core::marker::PhantomData);
        }
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            0usize => <i32>::find_encode(path),
            1usize => <u16>::find_encode(path),
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
    fn find_decode<'__a, D: serialization::__private::Decoder>(
        mut path: serialization::__private::FieldPath,
    ) -> &'__a dyn serialization::__private::CompositableDecode<D>
    where
        Self: '__a,
    {
        if path.remaining() == 0 {
            return &serialization::__private::CompositableWrapper::<()>(core::marker::PhantomData);
        }
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            0usize => <i32>::find_decode(path),
            1usize => <u16>::find_decode(path),
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
    fn calc_offset(mut path: serialization::__private::FieldPath) -> usize {
        if path.remaining() == 0 {
            return 0;
        }
        let value = core::mem::MaybeUninit::<Self>::uninit();
        let value = unsafe { value.assume_init_ref() };
        let A6 { v1, v2 } = value;
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            0usize => {
                serialization::__private::offset_of(value, v1)
                    + <i32 as serialization::__private::FieldPathFinder>::calc_offset(path)
            }
            1usize => {
                serialization::__private::offset_of(value, v2)
                    + <u16 as serialization::__private::FieldPathFinder>::calc_offset(path)
            }
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for A6 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(f, "A6", "v1", &self.v1, "v2", &&self.v2)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for A6 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<i32>;
        let _: ::core::cmp::AssertParamIsEq<u16>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for A6 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for A6 {
    #[inline]
    fn eq(&self, other: &A6) -> bool {
        self.v1 == other.v1 && self.v2 == other.v2
    }
}
struct A7(u32, i16);
impl serialization::__private::Encode for A7 {
    fn encode<__E: serialization::__private::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> core::result::Result<(), __E::Error> {
        let A7(f0, f1) = self;
        {
            serialization::__private::encode_struct(self, encoder)
        }
    }
}
impl serialization::__private::Decode for A7 {
    fn decode_in_place<__D: serialization::__private::Decoder>(
        decoder: &mut __D,
        out: &mut core::mem::MaybeUninit<Self>,
    ) -> Result<(), __D::Error> {
        {
            serialization::__private::decode_struct(decoder, out)
        }
    }
}
impl const serialization::__private::SerialDescriptor for A7 {
    const SIZES_LEN: usize = 0
        + <u32 as serialization::__private::SerialDescriptor>::SIZES_LEN
        + <i16 as serialization::__private::SerialDescriptor>::SIZES_LEN;
    fn serial_sizes<__S: const serialization::__private::PrimitiveTypeSizeChecker>(
    ) -> serialization::__private::fastbuf::Buffer<
        [serialization::__private::SerialSize; { Self::SIZES_LEN }],
    > {
        pub use serialization::__private::fastbuf::WriteBuf;
        serialization::__private::order_sizes_by_repr_and_calc_offset::<
            Self,
            __S,
            { Self::SIZES_LEN },
        >(&[
            <u32 as serialization::__private::SerialDescriptor>::serial_sizes::<__S>().as_slice(),
            <i16 as serialization::__private::SerialDescriptor>::serial_sizes::<__S>().as_slice(),
        ])
    }
}
impl serialization::__private::FieldPathDrop for A7 {
    fn drop_fields(
        value: &mut core::mem::MaybeUninit<Self>,
        fields: serialization::__private::FieldPath,
    ) {
        let fields = fields.as_slice();
        let A7(f0, f1) = unsafe { value.assume_init_mut() };
        if fields.contains(&(0usize as serialization::__private::FieldIndex)) {
            unsafe {
                (&mut *((f0) as *mut _ as *mut core::mem::MaybeUninit<u32>)).assume_init_drop();
            }
        }
        if fields.contains(&(1usize as serialization::__private::FieldIndex)) {
            unsafe {
                (&mut *((f1) as *mut _ as *mut core::mem::MaybeUninit<u32>)).assume_init_drop();
            }
        }
    }
}
impl const serialization::__private::FieldPathFinder for A7 {
    fn find_encode<'__a, E: serialization::__private::Encoder>(
        mut path: serialization::__private::FieldPath,
    ) -> &'__a dyn serialization::__private::CompositableEncode<E>
    where
        Self: '__a,
    {
        if path.remaining() == 0 {
            return &serialization::__private::CompositableWrapper::<()>(core::marker::PhantomData);
        }
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            0usize => <u32>::find_encode(path),
            1usize => <i16>::find_encode(path),
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
    fn find_decode<'__a, D: serialization::__private::Decoder>(
        mut path: serialization::__private::FieldPath,
    ) -> &'__a dyn serialization::__private::CompositableDecode<D>
    where
        Self: '__a,
    {
        if path.remaining() == 0 {
            return &serialization::__private::CompositableWrapper::<()>(core::marker::PhantomData);
        }
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            0usize => <u32>::find_decode(path),
            1usize => <i16>::find_decode(path),
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
    fn calc_offset(mut path: serialization::__private::FieldPath) -> usize {
        if path.remaining() == 0 {
            return 0;
        }
        let value = core::mem::MaybeUninit::<Self>::uninit();
        let value = unsafe { value.assume_init_ref() };
        let A7(f0, f1) = value;
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            0usize => {
                serialization::__private::offset_of(value, f0)
                    + <u32 as serialization::__private::FieldPathFinder>::calc_offset(path)
            }
            1usize => {
                serialization::__private::offset_of(value, f1)
                    + <i16 as serialization::__private::FieldPathFinder>::calc_offset(path)
            }
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for A7 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field2_finish(f, "A7", &self.0, &&self.1)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for A7 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
        let _: ::core::cmp::AssertParamIsEq<i16>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for A7 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for A7 {
    #[inline]
    fn eq(&self, other: &A7) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
enum A8 {
    A,
}
impl serialization::__private::Encode for A8 {
    fn encode<__E: serialization::__private::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> core::result::Result<(), __E::Error> {
        serialization::__private::Encoder::encode_enum_variant_key(
            encoder,
            core::any::type_name::<Self>(),
            match self {
                A8::A => "A",
                #[allow(unreachable_patterns)]
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            },
            #[allow(unreachable_code)]
            match self {
                A8::A => 0,
                #[allow(unreachable_patterns)]
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            },
        )?;
        match self {
            A8::A => serialization::__private::encode_struct(self, encoder),
            #[allow(unreachable_patterns)]
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
impl serialization::__private::Decode for A8 {
    fn decode_in_place<__D: serialization::__private::Decoder>(
        decoder: &mut __D,
        out: &mut core::mem::MaybeUninit<Self>,
    ) -> Result<(), __D::Error> {
        match serialization::__private::Decoder::decode_enum(
            decoder,
            core::any::type_name::<Self>(),
        )? {
            serialization::__private::EnumIdentifier::Name(name) => match name {
                "A" => {
                    let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                    *out = core::mem::MaybeUninit::new(A8::A);
                    match unsafe { out.assume_init_mut() } {
                        A8::A => {}
                        _ => ::core::panicking::panic("internal error: entered unreachable code"),
                    }
                    <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                }
                #[allow(unreachable_patterns)]
                _ => Err(serialization::__private::DecodeError::invalid_enum_variant_name())?,
            },
            serialization::__private::EnumIdentifier::Index(index) => {
                #[allow(non_upper_case_globals)]
                const A: usize = 0;

                #[allow(non_upper_case_globals)]
                match index {
                    A => {
                        let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                        *out = core::mem::MaybeUninit::new(A8::A);
                        match unsafe { out.assume_init_mut() } {
                            A8::A => {}
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        }
                        <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(serialization::__private::DecodeError::invalid_enum_variant_index())?,
                }
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for A8 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "A")
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for A8 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for A8 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for A8 {
    #[inline]
    fn eq(&self, other: &A8) -> bool {
        true
    }
}
enum A9 {
    A,
    B,
    C,
}
impl serialization::__private::Encode for A9 {
    fn encode<__E: serialization::__private::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> core::result::Result<(), __E::Error> {
        serialization::__private::Encoder::encode_enum_variant_key(
            encoder,
            core::any::type_name::<Self>(),
            match self {
                A9::A => "A",
                A9::B => "B",
                A9::C => "C",
                #[allow(unreachable_patterns)]
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            },
            #[allow(unreachable_code)]
            match self {
                A9::A => 0,
                A9::B => 0 + 1,
                A9::C => 0 + 1 + 1,
                #[allow(unreachable_patterns)]
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            },
        )?;
        match self {
            A9::A => serialization::__private::encode_struct(self, encoder),
            A9::B => serialization::__private::encode_struct(self, encoder),
            A9::C => serialization::__private::encode_struct(self, encoder),
            #[allow(unreachable_patterns)]
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
impl serialization::__private::Decode for A9 {
    fn decode_in_place<__D: serialization::__private::Decoder>(
        decoder: &mut __D,
        out: &mut core::mem::MaybeUninit<Self>,
    ) -> Result<(), __D::Error> {
        match serialization::__private::Decoder::decode_enum(
            decoder,
            core::any::type_name::<Self>(),
        )? {
            serialization::__private::EnumIdentifier::Name(name) => match name {
                "A" => {
                    let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                    *out = core::mem::MaybeUninit::new(A9::A);
                    match unsafe { out.assume_init_mut() } {
                        A9::A => {}
                        _ => ::core::panicking::panic("internal error: entered unreachable code"),
                    }
                    <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                }
                "B" => {
                    let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                    *out = core::mem::MaybeUninit::new(A9::B);
                    match unsafe { out.assume_init_mut() } {
                        A9::B => {}
                        _ => ::core::panicking::panic("internal error: entered unreachable code"),
                    }
                    <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                }
                "C" => {
                    let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                    *out = core::mem::MaybeUninit::new(A9::C);
                    match unsafe { out.assume_init_mut() } {
                        A9::C => {}
                        _ => ::core::panicking::panic("internal error: entered unreachable code"),
                    }
                    <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                }
                #[allow(unreachable_patterns)]
                _ => Err(serialization::__private::DecodeError::invalid_enum_variant_name())?,
            },
            serialization::__private::EnumIdentifier::Index(index) => {
                #[allow(non_upper_case_globals)]
                const A: usize = 0;
                #[allow(non_upper_case_globals)]
                const B: usize = 0 + 1;
                #[allow(non_upper_case_globals)]
                const C: usize = 0 + 1 + 1;

                #[allow(non_upper_case_globals)]
                match index {
                    A => {
                        let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                        *out = core::mem::MaybeUninit::new(A9::A);
                        match unsafe { out.assume_init_mut() } {
                            A9::A => {}
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        }
                        <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                    }
                    B => {
                        let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                        *out = core::mem::MaybeUninit::new(A9::B);
                        match unsafe { out.assume_init_mut() } {
                            A9::B => {}
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        }
                        <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                    }
                    C => {
                        let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                        *out = core::mem::MaybeUninit::new(A9::C);
                        match unsafe { out.assume_init_mut() } {
                            A9::C => {}
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        }
                        <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(serialization::__private::DecodeError::invalid_enum_variant_index())?,
                }
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for A9 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                A9::A => "A",
                A9::B => "B",
                A9::C => "C",
            },
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for A9 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for A9 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for A9 {
    #[inline]
    fn eq(&self, other: &A9) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
enum A10 {
    A(i32),
}
impl serialization::__private::Encode for A10 {
    fn encode<__E: serialization::__private::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> core::result::Result<(), __E::Error> {
        serialization::__private::Encoder::encode_enum_variant_key(
            encoder,
            core::any::type_name::<Self>(),
            match self {
                A10::A(f0) => "A",
                #[allow(unreachable_patterns)]
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            },
            #[allow(unreachable_code)]
            match self {
                A10::A(f0) => 0,
                #[allow(unreachable_patterns)]
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            },
        )?;
        match self {
            A10::A(f0) => serialization::__private::encode_struct(self, encoder),
            #[allow(unreachable_patterns)]
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
impl serialization::__private::Decode for A10 {
    fn decode_in_place<__D: serialization::__private::Decoder>(
        decoder: &mut __D,
        out: &mut core::mem::MaybeUninit<Self>,
    ) -> Result<(), __D::Error> {
        match serialization::__private::Decoder::decode_enum(
            decoder,
            core::any::type_name::<Self>(),
        )? {
            serialization::__private::EnumIdentifier::Name(name) => match name {
                "A" => {
                    let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                    let f0 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
                    *out = core::mem::MaybeUninit::new(A10::A(f0));
                    match unsafe { out.assume_init_mut() } {
                        A10::A(f0) => {
                            let value_place: &mut core::mem::MaybeUninit<i32> =
                                unsafe { core::mem::transmute(f0) };
                            <__D::TupleDecoder as
                                            serialization::__private::CompositeDecoder>::decode_element::<i32>(tup,
                                        value_place)?;
                        }
                        _ => ::core::panicking::panic("internal error: entered unreachable code"),
                    }
                    <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                }
                #[allow(unreachable_patterns)]
                _ => Err(serialization::__private::DecodeError::invalid_enum_variant_name())?,
            },
            serialization::__private::EnumIdentifier::Index(index) => {
                #[allow(non_upper_case_globals)]
                const A: usize = 0;

                #[allow(non_upper_case_globals)]
                match index {
                    A => {
                        let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                        let f0 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
                        *out = core::mem::MaybeUninit::new(A10::A(f0));
                        match unsafe { out.assume_init_mut() } {
                            A10::A(f0) => {
                                let value_place: &mut core::mem::MaybeUninit<i32> =
                                    unsafe { core::mem::transmute(f0) };
                                <__D::TupleDecoder as
                                            serialization::__private::CompositeDecoder>::decode_element::<i32>(tup,
                                        value_place)?;
                            }
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        }
                        <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(serialization::__private::DecodeError::invalid_enum_variant_index())?,
                }
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for A10 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            A10::A(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "A", &__self_0)
            }
        }
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for A10 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<i32>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for A10 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for A10 {
    #[inline]
    fn eq(&self, other: &A10) -> bool {
        match (self, other) {
            (A10::A(__self_0), A10::A(__arg1_0)) => __self_0 == __arg1_0,
        }
    }
}
enum A11 {
    A(i32, u16),
}
impl serialization::__private::Encode for A11 {
    fn encode<__E: serialization::__private::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> core::result::Result<(), __E::Error> {
        serialization::__private::Encoder::encode_enum_variant_key(
            encoder,
            core::any::type_name::<Self>(),
            match self {
                A11::A(f0, f1) => "A",
                #[allow(unreachable_patterns)]
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            },
            #[allow(unreachable_code)]
            match self {
                A11::A(f0, f1) => 0,
                #[allow(unreachable_patterns)]
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            },
        )?;
        match self {
            A11::A(f0, f1) => serialization::__private::encode_struct(self, encoder),
            #[allow(unreachable_patterns)]
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
impl serialization::__private::Decode for A11 {
    fn decode_in_place<__D: serialization::__private::Decoder>(
        decoder: &mut __D,
        out: &mut core::mem::MaybeUninit<Self>,
    ) -> Result<(), __D::Error> {
        match serialization::__private::Decoder::decode_enum(
            decoder,
            core::any::type_name::<Self>(),
        )? {
            serialization::__private::EnumIdentifier::Name(name) => match name {
                "A" => {
                    let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                    let f0 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
                    let f1 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
                    *out = core::mem::MaybeUninit::new(A11::A(f0, f1));
                    match unsafe { out.assume_init_mut() } {
                        A11::A(f0, f1) => {
                            let value_place: &mut core::mem::MaybeUninit<i32> =
                                unsafe { core::mem::transmute(f0) };
                            <__D::TupleDecoder as
                                            serialization::__private::CompositeDecoder>::decode_element::<i32>(tup,
                                        value_place)?;
                            let value_place: &mut core::mem::MaybeUninit<u16> =
                                unsafe { core::mem::transmute(f1) };
                            <__D::TupleDecoder as
                                            serialization::__private::CompositeDecoder>::decode_element::<u16>(tup,
                                        value_place)?;
                        }
                        _ => ::core::panicking::panic("internal error: entered unreachable code"),
                    }
                    <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                }
                #[allow(unreachable_patterns)]
                _ => Err(serialization::__private::DecodeError::invalid_enum_variant_name())?,
            },
            serialization::__private::EnumIdentifier::Index(index) => {
                #[allow(non_upper_case_globals)]
                const A: usize = 0;

                #[allow(non_upper_case_globals)]
                match index {
                    A => {
                        let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                        let f0 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
                        let f1 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
                        *out = core::mem::MaybeUninit::new(A11::A(f0, f1));
                        match unsafe { out.assume_init_mut() } {
                            A11::A(f0, f1) => {
                                let value_place: &mut core::mem::MaybeUninit<i32> =
                                    unsafe { core::mem::transmute(f0) };
                                <__D::TupleDecoder as
                                            serialization::__private::CompositeDecoder>::decode_element::<i32>(tup,
                                        value_place)?;
                                let value_place: &mut core::mem::MaybeUninit<u16> =
                                    unsafe { core::mem::transmute(f1) };
                                <__D::TupleDecoder as
                                            serialization::__private::CompositeDecoder>::decode_element::<u16>(tup,
                                        value_place)?;
                            }
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        }
                        <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(serialization::__private::DecodeError::invalid_enum_variant_index())?,
                }
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for A11 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            A11::A(__self_0, __self_1) => {
                ::core::fmt::Formatter::debug_tuple_field2_finish(f, "A", __self_0, &__self_1)
            }
        }
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for A11 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<i32>;
        let _: ::core::cmp::AssertParamIsEq<u16>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for A11 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for A11 {
    #[inline]
    fn eq(&self, other: &A11) -> bool {
        match (self, other) {
            (A11::A(__self_0, __self_1), A11::A(__arg1_0, __arg1_1)) => {
                __self_0 == __arg1_0 && __self_1 == __arg1_1
            }
        }
    }
}
enum A12 {
    A(i32, u16),
    B(u32, i16),
}
impl serialization::__private::Encode for A12 {
    fn encode<__E: serialization::__private::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> core::result::Result<(), __E::Error> {
        serialization::__private::Encoder::encode_enum_variant_key(
            encoder,
            core::any::type_name::<Self>(),
            match self {
                A12::A(f0, f1) => "A",
                A12::B(f0, f1) => "B",
                #[allow(unreachable_patterns)]
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            },
            #[allow(unreachable_code)]
            match self {
                A12::A(f0, f1) => 0,
                A12::B(f0, f1) => 0 + 1,
                #[allow(unreachable_patterns)]
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            },
        )?;
        match self {
            A12::A(f0, f1) => serialization::__private::encode_struct(self, encoder),
            A12::B(f0, f1) => serialization::__private::encode_struct(self, encoder),
            #[allow(unreachable_patterns)]
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
impl serialization::__private::Decode for A12 {
    fn decode_in_place<__D: serialization::__private::Decoder>(
        decoder: &mut __D,
        out: &mut core::mem::MaybeUninit<Self>,
    ) -> Result<(), __D::Error> {
        match serialization::__private::Decoder::decode_enum(
            decoder,
            core::any::type_name::<Self>(),
        )? {
            serialization::__private::EnumIdentifier::Name(name) => match name {
                "A" => {
                    let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                    let f0 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
                    let f1 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
                    *out = core::mem::MaybeUninit::new(A12::A(f0, f1));
                    match unsafe { out.assume_init_mut() } {
                        A12::A(f0, f1) => {
                            let value_place: &mut core::mem::MaybeUninit<i32> =
                                unsafe { core::mem::transmute(f0) };
                            <__D::TupleDecoder as
                                            serialization::__private::CompositeDecoder>::decode_element::<i32>(tup,
                                        value_place)?;
                            let value_place: &mut core::mem::MaybeUninit<u16> =
                                unsafe { core::mem::transmute(f1) };
                            <__D::TupleDecoder as
                                            serialization::__private::CompositeDecoder>::decode_element::<u16>(tup,
                                        value_place)?;
                        }
                        _ => ::core::panicking::panic("internal error: entered unreachable code"),
                    }
                    <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                }
                "B" => {
                    let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                    let f0 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
                    let f1 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
                    *out = core::mem::MaybeUninit::new(A12::B(f0, f1));
                    match unsafe { out.assume_init_mut() } {
                        A12::B(f0, f1) => {
                            let value_place: &mut core::mem::MaybeUninit<u32> =
                                unsafe { core::mem::transmute(f0) };
                            <__D::TupleDecoder as
                                            serialization::__private::CompositeDecoder>::decode_element::<u32>(tup,
                                        value_place)?;
                            let value_place: &mut core::mem::MaybeUninit<i16> =
                                unsafe { core::mem::transmute(f1) };
                            <__D::TupleDecoder as
                                            serialization::__private::CompositeDecoder>::decode_element::<i16>(tup,
                                        value_place)?;
                        }
                        _ => ::core::panicking::panic("internal error: entered unreachable code"),
                    }
                    <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                }
                #[allow(unreachable_patterns)]
                _ => Err(serialization::__private::DecodeError::invalid_enum_variant_name())?,
            },
            serialization::__private::EnumIdentifier::Index(index) => {
                #[allow(non_upper_case_globals)]
                const A: usize = 0;
                #[allow(non_upper_case_globals)]
                const B: usize = 0 + 1;

                #[allow(non_upper_case_globals)]
                match index {
                    A => {
                        let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                        let f0 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
                        let f1 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
                        *out = core::mem::MaybeUninit::new(A12::A(f0, f1));
                        match unsafe { out.assume_init_mut() } {
                            A12::A(f0, f1) => {
                                let value_place: &mut core::mem::MaybeUninit<i32> =
                                    unsafe { core::mem::transmute(f0) };
                                <__D::TupleDecoder as
                                            serialization::__private::CompositeDecoder>::decode_element::<i32>(tup,
                                        value_place)?;
                                let value_place: &mut core::mem::MaybeUninit<u16> =
                                    unsafe { core::mem::transmute(f1) };
                                <__D::TupleDecoder as
                                            serialization::__private::CompositeDecoder>::decode_element::<u16>(tup,
                                        value_place)?;
                            }
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        }
                        <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                    }
                    B => {
                        let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                        let f0 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
                        let f1 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
                        *out = core::mem::MaybeUninit::new(A12::B(f0, f1));
                        match unsafe { out.assume_init_mut() } {
                            A12::B(f0, f1) => {
                                let value_place: &mut core::mem::MaybeUninit<u32> =
                                    unsafe { core::mem::transmute(f0) };
                                <__D::TupleDecoder as
                                            serialization::__private::CompositeDecoder>::decode_element::<u32>(tup,
                                        value_place)?;
                                let value_place: &mut core::mem::MaybeUninit<i16> =
                                    unsafe { core::mem::transmute(f1) };
                                <__D::TupleDecoder as
                                            serialization::__private::CompositeDecoder>::decode_element::<i16>(tup,
                                        value_place)?;
                            }
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        }
                        <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(serialization::__private::DecodeError::invalid_enum_variant_index())?,
                }
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for A12 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            A12::A(__self_0, __self_1) => {
                ::core::fmt::Formatter::debug_tuple_field2_finish(f, "A", __self_0, &__self_1)
            }
            A12::B(__self_0, __self_1) => {
                ::core::fmt::Formatter::debug_tuple_field2_finish(f, "B", __self_0, &__self_1)
            }
        }
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for A12 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<i32>;
        let _: ::core::cmp::AssertParamIsEq<u16>;
        let _: ::core::cmp::AssertParamIsEq<u32>;
        let _: ::core::cmp::AssertParamIsEq<i16>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for A12 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for A12 {
    #[inline]
    fn eq(&self, other: &A12) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (A12::A(__self_0, __self_1), A12::A(__arg1_0, __arg1_1)) => {
                    __self_0 == __arg1_0 && __self_1 == __arg1_1
                }
                (A12::B(__self_0, __self_1), A12::B(__arg1_0, __arg1_1)) => {
                    __self_0 == __arg1_0 && __self_1 == __arg1_1
                }
                _ => unsafe { ::core::intrinsics::unreachable() },
            }
    }
}
enum A13 {
    A,
    B(u32),
    C,
}
impl serialization::__private::Encode for A13 {
    fn encode<__E: serialization::__private::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> core::result::Result<(), __E::Error> {
        serialization::__private::Encoder::encode_enum_variant_key(
            encoder,
            core::any::type_name::<Self>(),
            match self {
                A13::A => "A",
                A13::B(f0) => "B",
                A13::C => "C",
                #[allow(unreachable_patterns)]
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            },
            #[allow(unreachable_code)]
            match self {
                A13::A => 0,
                A13::B(f0) => 0 + 1,
                A13::C => 0 + 1 + 1,
                #[allow(unreachable_patterns)]
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            },
        )?;
        match self {
            A13::A => serialization::__private::encode_struct(self, encoder),
            A13::B(f0) => serialization::__private::encode_struct(self, encoder),
            A13::C => serialization::__private::encode_struct(self, encoder),
            #[allow(unreachable_patterns)]
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
impl serialization::__private::Decode for A13 {
    fn decode_in_place<__D: serialization::__private::Decoder>(
        decoder: &mut __D,
        out: &mut core::mem::MaybeUninit<Self>,
    ) -> Result<(), __D::Error> {
        match serialization::__private::Decoder::decode_enum(
            decoder,
            core::any::type_name::<Self>(),
        )? {
            serialization::__private::EnumIdentifier::Name(name) => match name {
                "A" => {
                    let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                    *out = core::mem::MaybeUninit::new(A13::A);
                    match unsafe { out.assume_init_mut() } {
                        A13::A => {}
                        _ => ::core::panicking::panic("internal error: entered unreachable code"),
                    }
                    <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                }
                "B" => {
                    let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                    let f0 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
                    *out = core::mem::MaybeUninit::new(A13::B(f0));
                    match unsafe { out.assume_init_mut() } {
                        A13::B(f0) => {
                            let value_place: &mut core::mem::MaybeUninit<u32> =
                                unsafe { core::mem::transmute(f0) };
                            <__D::TupleDecoder as
                                            serialization::__private::CompositeDecoder>::decode_element::<u32>(tup,
                                        value_place)?;
                        }
                        _ => ::core::panicking::panic("internal error: entered unreachable code"),
                    }
                    <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                }
                "C" => {
                    let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                    *out = core::mem::MaybeUninit::new(A13::C);
                    match unsafe { out.assume_init_mut() } {
                        A13::C => {}
                        _ => ::core::panicking::panic("internal error: entered unreachable code"),
                    }
                    <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                }
                #[allow(unreachable_patterns)]
                _ => Err(serialization::__private::DecodeError::invalid_enum_variant_name())?,
            },
            serialization::__private::EnumIdentifier::Index(index) => {
                #[allow(non_upper_case_globals)]
                const A: usize = 0;
                #[allow(non_upper_case_globals)]
                const B: usize = 0 + 1;
                #[allow(non_upper_case_globals)]
                const C: usize = 0 + 1 + 1;

                #[allow(non_upper_case_globals)]
                match index {
                    A => {
                        let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                        *out = core::mem::MaybeUninit::new(A13::A);
                        match unsafe { out.assume_init_mut() } {
                            A13::A => {}
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        }
                        <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                    }
                    B => {
                        let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                        let f0 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
                        *out = core::mem::MaybeUninit::new(A13::B(f0));
                        match unsafe { out.assume_init_mut() } {
                            A13::B(f0) => {
                                let value_place: &mut core::mem::MaybeUninit<u32> =
                                    unsafe { core::mem::transmute(f0) };
                                <__D::TupleDecoder as
                                            serialization::__private::CompositeDecoder>::decode_element::<u32>(tup,
                                        value_place)?;
                            }
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        }
                        <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                    }
                    C => {
                        let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                        *out = core::mem::MaybeUninit::new(A13::C);
                        match unsafe { out.assume_init_mut() } {
                            A13::C => {}
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        }
                        <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(serialization::__private::DecodeError::invalid_enum_variant_index())?,
                }
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for A13 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            A13::A => ::core::fmt::Formatter::write_str(f, "A"),
            A13::B(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "B", &__self_0)
            }
            A13::C => ::core::fmt::Formatter::write_str(f, "C"),
        }
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for A13 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for A13 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for A13 {
    #[inline]
    fn eq(&self, other: &A13) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (A13::B(__self_0), A13::B(__arg1_0)) => __self_0 == __arg1_0,
                _ => true,
            }
    }
}
struct A14<T> {
    vaule: std::marker::PhantomData<T>,
}
impl<T> serialization::__private::Encode for A14<T>
where
    T: serialization::__private::Encode + serialization::__private::Decode + 'static,
{
    fn encode<__E: serialization::__private::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> core::result::Result<(), __E::Error> {
        let A14 { vaule } = self;
        {
            pub use serialization::__private::CompositeEncoder;
            let ref mut compound = serialization::__private::Encoder::encode_struct(encoder)?;
            <__E::StructEncoder>::encode_element(compound, vaule)?;
            <__E::StructEncoder>::end(compound)
        }
    }
}
impl<T> serialization::__private::Decode for A14<T>
where
    T: serialization::__private::Encode + serialization::__private::Decode + 'static,
{
    fn decode_in_place<__D: serialization::__private::Decoder>(
        decoder: &mut __D,
        out: &mut core::mem::MaybeUninit<Self>,
    ) -> Result<(), __D::Error> {
        {
            let A14 { vaule } = unsafe { out.assume_init_mut() };
            pub use serialization::__private::CompositeDecoder;
            let compound = serialization::__private::Decoder::decode_struct(decoder)?;
            let __v: &mut core::mem::MaybeUninit<std::marker::PhantomData<T>> =
                unsafe { core::mem::transmute(vaule) };
            <__D::StructDecoder>::decode_element(compound, __v)?;
            <__D::StructDecoder>::end(compound)
        }
    }
}
#[automatically_derived]
impl<T: ::core::fmt::Debug> ::core::fmt::Debug for A14<T> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "A14", "vaule", &&self.vaule)
    }
}
#[automatically_derived]
impl<T: ::core::cmp::Eq> ::core::cmp::Eq for A14<T> {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<std::marker::PhantomData<T>>;
    }
}
#[automatically_derived]
impl<T> ::core::marker::StructuralPartialEq for A14<T> {}
#[automatically_derived]
impl<T: ::core::cmp::PartialEq> ::core::cmp::PartialEq for A14<T> {
    #[inline]
    fn eq(&self, other: &A14<T>) -> bool {
        self.vaule == other.vaule
    }
}
struct A15<T> {
    value: T,
}
impl<T> serialization::__private::Encode for A15<T>
where
    T: serialization::__private::Encode + serialization::__private::Decode + 'static,
{
    fn encode<__E: serialization::__private::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> core::result::Result<(), __E::Error> {
        let A15 { value } = self;
        {
            pub use serialization::__private::CompositeEncoder;
            let ref mut compound = serialization::__private::Encoder::encode_struct(encoder)?;
            <__E::StructEncoder>::encode_element(compound, value)?;
            <__E::StructEncoder>::end(compound)
        }
    }
}
impl<T> serialization::__private::Decode for A15<T>
where
    T: serialization::__private::Encode + serialization::__private::Decode + 'static,
{
    fn decode_in_place<__D: serialization::__private::Decoder>(
        decoder: &mut __D,
        out: &mut core::mem::MaybeUninit<Self>,
    ) -> Result<(), __D::Error> {
        {
            let A15 { value } = unsafe { out.assume_init_mut() };
            pub use serialization::__private::CompositeDecoder;
            let compound = serialization::__private::Decoder::decode_struct(decoder)?;
            let __v: &mut core::mem::MaybeUninit<T> = unsafe { core::mem::transmute(value) };
            <__D::StructDecoder>::decode_element(compound, __v)?;
            <__D::StructDecoder>::end(compound)
        }
    }
}
#[automatically_derived]
impl<T: ::core::fmt::Debug> ::core::fmt::Debug for A15<T> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "A15", "value", &&self.value)
    }
}
#[automatically_derived]
impl<T: ::core::cmp::Eq> ::core::cmp::Eq for A15<T> {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<T>;
    }
}
#[automatically_derived]
impl<T> ::core::marker::StructuralPartialEq for A15<T> {}
#[automatically_derived]
impl<T: ::core::cmp::PartialEq> ::core::cmp::PartialEq for A15<T> {
    #[inline]
    fn eq(&self, other: &A15<T>) -> bool {
        self.value == other.value
    }
}
enum A16<T, T2> {
    A(T),
    B(std::marker::PhantomData<T2>),
}
impl<T, T2> serialization::__private::Encode for A16<T, T2>
where
    T: serialization::__private::Encode + serialization::__private::Decode + 'static,
    T2: serialization::__private::Encode + serialization::__private::Decode + 'static,
{
    fn encode<__E: serialization::__private::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> core::result::Result<(), __E::Error> {
        serialization::__private::Encoder::encode_enum_variant_key(
            encoder,
            core::any::type_name::<Self>(),
            match self {
                A16::A(f0) => "A",
                A16::B(f0) => "B",
                #[allow(unreachable_patterns)]
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            },
            #[allow(unreachable_code)]
            match self {
                A16::A(f0) => 0,
                A16::B(f0) => 0 + 1,
                #[allow(unreachable_patterns)]
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            },
        )?;
        match self {
            A16::A(f0) => {
                pub use serialization::__private::CompositeEncoder;
                let ref mut compound = serialization::__private::Encoder::encode_struct(encoder)?;
                <__E::StructEncoder>::encode_element(compound, f0)?;
                <__E::StructEncoder>::end(compound)
            }
            A16::B(f0) => {
                pub use serialization::__private::CompositeEncoder;
                let ref mut compound = serialization::__private::Encoder::encode_struct(encoder)?;
                <__E::StructEncoder>::encode_element(compound, f0)?;
                <__E::StructEncoder>::end(compound)
            }
            #[allow(unreachable_patterns)]
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
impl<T, T2> serialization::__private::Decode for A16<T, T2>
where
    T: serialization::__private::Encode + serialization::__private::Decode + 'static,
    T2: serialization::__private::Encode + serialization::__private::Decode + 'static,
{
    fn decode_in_place<__D: serialization::__private::Decoder>(
        decoder: &mut __D,
        out: &mut core::mem::MaybeUninit<Self>,
    ) -> Result<(), __D::Error> {
        match serialization::__private::Decoder::decode_enum(
            decoder,
            core::any::type_name::<Self>(),
        )? {
            serialization::__private::EnumIdentifier::Name(name) => match name {
                "A" => {
                    let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                    let f0 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
                    *out = core::mem::MaybeUninit::new(A16::A(f0));
                    match unsafe { out.assume_init_mut() } {
                        A16::A(f0) => {
                            let value_place: &mut core::mem::MaybeUninit<T> =
                                unsafe { core::mem::transmute(f0) };
                            <__D::TupleDecoder as
                                            serialization::__private::CompositeDecoder>::decode_element::<T>(tup,
                                        value_place)?;
                        }
                        _ => ::core::panicking::panic("internal error: entered unreachable code"),
                    }
                    <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                }
                "B" => {
                    let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                    let f0 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
                    *out = core::mem::MaybeUninit::new(A16::B(f0));
                    match unsafe { out.assume_init_mut() } {
                        A16::B(f0) => {
                            let value_place: &mut core::mem::MaybeUninit<
                                std::marker::PhantomData<T2>,
                            > = unsafe { core::mem::transmute(f0) };
                            <__D::TupleDecoder as
                                            serialization::__private::CompositeDecoder>::decode_element::<std::marker::PhantomData<T2>>(tup,
                                        value_place)?;
                        }
                        _ => ::core::panicking::panic("internal error: entered unreachable code"),
                    }
                    <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                }
                #[allow(unreachable_patterns)]
                _ => Err(serialization::__private::DecodeError::invalid_enum_variant_name())?,
            },
            serialization::__private::EnumIdentifier::Index(index) => {
                #[allow(non_upper_case_globals)]
                const A: usize = 0;
                #[allow(non_upper_case_globals)]
                const B: usize = 0 + 1;

                #[allow(non_upper_case_globals)]
                match index {
                    A => {
                        let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                        let f0 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
                        *out = core::mem::MaybeUninit::new(A16::A(f0));
                        match unsafe { out.assume_init_mut() } {
                            A16::A(f0) => {
                                let value_place: &mut core::mem::MaybeUninit<T> =
                                    unsafe { core::mem::transmute(f0) };
                                <__D::TupleDecoder as
                                            serialization::__private::CompositeDecoder>::decode_element::<T>(tup,
                                        value_place)?;
                            }
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        }
                        <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                    }
                    B => {
                        let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                        let f0 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
                        *out = core::mem::MaybeUninit::new(A16::B(f0));
                        match unsafe { out.assume_init_mut() } {
                            A16::B(f0) => {
                                let value_place: &mut core::mem::MaybeUninit<
                                    std::marker::PhantomData<T2>,
                                > = unsafe { core::mem::transmute(f0) };
                                <__D::TupleDecoder as
                                            serialization::__private::CompositeDecoder>::decode_element::<std::marker::PhantomData<T2>>(tup,
                                        value_place)?;
                            }
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        }
                        <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(serialization::__private::DecodeError::invalid_enum_variant_index())?,
                }
            }
        }
    }
}
#[automatically_derived]
impl<T: ::core::fmt::Debug, T2: ::core::fmt::Debug> ::core::fmt::Debug for A16<T, T2> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            A16::A(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "A", &__self_0)
            }
            A16::B(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "B", &__self_0)
            }
        }
    }
}
#[automatically_derived]
impl<T: ::core::cmp::Eq, T2: ::core::cmp::Eq> ::core::cmp::Eq for A16<T, T2> {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<T>;
        let _: ::core::cmp::AssertParamIsEq<std::marker::PhantomData<T2>>;
    }
}
#[automatically_derived]
impl<T, T2> ::core::marker::StructuralPartialEq for A16<T, T2> {}
#[automatically_derived]
impl<T: ::core::cmp::PartialEq, T2: ::core::cmp::PartialEq> ::core::cmp::PartialEq for A16<T, T2> {
    #[inline]
    fn eq(&self, other: &A16<T, T2>) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (A16::A(__self_0), A16::A(__arg1_0)) => __self_0 == __arg1_0,
                (A16::B(__self_0), A16::B(__arg1_0)) => __self_0 == __arg1_0,
                _ => unsafe { ::core::intrinsics::unreachable() },
            }
    }
}
struct A17<'a> {
    value: &'a str,
}
impl<'a> serialization::__private::Encode for A17<'a>
where
    'a: 'static,
{
    fn encode<__E: serialization::__private::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> core::result::Result<(), __E::Error> {
        let A17 { value } = self;
        {
            serialization::__private::encode_struct(self, encoder)
        }
    }
}
impl<'a> serialization::__private::Decode for A17<'a>
where
    'a: 'static,
{
    fn decode_in_place<__D: serialization::__private::Decoder>(
        decoder: &mut __D,
        out: &mut core::mem::MaybeUninit<Self>,
    ) -> Result<(), __D::Error> {
        {
            serialization::__private::decode_struct(decoder, out)
        }
    }
}
impl<'a> const serialization::__private::SerialDescriptor for A17<'a>
where
    'a: 'static,
{
    const SIZES_LEN: usize = 0 + <&'a str as serialization::__private::SerialDescriptor>::SIZES_LEN;
    fn serial_sizes<__S: const serialization::__private::PrimitiveTypeSizeChecker>(
    ) -> serialization::__private::fastbuf::Buffer<
        [serialization::__private::SerialSize; { Self::SIZES_LEN }],
    > {
        pub use serialization::__private::fastbuf::WriteBuf;
        serialization::__private::order_sizes_by_repr_and_calc_offset::<
            Self,
            __S,
            { Self::SIZES_LEN },
        >(&[
            <&'a str as serialization::__private::SerialDescriptor>::serial_sizes::<__S>()
                .as_slice(),
        ])
    }
}
impl<'a> serialization::__private::FieldPathDrop for A17<'a>
where
    'a: 'static,
{
    fn drop_fields(
        value: &mut core::mem::MaybeUninit<Self>,
        fields: serialization::__private::FieldPath,
    ) {
        let fields = fields.as_slice();
        let A17 { value } = unsafe { value.assume_init_mut() };
        if fields.contains(&(0usize as serialization::__private::FieldIndex)) {
            unsafe {
                (&mut *((value) as *mut _ as *mut core::mem::MaybeUninit<u32>)).assume_init_drop();
            }
        }
    }
}
impl<'a> const serialization::__private::FieldPathFinder for A17<'a>
where
    'a: 'static,
{
    fn find_encode<'__a, E: serialization::__private::Encoder>(
        mut path: serialization::__private::FieldPath,
    ) -> &'__a dyn serialization::__private::CompositableEncode<E>
    where
        Self: '__a,
    {
        if path.remaining() == 0 {
            return &serialization::__private::CompositableWrapper::<()>(core::marker::PhantomData);
        }
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            0usize => <&'a str>::find_encode(path),
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
    fn find_decode<'__a, D: serialization::__private::Decoder>(
        mut path: serialization::__private::FieldPath,
    ) -> &'__a dyn serialization::__private::CompositableDecode<D>
    where
        Self: '__a,
    {
        if path.remaining() == 0 {
            return &serialization::__private::CompositableWrapper::<()>(core::marker::PhantomData);
        }
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            0usize => <&'a str>::find_decode(path),
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
    fn calc_offset(mut path: serialization::__private::FieldPath) -> usize {
        if path.remaining() == 0 {
            return 0;
        }
        let value = core::mem::MaybeUninit::<Self>::uninit();
        let value = unsafe { value.assume_init_ref() };
        let A17 { value } = value;
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            0usize => {
                serialization::__private::offset_of(value, value)
                    + <&'a str as serialization::__private::FieldPathFinder>::calc_offset(path)
            }
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
#[automatically_derived]
impl<'a> ::core::fmt::Debug for A17<'a> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "A17", "value", &&self.value)
    }
}
#[automatically_derived]
impl<'a> ::core::cmp::Eq for A17<'a> {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<&'a str>;
    }
}
#[automatically_derived]
impl<'a> ::core::marker::StructuralPartialEq for A17<'a> {}
#[automatically_derived]
impl<'a> ::core::cmp::PartialEq for A17<'a> {
    #[inline]
    fn eq(&self, other: &A17<'a>) -> bool {
        self.value == other.value
    }
}
struct A18<'a> {
    value: &'a str,
}
impl<'a> serialization::__private::Encode for A18<'a>
where
    'a: 'static,
{
    fn encode<__E: serialization::__private::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> core::result::Result<(), __E::Error> {
        let A18 { value } = self;
        {
            serialization::__private::encode_struct(self, encoder)
        }
    }
}
impl<'a> serialization::__private::Decode for A18<'a>
where
    'a: 'static,
{
    fn decode_in_place<__D: serialization::__private::Decoder>(
        decoder: &mut __D,
        out: &mut core::mem::MaybeUninit<Self>,
    ) -> Result<(), __D::Error> {
        {
            serialization::__private::decode_struct(decoder, out)
        }
    }
}
impl<'a> const serialization::__private::SerialDescriptor for A18<'a>
where
    'a: 'static,
{
    const SIZES_LEN: usize = 0 + <&'a str as serialization::__private::SerialDescriptor>::SIZES_LEN;
    fn serial_sizes<__S: const serialization::__private::PrimitiveTypeSizeChecker>(
    ) -> serialization::__private::fastbuf::Buffer<
        [serialization::__private::SerialSize; { Self::SIZES_LEN }],
    > {
        pub use serialization::__private::fastbuf::WriteBuf;
        serialization::__private::order_sizes_by_repr_and_calc_offset::<
            Self,
            __S,
            { Self::SIZES_LEN },
        >(&[
            <&'a str as serialization::__private::SerialDescriptor>::serial_sizes::<__S>()
                .as_slice(),
        ])
    }
}
impl<'a> serialization::__private::FieldPathDrop for A18<'a>
where
    'a: 'static,
{
    fn drop_fields(
        value: &mut core::mem::MaybeUninit<Self>,
        fields: serialization::__private::FieldPath,
    ) {
        let fields = fields.as_slice();
        let A18 { value } = unsafe { value.assume_init_mut() };
        if fields.contains(&(0usize as serialization::__private::FieldIndex)) {
            unsafe {
                (&mut *((value) as *mut _ as *mut core::mem::MaybeUninit<u32>)).assume_init_drop();
            }
        }
    }
}
impl<'a> const serialization::__private::FieldPathFinder for A18<'a>
where
    'a: 'static,
{
    fn find_encode<'__a, E: serialization::__private::Encoder>(
        mut path: serialization::__private::FieldPath,
    ) -> &'__a dyn serialization::__private::CompositableEncode<E>
    where
        Self: '__a,
    {
        if path.remaining() == 0 {
            return &serialization::__private::CompositableWrapper::<()>(core::marker::PhantomData);
        }
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            0usize => <&'a str>::find_encode(path),
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
    fn find_decode<'__a, D: serialization::__private::Decoder>(
        mut path: serialization::__private::FieldPath,
    ) -> &'__a dyn serialization::__private::CompositableDecode<D>
    where
        Self: '__a,
    {
        if path.remaining() == 0 {
            return &serialization::__private::CompositableWrapper::<()>(core::marker::PhantomData);
        }
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            0usize => <&'a str>::find_decode(path),
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
    fn calc_offset(mut path: serialization::__private::FieldPath) -> usize {
        if path.remaining() == 0 {
            return 0;
        }
        let value = core::mem::MaybeUninit::<Self>::uninit();
        let value = unsafe { value.assume_init_ref() };
        let A18 { value } = value;
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            0usize => {
                serialization::__private::offset_of(value, value)
                    + <&'a str as serialization::__private::FieldPathFinder>::calc_offset(path)
            }
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
#[automatically_derived]
impl<'a> ::core::fmt::Debug for A18<'a> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "A18", "value", &&self.value)
    }
}
#[automatically_derived]
impl<'a> ::core::cmp::Eq for A18<'a> {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<&'a str>;
    }
}
#[automatically_derived]
impl<'a> ::core::marker::StructuralPartialEq for A18<'a> {}
#[automatically_derived]
impl<'a> ::core::cmp::PartialEq for A18<'a> {
    #[inline]
    fn eq(&self, other: &A18<'a>) -> bool {
        self.value == other.value
    }
}
enum A19 {}
impl serialization::__private::Encode for A19 {
    fn encode<__E: serialization::__private::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> core::result::Result<(), __E::Error> {
        serialization::__private::Encoder::encode_enum_variant_key(
            encoder,
            core::any::type_name::<Self>(),
            match self {
                #[allow(unreachable_patterns)]
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            },
            #[allow(unreachable_code)]
            match self {
                #[allow(unreachable_patterns)]
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            },
        )?;
        match self {
            #[allow(unreachable_patterns)]
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
impl serialization::__private::Decode for A19 {
    fn decode_in_place<__D: serialization::__private::Decoder>(
        decoder: &mut __D,
        out: &mut core::mem::MaybeUninit<Self>,
    ) -> Result<(), __D::Error> {
        match serialization::__private::Decoder::decode_enum(
            decoder,
            core::any::type_name::<Self>(),
        )? {
            serialization::__private::EnumIdentifier::Name(name) => match name {
                #[allow(unreachable_patterns)]
                _ => Err(serialization::__private::DecodeError::invalid_enum_variant_name())?,
            },
            serialization::__private::EnumIdentifier::Index(index) =>
            {
                #[allow(non_upper_case_globals)]
                match index {
                    #[allow(unreachable_patterns)]
                    _ => Err(serialization::__private::DecodeError::invalid_enum_variant_index())?,
                }
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for A19 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {}
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for A19 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for A19 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for A19 {
    #[inline]
    fn eq(&self, other: &A19) -> bool {
        match *self {}
    }
}
enum A20 {
    A = 2,
    B = 4,
    C = 6,
    D = 8,
    E = 10,
}
impl serialization::__private::Encode for A20 {
    fn encode<__E: serialization::__private::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> core::result::Result<(), __E::Error> {
        serialization::__private::Encoder::encode_enum_variant_key(
            encoder,
            core::any::type_name::<Self>(),
            match self {
                A20::A => "A",
                A20::B => "B",
                A20::C => "C",
                A20::D => "D",
                A20::E => "E",
                #[allow(unreachable_patterns)]
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            },
            #[allow(unreachable_code)]
            match self {
                A20::A => 2,
                A20::B => 4,
                A20::C => 6,
                A20::D => 8,
                A20::E => 10,
                #[allow(unreachable_patterns)]
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            },
        )?;
        match self {
            A20::A => serialization::__private::encode_struct(self, encoder),
            A20::B => serialization::__private::encode_struct(self, encoder),
            A20::C => serialization::__private::encode_struct(self, encoder),
            A20::D => serialization::__private::encode_struct(self, encoder),
            A20::E => serialization::__private::encode_struct(self, encoder),
            #[allow(unreachable_patterns)]
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
impl serialization::__private::Decode for A20 {
    fn decode_in_place<__D: serialization::__private::Decoder>(
        decoder: &mut __D,
        out: &mut core::mem::MaybeUninit<Self>,
    ) -> Result<(), __D::Error> {
        match serialization::__private::Decoder::decode_enum(
            decoder,
            core::any::type_name::<Self>(),
        )? {
            serialization::__private::EnumIdentifier::Name(name) => match name {
                "A" => {
                    let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                    *out = core::mem::MaybeUninit::new(A20::A);
                    match unsafe { out.assume_init_mut() } {
                        A20::A => {}
                        _ => ::core::panicking::panic("internal error: entered unreachable code"),
                    }
                    <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                }
                "B" => {
                    let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                    *out = core::mem::MaybeUninit::new(A20::B);
                    match unsafe { out.assume_init_mut() } {
                        A20::B => {}
                        _ => ::core::panicking::panic("internal error: entered unreachable code"),
                    }
                    <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                }
                "C" => {
                    let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                    *out = core::mem::MaybeUninit::new(A20::C);
                    match unsafe { out.assume_init_mut() } {
                        A20::C => {}
                        _ => ::core::panicking::panic("internal error: entered unreachable code"),
                    }
                    <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                }
                "D" => {
                    let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                    *out = core::mem::MaybeUninit::new(A20::D);
                    match unsafe { out.assume_init_mut() } {
                        A20::D => {}
                        _ => ::core::panicking::panic("internal error: entered unreachable code"),
                    }
                    <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                }
                "E" => {
                    let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                    *out = core::mem::MaybeUninit::new(A20::E);
                    match unsafe { out.assume_init_mut() } {
                        A20::E => {}
                        _ => ::core::panicking::panic("internal error: entered unreachable code"),
                    }
                    <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                }
                #[allow(unreachable_patterns)]
                _ => Err(serialization::__private::DecodeError::invalid_enum_variant_name())?,
            },
            serialization::__private::EnumIdentifier::Index(index) => {
                #[allow(non_upper_case_globals)]
                const A: usize = 2;
                #[allow(non_upper_case_globals)]
                const B: usize = 4;
                #[allow(non_upper_case_globals)]
                const C: usize = 6;
                #[allow(non_upper_case_globals)]
                const D: usize = 8;
                #[allow(non_upper_case_globals)]
                const E: usize = 10;

                #[allow(non_upper_case_globals)]
                match index {
                    A => {
                        let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                        *out = core::mem::MaybeUninit::new(A20::A);
                        match unsafe { out.assume_init_mut() } {
                            A20::A => {}
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        }
                        <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                    }
                    B => {
                        let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                        *out = core::mem::MaybeUninit::new(A20::B);
                        match unsafe { out.assume_init_mut() } {
                            A20::B => {}
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        }
                        <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                    }
                    C => {
                        let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                        *out = core::mem::MaybeUninit::new(A20::C);
                        match unsafe { out.assume_init_mut() } {
                            A20::C => {}
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        }
                        <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                    }
                    D => {
                        let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                        *out = core::mem::MaybeUninit::new(A20::D);
                        match unsafe { out.assume_init_mut() } {
                            A20::D => {}
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        }
                        <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                    }
                    E => {
                        let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                        *out = core::mem::MaybeUninit::new(A20::E);
                        match unsafe { out.assume_init_mut() } {
                            A20::E => {}
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        }
                        <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(serialization::__private::DecodeError::invalid_enum_variant_index())?,
                }
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for A20 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                A20::A => "A",
                A20::B => "B",
                A20::C => "C",
                A20::D => "D",
                A20::E => "E",
            },
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for A20 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for A20 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for A20 {
    #[inline]
    fn eq(&self, other: &A20) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
struct A21 {
    value: &'static str,
}
impl serialization::__private::Encode for A21 {
    fn encode<__E: serialization::__private::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> core::result::Result<(), __E::Error> {
        let A21 { value } = self;
        {
            serialization::__private::encode_struct(self, encoder)
        }
    }
}
impl serialization::__private::Decode for A21 {
    fn decode_in_place<__D: serialization::__private::Decoder>(
        decoder: &mut __D,
        out: &mut core::mem::MaybeUninit<Self>,
    ) -> Result<(), __D::Error> {
        {
            serialization::__private::decode_struct(decoder, out)
        }
    }
}
impl const serialization::__private::SerialDescriptor for A21 {
    const SIZES_LEN: usize =
        0 + <&'static str as serialization::__private::SerialDescriptor>::SIZES_LEN;
    fn serial_sizes<__S: const serialization::__private::PrimitiveTypeSizeChecker>(
    ) -> serialization::__private::fastbuf::Buffer<
        [serialization::__private::SerialSize; { Self::SIZES_LEN }],
    > {
        pub use serialization::__private::fastbuf::WriteBuf;
        serialization::__private::order_sizes_by_repr_and_calc_offset::<
            Self,
            __S,
            { Self::SIZES_LEN },
        >(&[
            <&'static str as serialization::__private::SerialDescriptor>::serial_sizes::<__S>()
                .as_slice(),
        ])
    }
}
impl serialization::__private::FieldPathDrop for A21 {
    fn drop_fields(
        value: &mut core::mem::MaybeUninit<Self>,
        fields: serialization::__private::FieldPath,
    ) {
        let fields = fields.as_slice();
        let A21 { value } = unsafe { value.assume_init_mut() };
        if fields.contains(&(0usize as serialization::__private::FieldIndex)) {
            unsafe {
                (&mut *((value) as *mut _ as *mut core::mem::MaybeUninit<u32>)).assume_init_drop();
            }
        }
    }
}
impl const serialization::__private::FieldPathFinder for A21 {
    fn find_encode<'__a, E: serialization::__private::Encoder>(
        mut path: serialization::__private::FieldPath,
    ) -> &'__a dyn serialization::__private::CompositableEncode<E>
    where
        Self: '__a,
    {
        if path.remaining() == 0 {
            return &serialization::__private::CompositableWrapper::<()>(core::marker::PhantomData);
        }
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            0usize => <&'static str>::find_encode(path),
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
    fn find_decode<'__a, D: serialization::__private::Decoder>(
        mut path: serialization::__private::FieldPath,
    ) -> &'__a dyn serialization::__private::CompositableDecode<D>
    where
        Self: '__a,
    {
        if path.remaining() == 0 {
            return &serialization::__private::CompositableWrapper::<()>(core::marker::PhantomData);
        }
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            0usize => <&'static str>::find_decode(path),
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
    fn calc_offset(mut path: serialization::__private::FieldPath) -> usize {
        if path.remaining() == 0 {
            return 0;
        }
        let value = core::mem::MaybeUninit::<Self>::uninit();
        let value = unsafe { value.assume_init_ref() };
        let A21 { value } = value;
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            0usize => {
                serialization::__private::offset_of(value, value)
                    + <&'static str as serialization::__private::FieldPathFinder>::calc_offset(path)
            }
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for A21 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "A21", "value", &&self.value)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for A21 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<&'static str>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for A21 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for A21 {
    #[inline]
    fn eq(&self, other: &A21) -> bool {
        self.value == other.value
    }
}
enum A22 {
    A { value: i32 },
    B { value2: u16 },
}
impl serialization::__private::Encode for A22 {
    fn encode<__E: serialization::__private::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> core::result::Result<(), __E::Error> {
        serialization::__private::Encoder::encode_enum_variant_key(
            encoder,
            core::any::type_name::<Self>(),
            match self {
                A22::A { value } => "A",
                A22::B { value2 } => "B",
                #[allow(unreachable_patterns)]
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            },
            #[allow(unreachable_code)]
            match self {
                A22::A { value } => 0,
                A22::B { value2 } => 0 + 1,
                #[allow(unreachable_patterns)]
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            },
        )?;
        match self {
            A22::A { value } => serialization::__private::encode_struct(self, encoder),
            A22::B { value2 } => serialization::__private::encode_struct(self, encoder),
            #[allow(unreachable_patterns)]
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
impl serialization::__private::Decode for A22 {
    fn decode_in_place<__D: serialization::__private::Decoder>(
        decoder: &mut __D,
        out: &mut core::mem::MaybeUninit<Self>,
    ) -> Result<(), __D::Error> {
        match serialization::__private::Decoder::decode_enum(
            decoder,
            core::any::type_name::<Self>(),
        )? {
            serialization::__private::EnumIdentifier::Name(name) => match name {
                "A" => {
                    let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                    let value = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
                    *out = core::mem::MaybeUninit::new(A22::A { value });
                    match unsafe { out.assume_init_mut() } {
                        A22::A { value } => {
                            let value_place: &mut core::mem::MaybeUninit<i32> =
                                unsafe { core::mem::transmute(value) };
                            <__D::TupleDecoder as
                                            serialization::__private::CompositeDecoder>::decode_element::<i32>(tup,
                                        value_place)?;
                        }
                        _ => ::core::panicking::panic("internal error: entered unreachable code"),
                    }
                    <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                }
                "B" => {
                    let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                    let value2 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
                    *out = core::mem::MaybeUninit::new(A22::B { value2 });
                    match unsafe { out.assume_init_mut() } {
                        A22::B { value2 } => {
                            let value_place: &mut core::mem::MaybeUninit<u16> =
                                unsafe { core::mem::transmute(value2) };
                            <__D::TupleDecoder as
                                            serialization::__private::CompositeDecoder>::decode_element::<u16>(tup,
                                        value_place)?;
                        }
                        _ => ::core::panicking::panic("internal error: entered unreachable code"),
                    }
                    <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                }
                #[allow(unreachable_patterns)]
                _ => Err(serialization::__private::DecodeError::invalid_enum_variant_name())?,
            },
            serialization::__private::EnumIdentifier::Index(index) => {
                #[allow(non_upper_case_globals)]
                const A: usize = 0;
                #[allow(non_upper_case_globals)]
                const B: usize = 0 + 1;

                #[allow(non_upper_case_globals)]
                match index {
                    A => {
                        let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                        let value = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
                        *out = core::mem::MaybeUninit::new(A22::A { value });
                        match unsafe { out.assume_init_mut() } {
                            A22::A { value } => {
                                let value_place: &mut core::mem::MaybeUninit<i32> =
                                    unsafe { core::mem::transmute(value) };
                                <__D::TupleDecoder as
                                            serialization::__private::CompositeDecoder>::decode_element::<i32>(tup,
                                        value_place)?;
                            }
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        }
                        <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                    }
                    B => {
                        let tup = serialization::__private::Decoder::decode_tuple(decoder)?;
                        let value2 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
                        *out = core::mem::MaybeUninit::new(A22::B { value2 });
                        match unsafe { out.assume_init_mut() } {
                            A22::B { value2 } => {
                                let value_place: &mut core::mem::MaybeUninit<u16> =
                                    unsafe { core::mem::transmute(value2) };
                                <__D::TupleDecoder as
                                            serialization::__private::CompositeDecoder>::decode_element::<u16>(tup,
                                        value_place)?;
                            }
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        }
                        <__D::TupleDecoder as serialization::__private::CompositeDecoder>::end(tup)
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(serialization::__private::DecodeError::invalid_enum_variant_index())?,
                }
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for A22 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            A22::A { value: __self_0 } => {
                ::core::fmt::Formatter::debug_struct_field1_finish(f, "A", "value", &__self_0)
            }
            A22::B { value2: __self_0 } => {
                ::core::fmt::Formatter::debug_struct_field1_finish(f, "B", "value2", &__self_0)
            }
        }
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for A22 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<i32>;
        let _: ::core::cmp::AssertParamIsEq<u16>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for A22 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for A22 {
    #[inline]
    fn eq(&self, other: &A22) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (A22::A { value: __self_0 }, A22::A { value: __arg1_0 }) => __self_0 == __arg1_0,
                (A22::B { value2: __self_0 }, A22::B { value2: __arg1_0 }) => __self_0 == __arg1_0,
                _ => unsafe { ::core::intrinsics::unreachable() },
            }
    }
}
struct A23 {}
impl serialization::__private::Encode for A23 {
    fn encode<__E: serialization::__private::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> core::result::Result<(), __E::Error> {
        let A23 {} = self;
        {
            serialization::__private::encode_struct(self, encoder)
        }
    }
}
impl serialization::__private::Decode for A23 {
    fn decode_in_place<__D: serialization::__private::Decoder>(
        decoder: &mut __D,
        out: &mut core::mem::MaybeUninit<Self>,
    ) -> Result<(), __D::Error> {
        {
            serialization::__private::decode_struct(decoder, out)
        }
    }
}
impl const serialization::__private::SerialDescriptor for A23 {
    const SIZES_LEN: usize = 0;
    fn serial_sizes<__S: const serialization::__private::PrimitiveTypeSizeChecker>(
    ) -> serialization::__private::fastbuf::Buffer<
        [serialization::__private::SerialSize; { Self::SIZES_LEN }],
    > {
        pub use serialization::__private::fastbuf::WriteBuf;
        serialization::__private::order_sizes_by_repr_and_calc_offset::<
            Self,
            __S,
            { Self::SIZES_LEN },
        >(&[])
    }
}
impl serialization::__private::FieldPathDrop for A23 {
    fn drop_fields(
        value: &mut core::mem::MaybeUninit<Self>,
        fields: serialization::__private::FieldPath,
    ) {
        let fields = fields.as_slice();
        let A23 {} = unsafe { value.assume_init_mut() };
    }
}
impl const serialization::__private::FieldPathFinder for A23 {
    fn find_encode<'__a, E: serialization::__private::Encoder>(
        mut path: serialization::__private::FieldPath,
    ) -> &'__a dyn serialization::__private::CompositableEncode<E>
    where
        Self: '__a,
    {
        if path.remaining() == 0 {
            return &serialization::__private::CompositableWrapper::<()>(core::marker::PhantomData);
        }
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
    fn find_decode<'__a, D: serialization::__private::Decoder>(
        mut path: serialization::__private::FieldPath,
    ) -> &'__a dyn serialization::__private::CompositableDecode<D>
    where
        Self: '__a,
    {
        if path.remaining() == 0 {
            return &serialization::__private::CompositableWrapper::<()>(core::marker::PhantomData);
        }
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
    fn calc_offset(mut path: serialization::__private::FieldPath) -> usize {
        if path.remaining() == 0 {
            return 0;
        }
        let value = core::mem::MaybeUninit::<Self>::uninit();
        let value = unsafe { value.assume_init_ref() };
        let A23 {} = value;
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for A23 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "A23")
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for A23 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for A23 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for A23 {
    #[inline]
    fn eq(&self, other: &A23) -> bool {
        true
    }
}
struct A24 {}
impl serialization::__private::Encode for A24 {
    fn encode<__E: serialization::__private::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> core::result::Result<(), __E::Error> {
        let A24 {} = self;
        {
            serialization::__private::encode_struct(self, encoder)
        }
    }
}
impl serialization::__private::Decode for A24 {
    fn decode_in_place<__D: serialization::__private::Decoder>(
        decoder: &mut __D,
        out: &mut core::mem::MaybeUninit<Self>,
    ) -> Result<(), __D::Error> {
        {
            serialization::__private::decode_struct(decoder, out)
        }
    }
}
impl const serialization::__private::SerialDescriptor for A24 {
    const SIZES_LEN: usize = 0;
    fn serial_sizes<__S: const serialization::__private::PrimitiveTypeSizeChecker>(
    ) -> serialization::__private::fastbuf::Buffer<
        [serialization::__private::SerialSize; { Self::SIZES_LEN }],
    > {
        pub use serialization::__private::fastbuf::WriteBuf;
        serialization::__private::order_sizes_by_repr_and_calc_offset::<
            Self,
            __S,
            { Self::SIZES_LEN },
        >(&[])
    }
}
impl serialization::__private::FieldPathDrop for A24 {
    fn drop_fields(
        value: &mut core::mem::MaybeUninit<Self>,
        fields: serialization::__private::FieldPath,
    ) {
        let fields = fields.as_slice();
        let A24 {} = unsafe { value.assume_init_mut() };
    }
}
impl const serialization::__private::FieldPathFinder for A24 {
    fn find_encode<'__a, E: serialization::__private::Encoder>(
        mut path: serialization::__private::FieldPath,
    ) -> &'__a dyn serialization::__private::CompositableEncode<E>
    where
        Self: '__a,
    {
        if path.remaining() == 0 {
            return &serialization::__private::CompositableWrapper::<()>(core::marker::PhantomData);
        }
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
    fn find_decode<'__a, D: serialization::__private::Decoder>(
        mut path: serialization::__private::FieldPath,
    ) -> &'__a dyn serialization::__private::CompositableDecode<D>
    where
        Self: '__a,
    {
        if path.remaining() == 0 {
            return &serialization::__private::CompositableWrapper::<()>(core::marker::PhantomData);
        }
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
    fn calc_offset(mut path: serialization::__private::FieldPath) -> usize {
        if path.remaining() == 0 {
            return 0;
        }
        let value = core::mem::MaybeUninit::<Self>::uninit();
        let value = unsafe { value.assume_init_ref() };
        let A24 {} = value;
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for A24 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "A24")
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for A24 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for A24 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for A24 {
    #[inline]
    fn eq(&self, other: &A24) -> bool {
        true
    }
}
struct A25 {}
impl serialization::__private::Encode for A25 {
    fn encode<__E: serialization::__private::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> core::result::Result<(), __E::Error> {
        let A25 {} = self;
        {
            serialization::__private::encode_struct(self, encoder)
        }
    }
}
impl serialization::__private::Decode for A25 {
    fn decode_in_place<__D: serialization::__private::Decoder>(
        decoder: &mut __D,
        out: &mut core::mem::MaybeUninit<Self>,
    ) -> Result<(), __D::Error> {
        {
            serialization::__private::decode_struct(decoder, out)
        }
    }
}
impl const serialization::__private::SerialDescriptor for A25 {
    const SIZES_LEN: usize = 0;
    fn serial_sizes<__S: const serialization::__private::PrimitiveTypeSizeChecker>(
    ) -> serialization::__private::fastbuf::Buffer<
        [serialization::__private::SerialSize; { Self::SIZES_LEN }],
    > {
        pub use serialization::__private::fastbuf::WriteBuf;
        serialization::__private::order_sizes_by_repr_and_calc_offset::<
            Self,
            __S,
            { Self::SIZES_LEN },
        >(&[])
    }
}
impl serialization::__private::FieldPathDrop for A25 {
    fn drop_fields(
        value: &mut core::mem::MaybeUninit<Self>,
        fields: serialization::__private::FieldPath,
    ) {
        let fields = fields.as_slice();
        let A25 {} = unsafe { value.assume_init_mut() };
    }
}
impl const serialization::__private::FieldPathFinder for A25 {
    fn find_encode<'__a, E: serialization::__private::Encoder>(
        mut path: serialization::__private::FieldPath,
    ) -> &'__a dyn serialization::__private::CompositableEncode<E>
    where
        Self: '__a,
    {
        if path.remaining() == 0 {
            return &serialization::__private::CompositableWrapper::<()>(core::marker::PhantomData);
        }
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
    fn find_decode<'__a, D: serialization::__private::Decoder>(
        mut path: serialization::__private::FieldPath,
    ) -> &'__a dyn serialization::__private::CompositableDecode<D>
    where
        Self: '__a,
    {
        if path.remaining() == 0 {
            return &serialization::__private::CompositableWrapper::<()>(core::marker::PhantomData);
        }
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
    fn calc_offset(mut path: serialization::__private::FieldPath) -> usize {
        if path.remaining() == 0 {
            return 0;
        }
        let value = core::mem::MaybeUninit::<Self>::uninit();
        let value = unsafe { value.assume_init_ref() };
        let A25 {} = value;
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for A25 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "A25")
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for A25 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for A25 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for A25 {
    #[inline]
    fn eq(&self, other: &A25) -> bool {
        true
    }
}
struct A26 {}
impl serialization::__private::Encode for A26 {
    fn encode<__E: serialization::__private::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> core::result::Result<(), __E::Error> {
        let A26 {} = self;
        {
            serialization::__private::encode_struct(self, encoder)
        }
    }
}
impl serialization::__private::Decode for A26 {
    fn decode_in_place<__D: serialization::__private::Decoder>(
        decoder: &mut __D,
        out: &mut core::mem::MaybeUninit<Self>,
    ) -> Result<(), __D::Error> {
        {
            serialization::__private::decode_struct(decoder, out)
        }
    }
}
impl const serialization::__private::SerialDescriptor for A26 {
    const SIZES_LEN: usize = 0;
    fn serial_sizes<__S: const serialization::__private::PrimitiveTypeSizeChecker>(
    ) -> serialization::__private::fastbuf::Buffer<
        [serialization::__private::SerialSize; { Self::SIZES_LEN }],
    > {
        pub use serialization::__private::fastbuf::WriteBuf;
        serialization::__private::order_sizes_by_repr_and_calc_offset::<
            Self,
            __S,
            { Self::SIZES_LEN },
        >(&[])
    }
}
impl serialization::__private::FieldPathDrop for A26 {
    fn drop_fields(
        value: &mut core::mem::MaybeUninit<Self>,
        fields: serialization::__private::FieldPath,
    ) {
        let fields = fields.as_slice();
        let A26 {} = unsafe { value.assume_init_mut() };
    }
}
impl const serialization::__private::FieldPathFinder for A26 {
    fn find_encode<'__a, E: serialization::__private::Encoder>(
        mut path: serialization::__private::FieldPath,
    ) -> &'__a dyn serialization::__private::CompositableEncode<E>
    where
        Self: '__a,
    {
        if path.remaining() == 0 {
            return &serialization::__private::CompositableWrapper::<()>(core::marker::PhantomData);
        }
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
    fn find_decode<'__a, D: serialization::__private::Decoder>(
        mut path: serialization::__private::FieldPath,
    ) -> &'__a dyn serialization::__private::CompositableDecode<D>
    where
        Self: '__a,
    {
        if path.remaining() == 0 {
            return &serialization::__private::CompositableWrapper::<()>(core::marker::PhantomData);
        }
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
    fn calc_offset(mut path: serialization::__private::FieldPath) -> usize {
        if path.remaining() == 0 {
            return 0;
        }
        let value = core::mem::MaybeUninit::<Self>::uninit();
        let value = unsafe { value.assume_init_ref() };
        let A26 {} = value;
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for A26 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "A26")
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for A26 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for A26 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for A26 {
    #[inline]
    fn eq(&self, other: &A26) -> bool {
        true
    }
}
struct A27 {
    value: Vec<u8>,
}
impl serialization::__private::Encode for A27 {
    fn encode<__E: serialization::__private::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> core::result::Result<(), __E::Error> {
        let A27 { value } = self;
        {
            serialization::__private::encode_struct(self, encoder)
        }
    }
}
impl serialization::__private::Decode for A27 {
    fn decode_in_place<__D: serialization::__private::Decoder>(
        decoder: &mut __D,
        out: &mut core::mem::MaybeUninit<Self>,
    ) -> Result<(), __D::Error> {
        {
            serialization::__private::decode_struct(decoder, out)
        }
    }
}
impl const serialization::__private::SerialDescriptor for A27 {
    const SIZES_LEN: usize = 0 + <Vec<u8> as serialization::__private::SerialDescriptor>::SIZES_LEN;
    fn serial_sizes<__S: const serialization::__private::PrimitiveTypeSizeChecker>(
    ) -> serialization::__private::fastbuf::Buffer<
        [serialization::__private::SerialSize; { Self::SIZES_LEN }],
    > {
        pub use serialization::__private::fastbuf::WriteBuf;
        serialization::__private::order_sizes_by_repr_and_calc_offset::<
            Self,
            __S,
            { Self::SIZES_LEN },
        >(&[
            <Vec<u8> as serialization::__private::SerialDescriptor>::serial_sizes::<__S>()
                .as_slice(),
        ])
    }
}
impl serialization::__private::FieldPathDrop for A27 {
    fn drop_fields(
        value: &mut core::mem::MaybeUninit<Self>,
        fields: serialization::__private::FieldPath,
    ) {
        let fields = fields.as_slice();
        let A27 { value } = unsafe { value.assume_init_mut() };
        if fields.contains(&(0usize as serialization::__private::FieldIndex)) {
            unsafe {
                (&mut *((value) as *mut _ as *mut core::mem::MaybeUninit<u32>)).assume_init_drop();
            }
        }
    }
}
impl const serialization::__private::FieldPathFinder for A27 {
    fn find_encode<'__a, E: serialization::__private::Encoder>(
        mut path: serialization::__private::FieldPath,
    ) -> &'__a dyn serialization::__private::CompositableEncode<E>
    where
        Self: '__a,
    {
        if path.remaining() == 0 {
            return &serialization::__private::CompositableWrapper::<()>(core::marker::PhantomData);
        }
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            0usize => <Vec<u8>>::find_encode(path),
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
    fn find_decode<'__a, D: serialization::__private::Decoder>(
        mut path: serialization::__private::FieldPath,
    ) -> &'__a dyn serialization::__private::CompositableDecode<D>
    where
        Self: '__a,
    {
        if path.remaining() == 0 {
            return &serialization::__private::CompositableWrapper::<()>(core::marker::PhantomData);
        }
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            0usize => <Vec<u8>>::find_decode(path),
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
    fn calc_offset(mut path: serialization::__private::FieldPath) -> usize {
        if path.remaining() == 0 {
            return 0;
        }
        let value = core::mem::MaybeUninit::<Self>::uninit();
        let value = unsafe { value.assume_init_ref() };
        let A27 { value } = value;
        pub use serialization::__private::fastbuf::ReadBuf;
        match path.read(1)[0] as usize {
            0usize => {
                serialization::__private::offset_of(value, value)
                    + <Vec<u8> as serialization::__private::FieldPathFinder>::calc_offset(path)
            }
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for A27 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "A27", "value", &&self.value)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for A27 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<Vec<u8>>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for A27 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for A27 {
    #[inline]
    fn eq(&self, other: &A27) -> bool {
        self.value == other.value
    }
}
