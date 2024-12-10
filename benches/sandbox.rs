#![feature(const_copy_from_slice)]
#![feature(core_intrinsics)]
#![feature(const_ptr_sub_ptr)]
#![feature(ptr_sub_ptr)]
#![feature(generic_arg_infer)]
#![feature(const_try)]
#![feature(const_type_id)]
#![feature(const_trait_impl)]
#![feature(generic_const_items)]
#![feature(generic_const_exprs)]
#![feature(const_for)]
#![feature(inline_const_pat)]
#![feature(negative_impls)]
#![feature(specialization)]
#![feature(trivial_bounds)]
#![feature(auto_traits)]
#![feature(min_specialization)]

use core::slice;
use std::{
    hint::black_box,
    intrinsics::type_id,
    mem::{transmute, MaybeUninit},
    ops::Add,
};

use divan::{bench, Bencher};
use fastbuf::{Buf, Buffer};
use serialization::{
    BinaryEncoder, CheckPrimitiveTypeSize, CompositeDecoder, CompositeEncoder, Decode, Decoder,
    Encode, EncodeError, Encoder,
};
use serialization_minecraft::PacketEncoder;

#[derive(Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct Foo {
    field1: u32,
    field2: u16,
    field3: u8,
    field4: u32,
}

impl Encode for Foo {
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), E::Error> {
        let mut struc = encoder.encode_struct()?;
        struc.encode_element(&self.field1)?;
        struc.encode_element(&self.field2)?;
        struc.encode_element(&self.field3)?;
        struc.encode_element(&self.field4)?;
        struc.end()?;
        Ok(())
    }
}

impl EncodeField for Foo {
    fn encode_field<E: Encoder>(
        &self,
        field_indexes: &[usize],
        encoder: E,
    ) -> Result<(), E::Error> {
        if field_indexes.is_empty() {
            self.encode(encoder)
        } else if field_indexes.len() == 1 {
            match field_indexes[0] {
                0 => self.field1.encode(encoder),
                1 => self.field2.encode(encoder),
                2 => self.field3.encode(encoder),
                3 => self.field4.encode(encoder),
                _ => unreachable!(),
            }
        } else {
            match field_indexes[0] {
                0 => self.field1.encode_field(&field_indexes[1..], encoder),
                1 => self.field2.encode_field(&field_indexes[1..], encoder),
                2 => self.field3.encode_field(&field_indexes[1..], encoder),
                3 => self.field4.encode_field(&field_indexes[1..], encoder),
                _ => unreachable!(),
            }
        }
    }
}

impl<T: Encode> EncodeField for T {
    default fn encode_field<E: Encoder>(
        &self,
        _field_indexes: &[usize],
        encoder: E,
    ) -> Result<(), E::Error> {
        self.encode(encoder)
    }
}

impl<'de, T: Decode<'de>> DecodeField<'de> for T {
    default unsafe fn decode_field<D: Decoder<'de>>(
        _field_indexes: &[usize],
        decoder: D,
    ) -> Result<Self, D::Error> {
        T::decode(decoder)
    }
}

impl<'de> Decode<'de> for Foo {
    fn decode<D: Decoder<'de>>(decoder: D) -> Result<Self, D::Error> {
        let mut struc = decoder.decode_struct()?;
        let result = Self {
            field1: struc.decode_element()?,
            field2: struc.decode_element()?,
            field3: struc.decode_element()?,
            field4: struc.decode_element()?,
        };
        struc.end()?;
        Ok(result)
    }
}

impl<'de> DecodeField<'de> for Foo {
    unsafe fn decode_field<D: Decoder<'de>>(
        field_indexes: &[usize],
        decoder: D,
    ) -> Result<Self, D::Error> {
        #[allow(invalid_value)]
        let mut result: Self = unsafe { MaybeUninit::uninit().assume_init() };
        if field_indexes.is_empty() {
            result = Self::decode(decoder)?;
        } else if field_indexes.len() == 1 {
            match field_indexes[0] {
                0 => {
                    result.field1 = u32::decode(decoder)?;
                }
                1 => {
                    result.field2 = u16::decode(decoder)?;
                }
                2 => {
                    result.field3 = u8::decode(decoder)?;
                }
                3 => {
                    result.field4 = u32::decode(decoder)?;
                }
                _ => unreachable!(),
            }
        } else {
            match field_indexes[0] {
                0 => {
                    result.field1 = u32::decode_field(&field_indexes[1..], decoder)?;
                }
                1 => {
                    result.field2 = u16::decode_field(&field_indexes[1..], decoder)?;
                }
                2 => {
                    result.field3 = u8::decode_field(&field_indexes[1..], decoder)?;
                }
                3 => {
                    result.field4 = u32::decode_field(&field_indexes[1..], decoder)?;
                }
                _ => unreachable!(),
            }
        }
        Ok(result)
    }
}

