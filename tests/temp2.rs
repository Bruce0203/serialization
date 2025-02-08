fn main() {}

mod a {
    macro_rules! aa {
        () => {};
    }
    pub(crate) use aa;
}

macro_rules! aa {
    () => {};
}

mod aasdf {
    use super::a::*;
    use super::*;
    self::aa!();
}
