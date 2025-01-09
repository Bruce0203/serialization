use std::any::type_name;
use typenum::{ToUInt, Unsigned};

use serialization::{Actor, Edge, FieldOffset, impl_meshup, trim};

struct Model {
    field0: u8,
    field1: Foo,
    field2: Vec<u8>,
    field3: u32,
    field4: Bar,
    field5: u32,
}
impl_meshup!(Model; field0: u8, field1: Foo, field2: Vec<u8>, field3: u32, field4: Bar, field5: u32);

struct Foo {
    field0: u32,
    field1: u32,
    field2: Bar,
}
impl_meshup!(Foo; field0: u32, field1: u32, field2: Bar);

struct Bar {
    field0: u8,
    field1: u8,
}
impl_meshup!(Bar; field0: u8, field1: u8);

#[test]
fn actor() {
    <Model as Actor>::run_at(20);
}