#[derive(Debug)]
pub struct ArrayVec<T>(usize, T);

impl<T, const N: usize> ArrayVec<[T; N]> {
    pub const fn new(size: usize, arr: [T; N]) -> Self {
        Self(size, arr)
    }

    pub const fn len(&self) -> usize {
        self.0
    }

    pub const fn push(&mut self, value: &T)
    where
        [(); size_of::<T>()]:,
    {
        if self.0 >= N {
            panic!("not enough remaining buffer");
        } else {
            let slice: &mut [[u8; size_of::<T>()]; N] = unsafe { const_transmute(&mut self.1) };
            let value: &mut [u8; size_of::<T>()] = unsafe { const_transmute(value) };
            slice[self.0] = *value;
            self.0 += 1;
        }
    }

    pub const fn append<const N2: usize>(&mut self, target: &ArrayVec<[T; N2]>)
    where
        [(); size_of::<T>()]:,
    {
        if target.len() >= N - self.len() {
            panic!("not enough remaining buffer");
        } else {
            let mut i = 0;
            while i < target.len() {
                self.push(&target.1[i]);
                i += 1;
            }
        }
    }

    pub const fn clone(&self) -> Self
    where
        [(); size_of::<T>()]:,
    {
        #[allow(invalid_value)]
        let mut result: Self = unsafe { MaybeUninit::uninit().assume_init() };
        result.0 = self.0;
        let dst: &mut [[u8; size_of::<T>()]; N] = unsafe { const_transmute(&mut result.1) };
        let src: &[[u8; size_of::<T>()]; N] = unsafe { const_transmute(&self.1) };
        dst.copy_from_slice(src);
        result
    }
}

impl<T: Sized + Copy, const N1: usize, const N2: usize> const Add<ArrayVec<[T; N2]>>
    for ArrayVec<[T; N1]>
where
    [(); size_of::<T>()]:,
    [(); N1 + N2]:,
{
    type Output = ArrayVec<[T; N1 + N2]>;

    fn add(self, rhs: ArrayVec<[T; N2]>) -> Self::Output {
        let slice1 = self;
        let slice2 = rhs;
        let mut slice: [[u8; size_of::<T>()]; N1 + N2] =
            unsafe { MaybeUninit::uninit().assume_init() };
        {
            let mut i = 0;
            while i < slice1.0 {
                slice[i] = unsafe { const_transmute(slice1.1[i]) };
                i += 1;
            }
        }
        {
            let mut i = 0;
            while i < slice2.0 {
                slice[i + slice1.0] = unsafe { const_transmute(slice2.1[i]) };
                i += 1;
            }
        }
        ArrayVec(slice1.0 + slice2.0, unsafe { const_transmute(slice) })
    }
}

pub trait EncodeField: Encode {
    fn encode_field<E: Encoder>(&self, field_indexes: &[usize], encoder: E)
        -> Result<(), E::Error>;
}

pub trait DecodeField<'de>: Sized {
    unsafe fn decode_field<D: Decoder<'de>>(
        field_indexes: &[usize],
        decoder: D,
    ) -> Result<Self, D::Error>;
}

pub trait SerialDescriptor {
    const N: usize;
    const FLATTEN_FIELDS: ArrayVec<[SerialSize; Self::N]> where [(); Self::N ]:;
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[repr(u8)]
pub enum SerialSize {
    Unsized {
        type_id: u128,
        fields: &'static [usize],
    },
    Padding(usize),
    Sized {
        start: usize,
        len: usize,
    },
}

impl SerialSize {
    pub const fn unsized_of<T: 'static>() -> SerialSize {
        SerialSize::Unsized {
            type_id: type_id::<T>(),
            fields: &[],
        }
    }
}

impl<T: 'static> SerialDescriptor for T {
    default const N: usize = 1;
    default const FLATTEN_FIELDS: ArrayVec<[SerialSize; Self::N]> =
        ArrayVec::new(Self::N, [SerialSize::unsized_of::<Self>(); Self::N]) where [SerialSize; Self::N]:;
}

impl SerialDescriptor for u32 {
    const N: usize = 1;
    const FLATTEN_FIELDS: ArrayVec<[SerialSize; Self::N]> = ArrayVec::new(
        Self::N,
        [SerialSize::Sized {
            start: 0,
            len: size_of::<Self>(),
        }],
    );
}

