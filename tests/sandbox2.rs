use std::marker::PhantomData;

#[derive(serialization::Serializable)]
struct A;
#[derive(serialization::Serializable)]
struct B {}
#[derive(serialization::Serializable)]
struct C();
#[derive(serialization::Serializable)]
struct D(i32);
#[derive(serialization::Serializable)]
struct E {
    v: i32,
}
#[derive(serialization::Serializable)]
struct F {
    v1: i32,
    v2: u16,
}
#[derive(serialization::Serializable)]
struct G(u32, i16);

#[derive(serialization::Serializable)]
enum H {
    A,
}

#[derive(serialization::Serializable)]
enum I {
    A,
    B,
    C,
}
#[derive(serialization::Serializable)]
enum J {
    A(i32),
}
#[derive(serialization::Serializable)]
enum K {
    A(i32, u16),
}
#[derive(serialization::Serializable)]
enum L {
    A(i32, u16),
    B(u32, i16),
}
#[derive(serialization::Serializable)]
enum M {
    A,
    B(u32),
    C,
}

#[derive(serialization::Serializable)]
struct N<T> {
    vaule: PhantomData<T>,
}
#[derive(serialization::Serializable)]
struct O<T> {
    value: T,
}

#[derive(serialization::Serializable)]
enum P<T, T2> {
    A(T),
    B(PhantomData<T2>),
}
#[derive(serialization::Serializable)]
struct Q<'a> {
    value: &'a str,
}
#[derive(serialization::Serializable)]
struct R<'a> {
    value: &'a str,
}
#[derive(serialization::Serializable)]
struct S {}
#[derive(serialization::Serializable)]
struct T {}
#[derive(serialization::Serializable)]
struct U {}
#[derive(serialization::Serializable)]
struct V {}
#[derive(serialization::Serializable)]
struct W {}
#[derive(serialization::Serializable)]
struct X {}
#[derive(serialization::Serializable)]
struct Y {}
#[derive(serialization::Serializable)]
struct Z {}
