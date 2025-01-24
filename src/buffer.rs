use std::mem::{transmute, MaybeUninit};

pub struct Buffer {
    ptr: *mut u8,
    len: usize,
}

pub trait BufWrite {
    fn write_array<T: Copy, const N: usize>(&mut self, src: &[T; N]);
    fn write_slice<T: Copy>(&mut self, src: &[T]);
}

pub trait BufRead {
    fn read_array<T: Copy, const N: usize>(&mut self, out: &mut MaybeUninit<[T; N]>);
    fn read_slice<T: Copy>(&mut self, out: &mut [MaybeUninit<T>]);
}

impl From<&[u8]> for Buffer {
    fn from(value: &[u8]) -> Self {
        Self {
            ptr: value.as_ptr() as *const _ as *mut u8,
            len: value.len(),
        }
    }
}

impl From<&mut [u8]> for Buffer {
    fn from(value: &mut [u8]) -> Self {
        Self {
            ptr: value.as_mut_ptr(),
            len: value.len(),
        }
    }
}

/// Copies `N` or `n` bytes from `src` to `dst` depending on if `src` lies within a memory page.
/// https://stackoverflow.com/questions/37800739/is-it-safe-to-read-past-the-end-of-a-buffer-within-the-same-page-on-x86-and-x64
/// # Safety
/// Same as [`std::ptr::copy_nonoverlapping`] but with the additional requirements that
/// `n != 0 && n <= N` and `dst` has room for a `[T; N]`.
/// Is a macro instead of an `#[inline(always)] fn` because it optimizes better.
#[macro_export]
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

impl BufWrite for Buffer {
    fn write_array<T: Copy, const N: usize>(&mut self, src: &[T; N]) {
        let dst = self.ptr as *mut T;
        self.ptr = dst.wrapping_add(N) as *mut u8;
        let src = src.as_ptr();
        unsafe {
            unsafe_wild_copy!([T; N], src, dst, N);
        }
    }

    fn write_slice<T: Copy>(&mut self, src: &[T]) {
        // Most cpu cache lane is 64 bytes or 128 bytes. so 1/4 size will be fine.
        const CHUNK_SIZE: usize = if cfg!(any(
            target_arch = "x86",
            target_arch = "x86_64",
            target_arch = "aarch64"
        )) {
            16
        } else {
            4
        };
        let mut iter = src.chunks_exact(CHUNK_SIZE);
        loop {
            let chunk = match iter.next() {
                Some(v) => v,
                None => {
                    let remainder = iter.remainder();
                    let dst = self.ptr as *mut T;
                    let src = remainder.as_ptr();
                    unsafe {
                        //TODO DANGER!! must check buffer remaining size is more than CHUNK_SIZE
                        unsafe_wild_copy!([T; CHUNK_SIZE], src, dst, CHUNK_SIZE);
                    }
                    self.ptr = dst.wrapping_add(remainder.len()) as *mut u8;
                    break;
                }
            };
            let dst = self.ptr as *mut T;
            unsafe {
                let src = chunk.as_ptr();
                unsafe_wild_copy!([T; CHUNK_SIZE], src, dst, CHUNK_SIZE);
            }
            self.ptr = dst.wrapping_add(CHUNK_SIZE) as *mut u8;
        }
    }
}

impl BufRead for Buffer {
    fn read_slice<T: Copy>(&mut self, out: &mut [MaybeUninit<T>]) {
        // Most cpu cache lane is 64 bytes or 128 bytes. so 1/4 size will be fine.
        const CHUNK_SIZE: usize = if cfg!(any(
            target_arch = "x86",
            target_arch = "x86_64",
            target_arch = "aarch64"
        )) {
            16
        } else {
            4
        };
        let mut iter = out.chunks_exact_mut(CHUNK_SIZE);
        loop {
            let chunk = match iter.next() {
                Some(v) => v,
                None => {
                    let remainder = iter.into_remainder();
                    unsafe {
                        for v in remainder.into_iter() {
                            let src = self.ptr as *const T;
                            self.ptr = src.wrapping_add(1) as *mut u8;
                            let dst = v.as_ptr() as *mut T;
                            unsafe_wild_copy!([T; 1], src, dst, 1);
                        }
                    }
                    break;
                }
            };
            let src = self.ptr as *const T;
            let dst = chunk.as_ptr() as *mut T;
            unsafe {
                unsafe_wild_copy!([T; CHUNK_SIZE], src, dst, CHUNK_SIZE);
            }
            self.ptr = src.wrapping_add(CHUNK_SIZE) as *mut u8;
        }
    }

    fn read_array<T: Copy, const N: usize>(&mut self, out: &mut MaybeUninit<[T; N]>) {
        let src = self.ptr as *const [T; N];
        let dst = out.as_mut_ptr() as *mut [T; N];
        self.ptr = src.wrapping_add(1) as *mut u8;
        unsafe {
            unsafe_wild_copy!([T; N], src, dst, N);
        }
    }
}
