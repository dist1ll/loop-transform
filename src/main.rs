use std::{arch::asm, hint::black_box, sync::atomic::compiler_fence, time::Instant};

fn main() {
    const COUNT: usize = 2048;

    // // First version
    // let mut data = [1f32; COUNT];
    // let start = Instant::now();
    // data.iter_mut().for_each(|d| {
    //     (0..300000u64).into_iter().for_each(|_| {
    //         if *d > 0.5f32 {
    //             *d *= 0.499f32;
    //         } else {
    //             *d *= 0.501f32;
    //         }
    //     })
    // });
    // let elapsed = start.elapsed();
    // println!("first version:   {elapsed:?}");
    // black_box(data);

    // // divider
    // unsafe { asm!("nop; ") }
    // compiler_fence(std::sync::atomic::Ordering::SeqCst);

    // // Second version
    // let mut data = [1f32; COUNT];
    // let start = Instant::now();
    // (0..300000u64).into_iter().for_each(|_| {
    //     data.iter_mut().for_each(|d| {
    //         if *d > 0.5f32 {
    //             *d *= 0.499f32;
    //         } else {
    //             *d *= 0.501f32;
    //         }
    //     })
    // });
    // let elapsed = start.elapsed();
    // println!("second version:  {elapsed:?}");
    // black_box(data);

    // // divider
    // unsafe { asm!("nop; ") }
    // compiler_fence(std::sync::atomic::Ordering::SeqCst);

    // Third version
    let mut data = [1u32; COUNT];
    let start = Instant::now();
    let mut i = 0;
    while i < 300000u64 {
        for d in data.iter_mut() {
            if *d % 5 != 0 {
                *d += 7;
            } else {
                *d -= 1;
            }
        }
        i += 1;
    }
    let elapsed = start.elapsed();
    println!("third version:  {elapsed:?}");
    black_box(data);

    // divider
    unsafe { asm!("nop; ") }
    compiler_fence(std::sync::atomic::Ordering::SeqCst);

    // Fourth version
    let mut data = [1u32; COUNT];
    let start = Instant::now();
    let mut i = 0;
    for d in data.iter_mut() {
        while i < 300000u64 {
            if *d % 5 != 0 {
                *d += 7;
            } else {
                *d -= 1;
            }
            i += 1;
        }
    }
    let elapsed = start.elapsed();
    println!("fourth version: {elapsed:?}");
    black_box(data);
}
