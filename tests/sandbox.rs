use fastbuf::{Buf, Buffer};
use serialization::{
    CompositeDecoder, CompositeEncoder, Decode, DecodeError, Decoder, Encode, Encoder,
    EnumIdentifier,
};
use serialization_minecraft::PacketDecoder;

pub struct TeestA {}

#[derive(Debug, Clone, Copy, Encode, Decode)]
#[repr(u8)]
pub enum TestEnum {
    A(usize, usize) = 100,
    B = 102,
    C { value: i32 } = 105,
    D = 108,
    E = 134,
}

#[test]
fn testasdf() {
    let value = TestEnum::C { value: 123 };
    let mut buf = Buffer::<123>::new();
    let ref mut enc = serialization_minecraft::PacketEncoder::new(&mut buf);
    Encode::encode(&value, enc).unwrap();
    println!("{:?}", buf);
    let mut dec = PacketDecoder::new(&mut buf);
    let v: TestEnum = Decode::decode(&mut dec).unwrap();
    println!("{v:?}");
}

pub struct TestStruct {
    v1: u8,
    v2: Option<TestStruct2>,
}

pub struct TestStruct2 {
    v1: u16,
    v2: u8,
    vec: Vec<u8>,
}

impl Encode for TestStruct {
    fn encode<E>(&self, encoder: E) -> Result<(), E::Error>
    where
        E: Encoder,
    {
        let mut enc = encoder.encode_struct()?;
        enc.encode_element(&self.v1)?;
        enc.encode_element(&self.v2)?;
        enc.end()?;
        Ok(())
    }
}

impl Encode for TestStruct2 {
    fn encode<E>(&self, encoder: E) -> Result<(), E::Error>
    where
        E: Encoder,
    {
        let mut enc = encoder.encode_struct()?;
        enc.encode_element(&self.v1)?;
        enc.encode_element(&self.v2)?;
        enc.encode_element(&self.vec)?;
        enc.end()?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{TestStruct, TestStruct2};
    use fastbuf::{Buffer, ReadBuf};
    use serialization::Encode;

    #[test]
    fn test_struct() {
        let mut encoder = serialization_minecraft::PacketEncoder::new(Buffer::<255>::new());
        TestStruct {
            v1: 123,
            v2: Some(TestStruct2 {
                v1: 123,
                v2: 234,
                vec: vec![1, 2, 3],
            }),
        }
        .encode(&mut encoder)
        .unwrap();
        let data = [123, 1, 0, 123, 234, 3, 1, 2, 3];
        assert_eq!(encoder.get_continuous(data.len()), data);
    }
}

#[derive(Encode, Decode)]
struct A {
    v1: u8,
    v2: u16,
}

#[derive(Encode, Decode)]
struct B();

#[derive(Encode, Decode)]
struct C(i32);
