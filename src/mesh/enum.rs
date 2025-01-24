use std::mem::MaybeUninit;

pub struct Enum<T>(MaybeUninit<T>);
