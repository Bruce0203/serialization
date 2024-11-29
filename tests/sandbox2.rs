#[derive(serialization::Encode, serialization::Decode)]
struct A;
#[derive(serialization::Encode, serialization::Decode)]
struct B {}
#[derive(serialization::Encode, serialization::Decode)]
struct C();
#[derive(serialization::Encode, serialization::Decode)]
struct D(i32);
#[derive(serialization::Encode, serialization::Decode)]
struct E {
    v: i32,
}
#[derive(serialization::Encode, serialization::Decode)]
struct F {
    v1: i32,
    v2: u16,
}
#[derive(serialization::Encode, serialization::Decode)]
struct G(u32, i16);

#[derive(serialization::Encode, serialization::Decode)]
enum H {
    A,
}

#[derive(serialization::Encode, serialization::Decode)]
enum I {
    A,
    B,
    C,
}
#[derive(serialization::Encode, serialization::Decode)]
enum J {
    A(i32),
}
#[derive(serialization::Encode, serialization::Decode)]
enum K {
    A(i32, u16),
}
#[derive(serialization::Encode, serialization::Decode)]
enum L {
    A(i32, u16),
    B(u32, i16),
}
#[derive(serialization::Encode, serialization::Decode)]
enum M {
    A,
    B(u32),
    C,
}