impl SerialDescriptor for u16 {
    const N: usize = 1;
    const FLATTEN_FIELDS: ArrayVec<[SerialSize; Self::N]> = ArrayVec::new(
        Self::N,
        [SerialSize::Sized {
            start: 0,
            len: size_of::<Self>(),
        }],
    );
}

impl SerialDescriptor for u8 {
    const N: usize = 1;
    const FLATTEN_FIELDS: ArrayVec<[SerialSize; Self::N]> = ArrayVec::new(
        Self::N,
        [SerialSize::Sized {
            start: 0,
            len: size_of::<Self>(),
        }],
    );
}

impl SerialDescriptor for Foo {
    const N: usize = <u32 as SerialDescriptor>::N
        + <u16 as SerialDescriptor>::N
        + <u8 as SerialDescriptor>::N
        + <u32 as SerialDescriptor>::N
        + 4
        + 1;
    const FLATTEN_FIELDS: ArrayVec<[SerialSize; Self::N]> = compact_fields(Foo::padding_inserted());
}

pub struct PaddingCalcState<'a, T: SerialDescriptor>
where
    [(); T::N]:,
{
    result: ArrayVec<[SerialSize; T::N]>,
    value: &'a T,
    cur: usize,
}

impl<'a, T: SerialDescriptor> PaddingCalcState<'a, T>
where
    [(); T::N]:,
{
    pub const fn new(value: &'a T) -> Self {
        Self {
            result: ArrayVec::new(0, unsafe { MaybeUninit::uninit().assume_init() }),
            value,
            cur: 0,
        }
    }

    pub const fn next_field<E: SerialDescriptor>(&mut self, field_ptr: &E)
    where
        [(); T::N]:,
        [(); <E as SerialDescriptor>::N]:,
    {
        let offset_from_cur =
            unsafe { (field_ptr as *const E).byte_sub_ptr(self.value as *const T) } - self.cur;
        if 0 < offset_from_cur {
            self.result.push(&SerialSize::Padding(offset_from_cur));
        }
        let mut slice = <E as SerialDescriptor>::FLATTEN_FIELDS.clone();
        let mut i = 0;
        while i < slice.len() {
            match slice.1[i] {
                SerialSize::Sized { start: _, len } => {
                    slice.1[i] = SerialSize::Sized {
                        start: offset_from_cur + self.cur,
                        len: len,
                    }
                }
                _ => {}
            }
            i += 1;
        }
        self.result.append(&slice);
        self.cur += offset_from_cur + size_of::<E>();
    }

    pub const fn finish(mut self) -> ArrayVec<[SerialSize; T::N]>
    where
        [(); T::N]:,
    {
        let last_padding = size_of::<T>() - self.cur;
        if last_padding > 0 {
            self.result.push(&SerialSize::Padding(last_padding));
        }
        self.result
    }
}

impl Foo {
    pub const fn padding_inserted() -> ArrayVec<[SerialSize; Self::N]> {
        #[allow(invalid_value)]
        let value: Self = unsafe { MaybeUninit::uninit().assume_init() };
        let mut padding_calc = PaddingCalcState::new(&value);
        padding_calc.next_field(&value.field1);
        padding_calc.next_field(&value.field2);
        padding_calc.next_field(&value.field3);
        padding_calc.next_field(&value.field4);
        padding_calc.finish()
    }
}

pub struct Field<'a, T: EncodeField> {
    value: &'a T,
    fields: &'static [usize],
}

impl<'a, T: EncodeField> Encode for Field<'a, T> {
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), E::Error> {
        self.value.encode_field(&self.fields, encoder)
    }
}

pub struct ReadablePadding(usize);

impl Encode for ReadablePadding {
    fn encode<E: Encoder>(&self, mut encoder: E) -> Result<(), E::Error> {
        encoder.skip_bytes(self.0);
        Ok(())
    }
}

pub struct WritingBytes<'a>(&'a [u8]);

impl Encode for WritingBytes<'_> {
    fn encode<E: Encoder>(&self, mut encoder: E) -> Result<(), E::Error> {
        encoder
            .write_bytes(self.0)
            .map_err(|()| EncodeError::not_enough_bytes_in_the_buffer())?;
        Ok(())
    }
}

impl Foo {
    fn encode2<E: Encoder + CheckPrimitiveTypeSize + BinaryEncoder>(
        &self,
        encoder: E,
    ) -> Result<(), E::Error> {
        let mut i = 0;
        let mut tup = encoder.encode_tuple()?;
        while i < Self::FLATTEN_FIELDS.len() {
            match Self::FLATTEN_FIELDS.1[i] {
                SerialSize::Unsized { type_id: _, fields } => {
                    tup.encode_element(&Field {
                        value: self,
                        fields,
                    })?;
                }
                SerialSize::Padding(_) => {}
                SerialSize::Sized { start, len: size } => {
                    let slice: *const u8 = unsafe { transmute(self) };
                    let start = unsafe { slice.byte_add(start) };
                    let value = unsafe { slice::from_raw_parts(start, size) };
                    tup.encode_element(&WritingBytes(value))?;
                }
            }
            i += 1;
        }
        tup.end()?;
        Ok(())
    }

