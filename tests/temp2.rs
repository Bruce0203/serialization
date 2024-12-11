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
    ops::{Add, Deref, DerefMut},
};

use constvec::ConstVec;
use fastbuf::{Buf, Buffer, WriteBuf};
use serialization::{
    BinaryDecoder, BinaryEncoder, CompositeDecoder, CompositeEncoder, Decode, DecodeError, Decoder,
    Encode, EncodeError, Encoder,
};
use serialization_minecraft::{PacketDecoder, PacketEncoder};

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Foo {
    field1: u32,
    field2: u16,
    field3: Bar,
    field4: u32,
}

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Bar {
    field1: u8,
    field2: u16,
    field3: u32,
}

impl Encode for Bar {
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), E::Error> {
        if Self::fields().as_slice()[0] == SerialSize::unsized_of::<Self>() {
            let mut struc = encoder.encode_struct()?;
            struc.encode_element(&self.field1)?;
            struc.encode_element(&self.field2)?;
            struc.encode_element(&self.field3)?;
            struc.end()?;
            Ok(())
        } else {
            encode2(self, encoder)
        }
    }
}

impl<'de> Decode<'de> for Bar {
    fn decode<D: Decoder<'de>>(decoder: D) -> Result<Self, D::Error> {
        if Self::fields().as_slice()[0] == SerialSize::unsized_of::<Self>() {
            let mut struc = decoder.decode_struct()?;
            let result = Self {
                field1: struc.decode_element()?,
                field2: struc.decode_element()?,
                field3: struc.decode_element()?,
            };
            struc.end()?;
            Ok(result)
        } else {
            decode2(decoder)
        }
    }
}

impl EncodeField for Bar {
    fn encode_field<E: Encoder>(&self, fields: &Fields, encoder: E) -> Result<(), E::Error> {
        if fields.len() == 0 {
            self.encode(encoder)
        } else {
            let mut fields = fields.clone();
            match fields.pop_last() {
                0 => self.field1.encode_field(&fields, encoder),
                1 => self.field2.encode_field(&fields, encoder),
                2 => self.field3.encode_field(&fields, encoder),
                _ => unreachable!(),
            }
        }
    }
}
impl EncodeField for Foo {
    fn encode_field<E: Encoder>(&self, fields: &Fields, encoder: E) -> Result<(), E::Error> {
        if fields.len() == 0 {
            self.encode(encoder)
        } else {
            let mut fields = fields.clone();
            match fields.pop_last() {
                0 => self.field1.encode_field(&fields, encoder),
                1 => self.field2.encode_field(&fields, encoder),
                2 => self.field3.encode_field(&fields, encoder),
                3 => self.field4.encode_field(&fields, encoder),
                _ => unreachable!(),
            }
        }
    }
}

impl<T: Encode> EncodeField for T {
    default fn encode_field<E: Encoder>(
        &self,
        _field_indexes: &Fields,
        encoder: E,
    ) -> Result<(), E::Error> {
        self.encode(encoder)
    }
}

impl<'de, T: Decode<'de>> DecodeField<'de> for T {
    default unsafe fn decode_field<D: CompositeDecoder<'de>>(
        _field_indexes: &Fields,
        decoder: &mut D,
    ) -> Result<ReadableField<Self>, D::Error> {
        Ok(ReadableField {
            offset: 0,
            len: size_of::<Self>(),
            value: decoder.decode_element()?,
        })
    }
}

fn calc_field_offset<T, F>(base_ptr: &T, field_ptr: &F) -> usize {
    let base_ptr = base_ptr as *const _ as *const u8;
    unsafe { (field_ptr as *const _ as *const u8).byte_sub_ptr(base_ptr) }
}

