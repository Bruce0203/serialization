use std::{
    mem::{discriminant, transmute, Discriminant, MaybeUninit},
    num::{NonZero, NonZeroUsize},
};

use serialization::count_items;

pub(crate) const unsafe fn const_transmute<A, B>(a: A) -> B {
    #[repr(C)]
    union Union<A, B> {
        a: std::mem::ManuallyDrop<A>,
        b: std::mem::ManuallyDrop<B>,
    }

    let a = std::mem::ManuallyDrop::new(a);
    std::mem::ManuallyDrop::into_inner(Union { a }.b)
}

macro_rules! test {
    ($T:ty, $fill:ident, $($value:expr),*) => {{
        use std::{any::type_name, mem::transmute};

        let mut punchboards = [[Option::<bool>::Some(false); size_of::<$T>()]; $crate::count_items!($($value,)*)];
        #[allow(unused_variables)]
        let mut variant_i = 0;
        $(
            #[allow(unused_assignments)]
            {
                let ref mut punchboard = punchboards[variant_i];
                let mut last_chunk: Option<[u8; size_of::<$T>()]> = None;
                println!("{:?}", type_name::<$T>());
                for default in u8::MIN..=u8::MAX {
                    let chunk = unsafe {
                        #[allow(unused_variables)]
                        let $fill = [default; size_of::<$T>()];
                        transmute::<$T, [u8; size_of::<$T>()]>($value)
                    };
                    println!("default = {default}, {chunk:?}");
                    if let Some(last_chunk) = last_chunk {
                        for i in 0..size_of::<$T>() {
                            if chunk[i] == last_chunk[i] {
                                if let Some(ref mut value) = punchboard[i] {
                                    *value = true
                                }
                            }
                        }
                    }
                    last_chunk = Some(chunk);
                }
                variant_i += 1;
            }
        )*
        // punchboard
        //     .into_iter()
        //     .fold(0, |acc, v| if let Some(true) = v { acc + 1 } else { acc })

    }};
}

fn main() {
    type T = Option<bool>;
    let value: T = None;
    let discriminant: std::mem::Discriminant<T> = discriminant::<T>(&value);
    println!("{:?}", unsafe {
        transmute::<_, [u8; size_of::<Discriminant<T>>()]>(discriminant)
    });
    println!("{:?}", unsafe {
        transmute::<_, [u8; size_of::<T>()]>(value)
    });
}
