#![allow(warnings)]
//TODO add where bound fuzzing

#[derive(serialization::Serializable, Debug, Eq, PartialEq)]
struct A1;
#[derive(serialization::Serializable, Debug, Eq, PartialEq)]
struct A2 {}
#[derive(serialization::Serializable, Debug, Eq, PartialEq)]
struct A3();
#[derive(serialization::Serializable, Debug, Eq, PartialEq)]
struct A4(i32);
#[derive(serialization::Serializable, Debug, Eq, PartialEq)]
struct A5 {
    v: i32,
}
#[derive(serialization::Serializable, Debug, Eq, PartialEq)]
struct A6 {
    v1: i32,
    v2: u16,
}
#[derive(serialization::Serializable, Debug, Eq, PartialEq)]
struct A7(u32, i16);

#[derive(serialization::Serializable, Debug, Eq, PartialEq)]
enum A8 {
    A,
}

#[derive(serialization::Serializable, Debug, Eq, PartialEq)]
enum A9 {
    A,
    B,
    C,
}
#[derive(serialization::Serializable, Debug, Eq, PartialEq)]
enum A10 {
    A(i32),
}
#[derive(serialization::Serializable, Debug, Eq, PartialEq)]
enum A11 {
    A(i32, u16),
}
#[derive(serialization::Serializable, Debug, Eq, PartialEq)]
enum A12 {
    A(i32, u16),
    B(u32, i16),
}
#[derive(serialization::Serializable, Debug, Eq, PartialEq)]
enum A13 {
    A,
    B(u32),
    C,
}

// generic_const_exprs required
//
// #[derive(serialization::Serializable, Debug, Eq, PartialEq)]
// struct A14<T> {
//     vaule: std::marker::PhantomData<T>,
// }
// #[derive(serialization::Serializable, Debug, Eq, PartialEq)]
// struct A15<T> {
//     value: T,
// }
//
// #[derive(serialization::Serializable, Debug, Eq, PartialEq)]
// enum A16<T, T2> {
//     A(T),
//     B(std::marker::PhantomData<T2>),
// }
// #[derive(serialization::Serializable, Debug, Eq, PartialEq)]
// struct A17<'a> {
//     value: &'a str,
// }

#[derive(serialization::Serializable, Debug, Eq, PartialEq)]
struct A18 {
    value: (u32),
}
#[derive(serialization::Serializable, Debug, Eq, PartialEq)]
enum A19 {}

#[derive(serialization::Serializable, Debug, Eq, PartialEq)]
enum A20 {
    A = 2,
    B = 4,
    C = 6,
    D = 8,
    E = 10,
}

//TODO support only Encode, so Serializable -> Serialize, Deserialize
// #[derive(serialization::Serializable, Debug, Eq, PartialEq)]
// struct A21 {
//     value: &'static str,
// }
#[derive(serialization::Serializable, Debug, Eq, PartialEq)]
enum A22 {
    A { value: i32 },
    B { value2: u16 },
}
#[derive(serialization::Serializable, Debug, Eq, PartialEq)]
struct A23 {
    value: Vec<u8>,
}

#[derive(serialization::Serializable, Debug, Eq, PartialEq)]
struct A24 {
    value: Vec<String>,
}
#[repr(C)]
#[derive(serialization::Serializable, Debug, Eq, PartialEq)]
struct A25 {
    value: Vec<A26>,
}
#[repr(C)]
#[derive(serialization::Serializable, Debug, Eq, PartialEq)]
struct A26 {
    value1: u32,
    value2: String,
    value3: u8,
}

#[derive(serialization::Serializable, Debug, Eq, PartialEq)]
struct A27 {
    value: (u32, u8),
    value2: (u32, String, String),
}

#[derive(serialization::Serializable, Debug, Eq, PartialEq)]
struct A28 {
    vec: Vec<(u8, String, String)>,
}

#[cfg(test)]
mod tests {
    use std::{any::type_name, convert::Infallible, fmt::Debug, marker::PhantomData, str::FromStr};

    use crate::{
        mock::BinaryCodecMock,
        prelude::{Mesh, SegmentCodec, SegmentDecoder, SegmentEncoder, SegmentWalker},
        Buffer,
    };

    use super::*;

    #[test]
    fn test_fuzzing() {
        test(A1);
        test(A2 {});
        test(A3 {});
        test(A4(123));
        test(A5 { v: 123 });
        test(A6 { v1: 11, v2: 22 });
        test(A7(11, 22));
        test(A8::A);
        test(A9::B);
        test(A10::A(123));
        test(A10::A(123));
        test(A11::A(11, 22));
        test(A12::B(11, 22));
        test(A13::C);
        // test(A14::<usize> { vaule: PhantomData });
        // test(A15::<usize> { value: 123 });
        // test(A16::<usize, usize>::A(123));
        // test(A17 { value: "hi" });
        test(A18 { value: 123 });
        //A19 is infallible type
        test(A20::E);
        // test(A21 { value: "hi" });
        test(A22::A { value: 123 });
        test(A23 { value: vec![123] });
        test(A24 {
            value: vec![String::from_str("hi").unwrap()],
        });

        // [4, 104, 105, 104, 105, 11, 0, 0, 0, 22]
        // [11, 0, 0, 0, 4, 104, 105, 104, 105, 22]
        test(A25 {
            value: vec![A26 {
                value1: 11,
                value2: String::from_str("hihi").unwrap(),
                value3: 22,
            }],
        });
        test(A27 {
            value: (11, 22),
            value2: (
                11,
                String::from_str("hello").unwrap(),
                String::from_str("hihi").unwrap(),
            ),
        });
        test(A28 {
            vec: vec![(
                1,
                String::from_str("HIHIHIHHIH").unwrap(),
                String::from_str("asdfasdfa").unwrap(),
            )],
        });
    }

    fn test<T: Eq + Debug>(value: T)
    where
        T: Mesh<BinaryCodecMock, SegmentEncoder>,
        T: Mesh<BinaryCodecMock, SegmentDecoder>,
        [(); size_of::<T>()]:,
    {
        let mut dst = [0u8; 100000];
        crate::mock::encode(&value, &mut dst).unwrap();
        let decoded = crate::mock::decode::<T>(&mut dst).unwrap();
        // println!("{}", type_name::<T>());
        // println!("{:?}", &dst[..66]);
        assert_eq!(value, decoded);
    }
}
