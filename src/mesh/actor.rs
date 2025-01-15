use core::primitive::usize;
use std::any::type_name;

use crate::Encode;

use super::{edge::PhantomEdge, end::End, field::Field, len::Len, padding::Padding};

pub trait Actor<S> {
    fn run_at(action: Action<S>, _index: usize) -> Continuous<S>;

    fn run(action: Action<S>);
}

#[derive(Debug)]
pub enum Continuous<T> {
    Next(Action<T>),
    Done,
}

#[derive(Debug)]
pub enum Action<T> {
    Encode { src: *const T, dst: *mut T },
    Decode { src: *const T, dst: *mut T },
    Drop { ptr: *const T },
}

/// Copies `N` or `n` bytes from `src` to `dst` depending on if `src` lies within a memory page.
/// https://stackoverflow.com/questions/37800739/is-it-safe-to-read-past-the-end-of-a-buffer-within-the-same-page-on-x86-and-x64
/// # Safety
/// Same as [`std::ptr::copy_nonoverlapping`] but with the additional requirements that
/// `n != 0 && n <= N` and `dst` has room for a `[T; N]`.
/// Is a macro instead of an `#[inline(always)] fn` because it optimizes better.
macro_rules! unsafe_wild_copy {
    // pub unsafe fn wild_copy<T, const N: usize>(src: *const T, dst: *mut T, n: usize) {
    ([$T:ident; $N:expr], $src:ident, $dst:ident, $n:expr) => {
        debug_assert!($n != 0 && $n <= $N);

        let page_size = 4096;
        let read_size = core::mem::size_of::<[$T; $N]>();
        let within_page = $src as usize & (page_size - 1) < (page_size - read_size) && cfg!(all(
            // Miri doesn't like this.
            not(miri),
            // cargo fuzz's memory sanitizer complains about buffer overrun.
            // Without nightly we can't detect memory sanitizers, so we check debug_assertions.
            not(debug_assertions),
            // x86/x86_64/aarch64 all have min page size of 4096, so reading past the end of a non-empty
            // buffer won't page fault.
            any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")
        ));

        if within_page {
            *($dst as *mut core::mem::MaybeUninit<[$T; $N]>) = core::ptr::read($src as *const core::mem::MaybeUninit<[$T; $N]>);
        } else {
            $src.copy_to_nonoverlapping($dst, $n);
        }
    }
}

impl<S, A, B> Actor<S> for PhantomEdge<S, (Field<A>, B)>
where
    Self: Len,
    Field<A>: Actor<S>,
    B: Actor<S>,
    A: Encode,
    [(); <Self as Len>::SIZE]:,
{
    fn run_at(action: Action<S>, mut index: usize) -> Continuous<S> {
        if index == 0 {
            Self::run(action);
            return Continuous::Done;
        }
        index -= 1;
        if let Continuous::Next(action) = Field::<A>::run_at(action, index) {
            B::run_at(action, index)
        } else {
            Continuous::Done
        }
    }

    fn run(action: Action<S>) {
        match action {
            Action::Encode { src, dst } => {
                if Self::SIZE == 0 {
                } else {
                    unsafe {
                        let src = src as *const [u8; Self::SIZE];
                        let dst = dst as *mut [u8; Self::SIZE];
                        unsafe_wild_copy!([u8; Self::SIZE], src, dst, Self::SIZE);
                    };
                }
            }
            Action::Decode { src, dst } => {}
            Action::Drop { ptr } => {}
        }
        println!("field {} {}", <Self as Len>::SIZE, type_name::<A>());
    }
}

impl<S, S2, B, FrontOffset> Actor<S> for PhantomEdge<S, (Padding<S2, FrontOffset>, B)>
where
    Self: Len,
    Padding<S2, FrontOffset>: Actor<S>,
    B: Actor<S>,
{
    fn run_at(action: Action<S>, index: usize) -> Continuous<S> {
        if let Continuous::Next(action) = Padding::<S2, FrontOffset>::run_at(action, index) {
            B::run_at(action, index)
        } else {
            Continuous::Done
        }
    }

    fn run(_action: Action<S>) {
        unreachable!()
    }
}

impl<S> Actor<S> for End<S> {
    fn run_at(action: Action<S>, _index: usize) -> Continuous<S> {
        Continuous::Next(action)
    }

    fn run(_action: Action<S>) {
        unreachable!()
    }
}

impl<S, FrontOffset> Actor<S> for Padding<S, FrontOffset> {
    fn run_at(action: Action<S>, _index: usize) -> Continuous<S> {
        Continuous::Next(action)
    }

    fn run(_action: Action<S>) {
        unreachable!()
    }
}

impl<S, T> Actor<S> for Field<T> {
    fn run_at(action: Action<S>, _index: usize) -> Continuous<S> {
        Continuous::Next(action)
    }

    fn run(_action: Action<S>) {
        unreachable!()
    }
}