impl<'de> DecodeField<'de> for Bar {
    unsafe fn decode_field<D: CompositeDecoder<'de>>(
        field_indexes: &Fields,
        decoder: &mut D,
    ) -> Result<ReadableField<Self>, D::Error> {
        #[allow(invalid_value)]
        let mut result: Self = unsafe { MaybeUninit::uninit().assume_init() };
        Ok(if field_indexes.len() == 0 {
            result = decoder.decode_element()?;
            ReadableField {
                offset: 0,
                len: size_of::<Self>(),
                value: result,
            }
        } else {
            match field_indexes.as_slice()[0] {
                0 => {
                    let ReadableField { offset, len, value } =
                        DecodeField::decode_field(&field_indexes, decoder)?;
                    result.field1 = value;
                    ReadableField {
                        offset: offset + calc_field_offset(&result, &result.field1),
                        len,
                        value: result,
                    }
                }
                1 => {
                    let ReadableField { offset, len, value } =
                        DecodeField::decode_field(&field_indexes, decoder)?;
                    result.field2 = value;
                    ReadableField {
                        offset: offset + calc_field_offset(&result, &result.field2),
                        len,
                        value: result,
                    }
                }
                2 => {
                    let ReadableField { offset, len, value } =
                        DecodeField::decode_field(&field_indexes, decoder)?;
                    result.field3 = value;
                    ReadableField {
                        offset: offset + calc_field_offset(&result, &result.field3),
                        len,
                        value: result,
                    }
                }
                _ => unreachable!(),
            }
        })
    }
}

impl<'de> DecodeField<'de> for Foo {
    unsafe fn decode_field<D: CompositeDecoder<'de>>(
        fields: &Fields,
        decoder: &mut D,
    ) -> Result<ReadableField<Self>, D::Error> {
        #[allow(invalid_value)]
        let mut result: Self = unsafe { MaybeUninit::uninit().assume_init() };
        Ok(if fields.len() == 0 {
            result = decoder.decode_element()?;
            ReadableField {
                offset: 0,
                len: size_of::<Self>(),
                value: result,
            }
        } else {
            let mut fields = fields.clone();
            match fields.pop_last() {
                0 => {
                    let ReadableField { offset, len, value } =
                        DecodeField::decode_field(&fields, decoder)?;
                    result.field1 = value;
                    ReadableField {
                        offset: offset + calc_field_offset(&result, &result.field1),
                        len,
                        value: result,
                    }
                }
                1 => {
                    let ReadableField { offset, len, value } =
                        DecodeField::decode_field(&fields, decoder)?;
                    result.field2 = value;
                    ReadableField {
                        offset: offset + calc_field_offset(&result, &result.field2),
                        len,
                        value: result,
                    }
                }
                2 => {
                    let ReadableField { offset, len, value } =
                        DecodeField::decode_field(&fields, decoder)?;
                    result.field3 = value;
                    ReadableField {
                        offset: offset + calc_field_offset(&result, &result.field3),
                        len,
                        value: result,
                    }
                }
                3 => {
                    let ReadableField { offset, len, value } =
                        DecodeField::decode_field(&fields, decoder)?;
                    result.field4 = value;
                    ReadableField {
                        offset: offset + calc_field_offset(&result, &result.field4),
                        len,
                        value: result,
                    }
                }
                _ => unreachable!(),
            }
        })
    }
}

pub trait EncodeField: Encode {
    fn encode_field<E: Encoder>(&self, field_indexes: &Fields, encoder: E) -> Result<(), E::Error>;
}

pub trait DecodeField<'de>: Sized {
    unsafe fn decode_field<D: CompositeDecoder<'de>>(
        field_indexes: &Fields,
        decoder: &mut D,
    ) -> Result<ReadableField<Self>, D::Error>;
}

#[const_trait]
pub trait SerialDescriptor {
    const N: usize;
    fn fields() -> ConstVec<[SerialSize; Self::N]>;
}

pub type Fields = ConstVec<[u16; 128]>;

#[derive(Eq, PartialEq, Clone, Debug)]
#[repr(u8)]
pub enum SerialSize {
    Unsized { type_id: u128, fields: Fields },
    Padding(usize),
    Sized { start: usize, len: usize },
}