    fn decode2<'de, D: Decoder<'de>>(mut decoder: D) -> Result<Self, D::Error> {
        #[allow(invalid_value)]
        let result = Buffer::<{ size_of::<Self>() }>::new();
        let mut i = 0;
        while i < Self::FLATTEN_FIELDS.len() {
            match Self::FLATTEN_FIELDS.1[i] {
                SerialSize::Unsized { type_id: _, fields } => {

                }
                SerialSize::Padding(size) => {
                }
                SerialSize::Sized { start, len: size } => {}
            }
            i += 1;
        }
        Ok(unsafe { transmute(*result.to_slice()) })
    }
}

const fn compact_fields<const N: usize>(
    fields: ArrayVec<[SerialSize; N]>,
) -> ArrayVec<[SerialSize; N]> {
    let mut result: [SerialSize; N] = [SerialSize::Sized { start: 0, len: 0 }; N];
    let mut result_i = 0;
    let mut slice: &[SerialSize] = &fields.1.split_at(fields.0).0;
    loop {
        let mut j = 0;
        let mut size = None;
        while j < slice.len() {
            match (size, slice[j]) {
                (
                    Some(SerialSize::Sized {
                        start,
                        len: origin_size,
                    }),
                    SerialSize::Sized {
                        start: _,
                        len: adder,
                    },
                ) => {
                    size = Some(SerialSize::Sized {
                        start,
                        len: origin_size + adder,
                    });
                }
                (Some(SerialSize::Padding(origin_size)), SerialSize::Padding(adder)) => {
                    size = Some(SerialSize::Padding(origin_size + adder));
                }
                (Some(SerialSize::Sized { .. }), SerialSize::Padding(new_size)) if j == 0 => {
                    size = Some(SerialSize::Padding(new_size));
                }
                (Some(SerialSize::Sized { .. }), SerialSize::Padding(_))
                | (Some(SerialSize::Padding(_)), SerialSize::Sized { .. })
                | (Some(_), SerialSize::Unsized { .. }) => {
                    break;
                }
                (None, SerialSize::Unsized { type_id, fields }) => {
                    size = Some(SerialSize::Unsized { type_id, fields });
                    j += 1;
                    break;
                }
                (_origin_size, new_size) => {
                    size = Some(new_size);
                }
            };
            j += 1;
        }
        result[result_i] = size.unwrap();
        result_i += 1;
        if j == slice.len() {
            break;
        }
        slice = slice.split_at(j).1;
    }
    // match Self::BINARY_SIZED {
    //     BinarySized::Sized => {
    //         println!("BinarySized encoding");
    //         type T = Foo;
    //         let ref slice =
    //             unsafe { slice::from_raw_parts(self as *const T as *const u8, size_of::<T>()) };
    //         Encoder::encode_bytes(&mut encoder, slice)?;
    //     }
    //     BinarySized::UnsizedOrUnknown => {
    //         println!("BinaryUnsized encoding");
    //         let mut struc = encoder.encode_struct()?;
    //         struc.encode_element(&self.field1)?;
    //         struc.encode_element(&self.field2)?;
    //         struc.end()?;
    //     }
    // }
    ArrayVec::new(result_i, result)
}

#[bench]
fn test(bencher: Bencher) {
    let foo = Foo {
        field1: 123,
        field2: 234,
        field3: 124,
        field4: 50,
    };
    let mut buf = Buffer::<1000>::new();
    bencher.bench_local(|| {
        let ref mut encoder = PacketEncoder::new(&mut buf);
        let _result = black_box(foo.encode2(encoder));
        unsafe { buf.set_filled_pos(0) };
    });
}

pub const unsafe fn const_transmute<A, B>(a: A) -> B {
    if std::mem::size_of::<A>() != std::mem::size_of::<B>() {
        panic!("Size mismatch for generic_array::const_transmute");
    }

    #[repr(C)]
    union Union<A, B> {
        a: std::mem::ManuallyDrop<A>,
        b: std::mem::ManuallyDrop<B>,
    }

    let a = std::mem::ManuallyDrop::new(a);
    std::mem::ManuallyDrop::into_inner(Union { a }.b)
}

fn main() {
    divan::main();
}
