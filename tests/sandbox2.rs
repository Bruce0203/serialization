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
