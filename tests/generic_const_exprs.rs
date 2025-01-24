#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::marker::PhantomData;

#[derive(serialization::Serializable, Debug, Eq, PartialEq)]
pub struct A14<T> {
    vaule: std::marker::PhantomData<T>,
}
#[derive(serialization::Serializable, Debug, Eq, PartialEq)]
pub struct A15<T> {
    value: T,
}

#[derive(serialization::Serializable, Debug, Eq, PartialEq)]
pub enum A16<T, T2> {
    A(T),
    B(std::marker::PhantomData<T2>),
}

//TODO support only Encode, so Serializable -> Serialize, Deserialize
// #[derive(serialization::serializable, debug, eq, partialeq)]
// struct a17<'a> {
//     value: &'a str,
// }

#[derive(serialization::Serializable, Debug, Eq, PartialEq)]
struct A17<'a> {
    value: PhantomData<&'a ()>,
}

#[derive(serialization::Serializable, Debug, Eq, PartialEq)]
struct AAA<T1, T2> {
    a: T1,
    b: T2,
}
