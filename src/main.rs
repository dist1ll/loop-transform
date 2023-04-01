#![allow(dead_code)]
#![allow(unused)]

use std::{
    arch::asm,
    collections::BTreeSet,
    hint::black_box,
    sync::atomic::compiler_fence,
    time::{Duration, Instant},
};

use rand::Rng;

struct Args {
    first: bool,
}

const COUNT: usize = 2048;

fn main() {
    let mut args: BTreeSet<_> = std::env::args_os()
        .skip(1)
        .map(|e| e.into_string().unwrap())
        .collect();
    // generate random data
    let mut data = [1f32; COUNT];
    let mut rng = rand::thread_rng();
    for val in data.iter_mut() {
        *val = rng.gen();
    }
    // call benchmark
    if args.contains("--first") {
        first(&mut data);
    } else if args.contains("--second") {
        second(&mut data);
    } else {
        println!("\x1b[0;31mError:\x1b[0m argument missing: '--first'");
        std::process::exit(0);
    }
}

#[inline(never)]
pub fn first(data: &mut [f32; COUNT]) {
    let mut i = 0;
    while i < 3000000u64 {
        for d in data.iter_mut() {
            if *d > 0.5f32 {
                *d *= 0.492f32;
            } else {
                *d *= 0.5012f32;
            }
        }
        i += 1;
    }
    black_box(data);
}

#[inline(never)]
pub fn second(data: &mut [f32; COUNT]) {
    const COUNT: usize = 2048;
    (0..3000000u64).into_iter().for_each(|_| {
        data.iter_mut().for_each(|d| {
            if *d > 0.5f32 {
                *d *= 0.492f32;
            } else {
                *d *= 0.5012f32;
            }
        });
    });
    black_box(data);
}
