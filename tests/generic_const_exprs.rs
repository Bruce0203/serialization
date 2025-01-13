#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

#[derive(serialization::Serializable, Debug, Eq, PartialEq)]
struct A14<T> {
    vaule: std::marker::PhantomData<T>,
}
#[derive(serialization::Serializable, Debug, Eq, PartialEq)]
struct A15<T> {
    value: T,
}

#[derive(serialization::Serializable, Debug, Eq, PartialEq)]
enum A16<T, T2> {
    A(T),
    B(std::marker::PhantomData<T2>),
}
#[derive(serialization::Serializable, Debug, Eq, PartialEq)]
struct A17<'a> {
    value: &'a str,
}