impl SerialSize {
    pub const fn unsized_of<T: 'static>() -> SerialSize {
        SerialSize::Unsized {
            type_id: unsafe { type_id::<T>() },
            fields: ConstVec::new(0, unsafe { MaybeUninit::zeroed().assume_init() }),
        }
    }

    pub const fn clone(&self) -> Self {
        match self {
            SerialSize::Unsized { type_id, fields } => SerialSize::Unsized {
                type_id: *type_id,
                fields: fields.clone(),
            },
            SerialSize::Padding(padding) => SerialSize::Padding(*padding),
            SerialSize::Sized { start, len } => SerialSize::Sized {
                start: *start,
                len: *len,
            },
        }
    }
}

impl<T: 'static> const SerialDescriptor for T {
    default const N: usize = 1;
    default fn fields() -> ConstVec<[SerialSize; Self::N]> {
        ConstVec::new(
            Self::N,
            [const { SerialSize::unsized_of::<Self>() }; Self::N],
        )
    }
}

impl const SerialDescriptor for u32 {
    const N: usize = 1;
    fn fields() -> ConstVec<[SerialSize; Self::N]> {
        ConstVec::new(
            Self::N,
            [SerialSize::Sized {
                start: 0,
                len: size_of::<Self>(),
            }],
        )
    }
}

impl const SerialDescriptor for u16 {
    const N: usize = 1;
    fn fields() -> ConstVec<[SerialSize; Self::N]> {
        ConstVec::new(
            Self::N,
            [SerialSize::Sized {
                start: 0,
                len: size_of::<Self>(),
            }],
        )
    }
}

impl const SerialDescriptor for u64 {
    const N: usize = 1;
    fn fields() -> ConstVec<[SerialSize; Self::N]> {
        ConstVec::new(
            Self::N,
            [SerialSize::Sized {
                start: 0,
                len: size_of::<Self>(),
            }],
        )
    }
}

impl const SerialDescriptor for Foo {
    const N: usize = <u32 as SerialDescriptor>::N
        + <u16 as SerialDescriptor>::N
        + <Bar as SerialDescriptor>::N
        + <u32 as SerialDescriptor>::N
        + 4
        + 1;
    fn fields() -> ConstVec<[SerialSize; Self::N]> {
        compact_fields({
            #[allow(invalid_value)]
            let value: Self = unsafe { MaybeUninit::uninit().assume_init() };
            let mut padding_calc = SizeCalcState::new(&value);
            padding_calc.next_field(&value.field1);
            padding_calc.next_field(&value.field2);
            padding_calc.next_field(&value.field3);
            padding_calc.next_field(&value.field4);
            padding_calc.finish()
        })
    }
}

impl const SerialDescriptor for Bar {
    const N: usize = <u8 as SerialDescriptor>::N
        + <u16 as SerialDescriptor>::N
        + <u32 as SerialDescriptor>::N
        + 3
        + 1;
    fn fields() -> ConstVec<[SerialSize; Self::N]> {
        compact_fields({
            #[allow(invalid_value)]
            let value: Self = unsafe { MaybeUninit::uninit().assume_init() };
            let mut padding_calc = SizeCalcState::new(&value);
            padding_calc.next_field(&value.field1);
            padding_calc.next_field(&value.field2);
            padding_calc.next_field(&value.field3);
            padding_calc.finish()
        })
    }
}

pub struct SizeCalcState<'a, T: SerialDescriptor>
where
    [(); T::N]:,
    [(); size_of::<T>()]:,
{
    temp: ConstVec<[SerialSize; T::N]>,
    value: &'a T,
    cursor: usize,
    counter: usize,
    board: [usize; size_of::<T>()],
}

