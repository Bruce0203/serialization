#![feature(test)]

use std::{hint::black_box, mem::MaybeUninit};

use test::Bencher;

extern crate test;

#[bench]
fn asdf(b: &mut Bencher) {
    b.iter(|| {
        let dst: [u8; 100] = unsafe { MaybeUninit::uninit().assume_init() };
        let splited = dst.split_at(100).1;
        black_box((&dst, &splited));
    });
}
