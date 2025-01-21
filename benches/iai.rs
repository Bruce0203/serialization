#![feature(test)]
use iai_callgrind::{library_benchmark, library_benchmark_group, main};
use rand::Rng;
use test::Bencher;

extern crate test;

const N: usize = 100;

#[bench]
// #[library_benchmark]
fn benchmark(b: &mut Bencher) {
    let mut dst = [0_u8; N];
    let src = [rand::thread_rng().gen_range(1..1); N];
    dst.copy_from_slice(&src);
}

fn main() {}

// library_benchmark_group!(name = bench_group; benchmarks = benchmark);
// main!(library_benchmark_groups = bench_group);