impl<'a, T: SerialDescriptor> SizeCalcState<'a, T>
where
    [(); T::N]:,
    [(); size_of::<T>()]:,
{
    const PADDING_ID: usize = usize::MAX;
    pub const fn new(value: &'a T) -> Self {
        Self {
            temp: ConstVec::new(0, unsafe { MaybeUninit::uninit().assume_init() }),
            value,
            cursor: 0,
            counter: 0,
            board: [Self::PADDING_ID; _],
        }
    }

    pub const fn next_field<E: const SerialDescriptor>(&mut self, field_ptr: &E)
    where
        [(); T::N]:,
        [(); <E as SerialDescriptor>::N]:,
    {
        let offset = unsafe { (field_ptr as *const E).byte_sub_ptr(self.value as *const T) };
        let size = size_of::<E>();
        let mut i = offset;
        while i < offset + size {
            self.board[i] = self.counter;
            i += 1;
        }
        let mut slice = <E as SerialDescriptor>::fields().clone();
        let mut i = 0;

        while i < slice.len() {
            match slice.as_slice()[i].clone() {
                SerialSize::Unsized { type_id, fields } => {
                    let mut fields = fields.clone();
                    fields.push(&(self.counter as u16));
                    *slice.get_mut(i) = SerialSize::Unsized {
                        type_id,
                        fields: fields,
                    };
                }
                _ => {}
            }
            i += 1;
        }
        self.temp.push(&SerialSize::Padding(slice.len()));
        self.temp.append(&slice);
        self.counter += 1;
    }

    pub const fn finish(mut self) -> ConstVec<[SerialSize; T::N]>
    where
        [(); T::N]:,
    {
        let mut result: ConstVec<[SerialSize; T::N]> =
            ConstVec::new(0, unsafe { MaybeUninit::uninit().assume_init() });
        let mut i = 0;
        while i < self.board.len() {
            let field_index = self.board[i];
            if field_index == Self::PADDING_ID {
                let mut padding = 0;
                while i < self.board.len() {
                    let field_index = self.board[i];
                    if field_index != Self::PADDING_ID {
                        break;
                    }
                    padding += 1;
                    i += 1;
                }
                i -= 1;
                let v = SerialSize::Padding(padding);
                result.push(&v);
            } else {
                let mut j = 0;
                let mut k = 0;
                while j < self.temp.len() {
                    let v = self.temp.as_slice()[j].clone();
                    let fields_len = match v {
                        SerialSize::Padding(size) => size,
                        _ => unreachable!(),
                    };
                    if field_index == k {
                        j += 1;
                        let repeat = j + fields_len;
                        while j < repeat {
                            result.push(&match &self.temp.as_slice()[j] {
                                SerialSize::Sized { start, len } => SerialSize::Sized {
                                    start: i + *start,
                                    len: *len,
                                },
                                size => size.clone(),
                            });
                            j += 1;
                        }
                        loop {
                            i += 1;
                            if i >= self.board.len() || self.board[i] != field_index {
                                i -= 1;
                                break;
                            }
                        }
                        break;
                    } else {
                        k += 1;
                        j += fields_len + 1;
                    }
                }
            }
            i += 1;
        }
        // if 0 < offset_from_cur {
        //     self.result.push(&SerialSize::Padding(offset_from_cur));
        // }
        // let mut slice = <E as SerialDescriptor>::FLATTEN_FIELDS.clone();
        // let mut i = 0;
        // while i < slice.len() {
        //     match slice.1[i] {
        //         SerialSize::Sized { start: _, len } => {
        //             slice.1[i] = SerialSize::Sized {
        //                 start: offset_from_cur + self.cursor,
        //                 len: len,
        //             }
        //         }
        //         _ => {}
        //     }
        //     i += 1;
        // }
        // self.result.append(&slice);
        // self.cursor += offset_from_cur + size_of::<E>();
        // self.counter += 1;

        let last_padding = size_of::<T>() - self.cursor;
        if last_padding > 0 {
            self.temp.push(&SerialSize::Padding(last_padding));
        }
        result
    }
}

pub struct WritableField<'a, T: EncodeField> {
    value: &'a T,
    fields: Fields,
}

pub struct ReadableField<T> {
    offset: usize,
    len: usize,
    value: T,
}

