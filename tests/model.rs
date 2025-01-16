#![feature(test)]

use std::{any::type_name, mem::offset_of};

use serialization::__private::{Edge, Flatten, Sorted};

extern crate test;

#[repr(C)]
#[derive(serialization::Serializable)]
struct Model {
    field0: u8, // offset 0 size 1
    // padding 3
    field1: Foo, // offset 4 size 12
    // padding 0
    field2: Vec<u8>, // offset 16 size 24
    // padding 0
    field3: u32, // offset 40 size 4
    // padding 0
    field4: Bar, // offset 44 size 2
    // padding 2
    field5: u32, // offset 48 size 4
                 // padding 4
                 // model size 56
}

#[repr(C)]
#[derive(serialization::Serializable)]
struct Foo {
    field0: u32, // offset 0  size 4
    // padding 0
    field1: u32, // offset 4 size 4
    // padding 0
    field2: Bar, //offset 8 size 2
                 // padding 2
                 // size 12
}

#[repr(C)]
#[derive(serialization::Serializable)]
struct Bar {
    field0: u8, // offset 0 size 1
    // padding 0
    field1: u8, // offset 0 size 1
                // padding 0
}

#[test]
fn actor() {
    type T = <<<Model as Edge>::Second as Sorted>::Output as Flatten<Model>>::Output;
    println!("{}", type_name::<T>());
    let src: &Model = &Model {
        field0: 11,
        field1: Foo {
            field0: 22,
            field1: 33,
            field2: Bar {
                field0: 44,
                field1: 55,
            },
        },
        field2: vec![1, 2, 3, 4],
        field3: 66,
        field4: Bar {
            field0: 77,
            field1: 88,
        },
        field5: 99,
    };
    #[allow(invalid_value)]
    let mut dst = [0u8; 10000];
    serialization_default_binary_encoder::encode(src, &mut dst);
    println!("{:?}", &dst[..66]);
}

#[cfg(not(debug_assertions))]
#[bench]
fn bench_must_0ns(b: &mut test::Bencher) {
    for i in 0..1000 {
        <<Model as crate::__private::Edge>::Second as crate::__private::Actor>::run_at(i, todo!());
    }
}
