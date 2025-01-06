use super::{CompoundWrapper, FieldOffset, Node, PhantomEdge, edge::Edge};

impl Edge for () {}
impl Edge for u32 {}
impl Edge for u8 {}
impl Edge for Vec<u8> {}

impl Node for () {}
impl Node for u32 {}
impl Node for u8 {}
impl Node for Vec<u8> {}

//
// impl Size for () {
//     const SIZE: usize = 0;
// }
//
// impl Size for u32 {
//     const SIZE: usize = size_of::<Self>();
// }
//
// impl Size for u8 {
//     const SIZE: usize = size_of::<Self>();
// }