impl<'a, T: EncodeField> Encode for WritableField<'a, T> {
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), E::Error> {
        self.value.encode_field(&self.fields, encoder)
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

fn encode2<T: SerialDescriptor + Encode, E: Encoder>(value: &T, encoder: E) -> Result<(), E::Error>
where
    [(); T::N]:,
    [(); size_of::<T>()]:,
{
    let mut i = 0;
    let mut tup = encoder.encode_tuple()?;
    let fields = T::fields();
    while i < fields.len() {
        match &fields.as_slice()[i] {
            SerialSize::Unsized { type_id: _, fields } => {
                tup.encode_element(&WritableField {
                    value,
                    fields: fields.clone(),
                })?;
            }
            SerialSize::Padding(size) => {
                tup.skip_bytes(*size);
            }
            SerialSize::Sized { start, len: size } => {
                let slice: *const u8 = unsafe { transmute(value) };
                let start = unsafe { slice.byte_add(*start) };
                let value = unsafe { slice::from_raw_parts(start, *size) };
                tup.encode_element(&WritingBytes(value))?;
            }
        }
        i += 1;
    }
    tup.end()?;
    Ok(())
}

impl Encode for Foo {
    fn encode<E: Encoder>(&self, encoder: E) -> Result<(), E::Error> {
        if Self::fields().as_slice()[0] == SerialSize::unsized_of::<Self>() {
            let mut struc = encoder.encode_struct()?;
            struc.encode_element(&self.field1)?;
            struc.encode_element(&self.field2)?;
            struc.encode_element(&self.field3)?;
            struc.encode_element(&self.field4)?;
            struc.end()?;
            return Ok(());
        }
        encode2(self, encoder)
    }
}

fn decode2<'de, T: Sized + SerialDescriptor + Decode<'de>, D: Decoder<'de>>(
    decoder: D,
) -> Result<T, D::Error>
where
    [(); T::N]:,
    [(); size_of::<T>()]:,
{
    #[allow(invalid_value)]
    let mut result = Buffer::<{ size_of::<T>() }>::new();
    let mut i = 0;
    let mut tup = decoder.decode_tuple()?;
    let fields = T::fields();
    while i < fields.len() {
        match &fields.as_slice()[i] {
            SerialSize::Unsized { type_id: _, fields } => {
                let ReadableField { offset, len, value } =
                    unsafe { T::decode_field(&fields, &mut tup)? };
                result.write(unsafe {
                    slice::from_raw_parts((&value as *const _ as *const u8).byte_add(offset), len)
                });
            }
            SerialSize::Padding(size) => {
                result.advance(*size);
            }
            SerialSize::Sized {
                start: _,
                len: size,
            } => {
                result.write(
                    tup.read_bytes(*size)
                        .map_err(|()| DecodeError::not_enough_bytes_in_the_buffer())?,
                );
            }
        }
        i += 1;
    }
    tup.end()?;
    Ok(unsafe { const_transmute(*result.to_slice()) })
}

impl<'de> Decode<'de> for Foo {
    fn decode<D: Decoder<'de>>(decoder: D) -> Result<Self, D::Error> {
        let fields = Self::fields();
        if fields.as_slice()[0] == SerialSize::unsized_of::<Self>() {
            let mut struc = decoder.decode_struct()?;
            let result = Self {
                field1: struc.decode_element()?,
                field2: struc.decode_element()?,
                field3: struc.decode_element()?,
                field4: struc.decode_element()?,
            };
            struc.end()?;
            return Ok(result);
        }
        decode2(decoder)
    }
}

