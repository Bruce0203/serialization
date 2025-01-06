use super::{CompoundWrapper, FieldOffset, PhantomEdge, edge::Edge};

impl Edge for () {}
impl Edge for u32 {}
impl Edge for u8 {}
impl Edge for Vec<u8> {}

impl<S> CompoundWrapper<S> for () {
    type Compound = PhantomEdge<S, Self>;
}
impl<S> CompoundWrapper<S> for u8 {
    type Compound = PhantomEdge<S, Self>;
}
impl<S> CompoundWrapper<S> for Vec<u8> {
    type Compound = PhantomEdge<S, Self>;
}
impl<S> CompoundWrapper<S> for u32 {
    type Compound = PhantomEdge<S, Self>;
}
impl<S> FieldOffset<S> for () {
    const OFFSET: usize = 0;
}
impl<S> FieldOffset<S> for u8 {
    const OFFSET: usize = 0;
}
impl<S> FieldOffset<S> for Vec<u8> {
    const OFFSET: usize = 0;
}
impl<S> FieldOffset<S> for u32 {
    const OFFSET: usize = 0;
}

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
