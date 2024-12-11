use core::slice;
use std::mem::{transmute, MaybeUninit};

use constvec::ConstVec;
use fastbuf::{Buf, Buffer, WriteBuf};

use crate::{
    BinaryDecoder, BinaryEncoder, CheckPrimitiveTypeSize, CompositeDecoder, CompositeEncoder,
    Decode, DecodeError, Decoder, Encode, EncodeError, Encoder,
};

pub trait EncodeField: Encode {
    fn encode_field<E: Encoder>(&self, field_indexes: &Fields, encoder: E) -> Result<(), E::Error>;
}

pub trait DecodeField<'de>: Sized {
    unsafe fn decode_field<D: CompositeDecoder<'de>>(
        fields: &Fields,
        decoder: &mut D,
    ) -> Result<ReadableField<Self>, D::Error>;
}

#[const_trait]
pub trait SerialDescriptor {
    const N: usize;
    fn fields<C: const CheckPrimitiveTypeSize>() -> ConstVec<[SerialSize; Self::N]>;
}

pub type Fields = ConstVec<[u16; 128]>;

#[derive(Eq, PartialEq, Clone, Debug)]
#[repr(u8)]
pub enum SerialSize {
    Unsized { fields: Fields },
    Padding(usize),
    Sized { start: usize, len: usize },
}

impl SerialSize {
    pub const fn unsized_of<T: 'static>() -> SerialSize {
        SerialSize::Unsized {
            fields: ConstVec::new(0, unsafe { MaybeUninit::zeroed().assume_init() }),
        }
    }

    pub const fn clone(&self) -> Self {
        match self {
            SerialSize::Unsized { fields } => SerialSize::Unsized {
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

impl<T: 'static> const SerialDescriptor for T {
    default const N: usize = 1;
    default fn fields<C: const CheckPrimitiveTypeSize>() -> ConstVec<[SerialSize; Self::N]> {
        if C::is_sized::<Self>() {
            ConstVec::new(Self::N, unsafe {
                const_transmute([SerialSize::Sized {
                    start: 0,
                    len: size_of::<Self>(),
                }])
            })
        } else {
            ConstVec::new(
                Self::N,
                [const { SerialSize::unsized_of::<Self>() }; Self::N],
            )
        }
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

    pub const fn next_field<E: const SerialDescriptor, C: const CheckPrimitiveTypeSize>(
        &mut self,
        field_ptr: &E,
    ) where
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
        let mut slice = <E as SerialDescriptor>::fields::<C>().clone();
        let mut i = 0;

        while i < slice.len() {
            match slice.as_slice()[i].clone() {
                SerialSize::Unsized { fields } => {
                    let mut fields = fields.clone();
                    fields.push(&(self.counter as u16));
                    *slice.get_mut(i) = SerialSize::Unsized { fields: fields };
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

pub fn encode2<T: SerialDescriptor + Encode, E: Encoder>(
    value: &T,
    encoder: E,
) -> Result<(), E::Error>
where
    [(); T::N]:,
    [(); size_of::<T>()]:,
{
    let mut i = 0;
    let mut tup = encoder.encode_tuple()?;
    let fields = T::fields::<E>();
    while i < fields.len() {
        match &fields.as_slice()[i] {
            SerialSize::Unsized { fields } => {
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

pub fn decode2<'de, T: Sized + SerialDescriptor + Decode<'de>, D: Decoder<'de>>(
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
    let fields = T::fields::<D>();
    while i < fields.len() {
        match &fields.as_slice()[i] {
            SerialSize::Unsized { fields } => {
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

pub fn calc_field_offset<T, F>(base_ptr: &T, field_ptr: &F) -> usize {
    let base_ptr = base_ptr as *const _ as *const u8;
    unsafe { (field_ptr as *const _ as *const u8).byte_sub_ptr(base_ptr) }
}

pub const fn compact_fields<const N: usize>(
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
                (None, SerialSize::Unsized { fields }) => {
                    size = Some(SerialSize::Unsized { fields });
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

pub struct DecodeFieldState<'a, T> {
    result: &'a T,
    fields: Fields,
}

impl<'de, 'a, T: Decode<'de>> DecodeFieldState<'a, T> {
    pub fn new(result: &'a T, fields: Fields) -> Self {
        Self { result, fields }
    }

    pub fn start<D: CompositeDecoder<'de>>(
        &mut self,
        decoder: &mut D,
    ) -> Result<Result<ReadableField<T>, D::Error>, u16> {
        if self.fields.len() == 0 {
            let result = match decoder.decode_element() {
                Err(err) => return Ok(Err(err)),
                Ok(value) => value,
            };
            Ok(Ok(ReadableField {
                offset: 0,
                len: size_of::<Self>(),
                value: result,
            }))
        } else {
            Err(*self.fields.pop_last())
        }
    }

    pub fn decode_field<D: CompositeDecoder<'de>, F: DecodeField<'de>>(
        &mut self,
        decoder: &mut D,
        field: &F,
    ) -> Result<ReadableField<T>, D::Error>
    where
        [(); size_of::<T>()]:,
    {
        let ReadableField { offset, len, value } =
            unsafe { DecodeField::decode_field(&self.fields, decoder)? };
        let result: T = unsafe { MaybeUninit::uninit().assume_init() };
        let field_offset = calc_field_offset(self.result, field);
        let mut result: [u8; size_of::<T>()] = unsafe { const_transmute(result) };
        let dst = unsafe {
            slice::from_raw_parts_mut(
                (&mut result as *mut _ as *mut u8).add(field_offset),
                size_of::<F>(),
            )
        };
        let src = unsafe { slice::from_raw_parts(&value, size_of::<F>()) };
        dst.copy_from_slice(src);

        Ok(ReadableField {
            offset: field_offset + offset,
            len,
            value: unsafe { const_transmute(result) },
        })
    }
}