const fn compact_fields<const N: usize>(
    fields: ConstVec<[SerialSize; N]>,
) -> ConstVec<[SerialSize; N]> {
    let mut result: [SerialSize; N] = [const { SerialSize::Sized { start: 0, len: 0 } }; N];
    let mut result_i = 0;
    let mut slice = fields.clone();
    loop {
        let mut j = 0;
        let mut size: Option<SerialSize> = None;
        while j < slice.as_slice().len() {
            let size_clone = if let Some(ref size) = &size {
                Some(size.clone())
            } else {
                None
            };
            match (size_clone, slice.as_slice()[j].clone()) {
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
        if j == slice.as_slice().len() {
            break;
        }
        slice = slice.cutoff_front_at(j);
    }
    ConstVec::new(result_i, result)
}

#[test]
fn clone_test() {
    let arr = ConstVec::new(5, [1, 2, 3, 4, 5]);
    assert_eq!(arr.clone(), ConstVec::new(5, [1, 2, 3, 4, 5]));
    let arr = ConstVec::new(0, [0i32; 0]);
    assert_eq!(arr.clone(), ConstVec::new(0, []));
    let arr = ConstVec::new(1, [0i32; 1]);
    assert_eq!(arr.clone(), ConstVec::new(1, [0]));
}

#[test]
fn cutoff_front_at() {
    {
        let arr = ConstVec::new(5, [1, 2, 3, 4, 5]);
        assert!(arr.cutoff_front_at(1) == ConstVec::<[_; 5]>::new(4, [2, 3, 4, 5, 0]));
        assert!(arr.cutoff_front_at(4) == ConstVec::<[_; 5]>::new(1, [5, 3, 4, 5, 0]));
        assert!(arr.cutoff_front_at(5) == ConstVec::<[_; 5]>::new(0, [5, 3, 4, 5, 0]));
        assert!(arr.cutoff_front_at(0) == ConstVec::<[_; 5]>::new(5, [1, 2, 3, 4, 5]));
        let arr = ConstVec::new(0, [0; 5]);
        assert_eq!(arr.cutoff_front_at(0), ConstVec::<[_; 5]>::new(0, [0; 5]));
        let arr = ConstVec::new(1, [0; 5]);
        assert_eq!(arr.cutoff_front_at(0), ConstVec::<[_; 5]>::new(1, [0; 5]));
        let arr = ConstVec::new(1, [0; 5]);
        assert_eq!(arr.cutoff_front_at(1), ConstVec::<[_; 5]>::new(0, [0; 5]));
        let arr = ConstVec::new(2, [0; 5]);
        assert_eq!(arr.cutoff_front_at(1), ConstVec::<[_; 5]>::new(1, [0; 5]));
        let arr = ConstVec::new(4, [0; 5]);
        assert_eq!(arr.cutoff_front_at(4), ConstVec::<[_; 5]>::new(0, [0; 5]));
    }
    assert_eq!([1, 2, 3, 4, 5].split_at(1).1, [2, 3, 4, 5]);
    assert_eq!([1, 2, 3, 4, 5].split_at(5).1, []);
    assert_eq!([0; 0].split_at(0).1, []);
    assert_eq!([0; 1].split_at(0).1, [0]);
}

#[test]
fn test() {
    println!("{:?}", Foo::fields());
    let foo = Foo {
        field1: 12,
        field2: 23,
        field3: Bar {
            field1: 11,
            field2: 22,
            field3: 33,
        },
        field4: 45,
    };
    {
        let temp = unsafe { transmute::<_, &[u8; size_of::<Foo>()]>(&foo) };
        println!("foo = {:?}, len = {}", temp, temp.len());
    }
    let mut buf = Buffer::<1000>::new();
    {
        let slice: &mut [u8; 1000] = unsafe { const_transmute(buf.to_slice_mut()) };
        *slice = [0; _];
    }
    let ref mut encoder = PacketEncoder::new(&mut buf);
    black_box(foo.encode(encoder)).unwrap();
    println!("buf = {buf:?}");
    let ref mut decoder = PacketDecoder::new(&mut buf);
    let decoded = Foo::decode(decoder).unwrap();
    println!("dec = {buf:?}");
    println!("res = {:?}", unsafe {
        transmute::<_, &[u8; size_of::<Foo>()]>(&decoded)
    });
    assert_eq!(decoded, foo);
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

#[test]
fn temp() {
    #[allow(invalid_value)]
    let value: Bar = unsafe { MaybeUninit::uninit().assume_init() };
    let mut padding_calc = SizeCalcState::new(&value);
    padding_calc.next_field(&value.field1);
    padding_calc.next_field(&value.field2);
    padding_calc.next_field(&value.field3);
    println!("{:?}", padding_calc.board);
    println!("{:?}", padding_calc.finish());
    println!("{}", u8::to_be(11_u8));
}
