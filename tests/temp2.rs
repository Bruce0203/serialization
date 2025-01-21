#[derive(serialization::Serializable, Debug, Eq, PartialEq)]
struct AAA<T1, T2> {
    t1: T1,
    t2: T2,
}
