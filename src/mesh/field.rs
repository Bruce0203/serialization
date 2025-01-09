use std::marker::PhantomData;

use typenum::Unsigned;

use super::{Compound, CompoundWrapper, Edge, PhantomEdge, PhantomLeaf};

pub trait FieldOffset<S> {
    type Offset;
}

pub struct PhantomField<S, T, const I: usize>(PhantomData<(S, T)>);

impl<S, T, const I: usize> Edge for PhantomField<S, T, I> where T: Edge {}

//TODO compound를 마친 후에 field offset sorting을 하자 즉 compound를 함과 동시에 field offset
//sorting을 하는 것이다. 깊이가 0을 초과하면 field offset ordering에 영향을 주지 않게끔 S타입을
//specializaiton 한다
//앗차! 체인을 감아가는식으로는 단순하게만 해서는 순서 정렬이 불가능하다
//
//해야할 것:  Compund 하기 전에 PhantomField 로 채우자
