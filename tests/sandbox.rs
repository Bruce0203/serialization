use serialization::{Encode, Encoder};

pub struct TeestA {}

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
        let mut enc = encoder.begin_struct()?;
        enc.encode_element(&self.v1)?;
        enc.encode_element(&self.v2)?;
        enc.end_struct()?;
        Ok(())
    }
}

impl Encode for TestStruct2 {
    fn encode<E>(&self, encoder: E) -> Result<(), E::Error>
    where
        E: Encoder,
    {
        let mut enc = encoder.begin_struct()?;
        enc.encode_element(&self.v1)?;
        enc.encode_element(&self.v2)?;
        enc.encode_element(&self.vec)?;
        enc.end_struct()?;
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
