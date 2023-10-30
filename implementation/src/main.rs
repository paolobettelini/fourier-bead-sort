mod fourier;
mod quicksort;

use rand::prelude::*;
use std::time::Instant;

fn main() {
    let mut res = vec![];
    let mut sizes = vec![];

    // 10, 20, ..., 90, 100, 200, ..., 900, 1000, 2000, ...
    for i in 1..5 {
        for n in 1..10 {
            sizes.push(n * 10_i32.pow(i as u32) as usize);
        }
    }
    sizes.extend_from_slice(&vec![
        100000, 110000, 120000, 130000, 140000, 150000, 160000, 170000, 180000, 190000, 200000,
    ]);

    // Sort arrays
    for n in sizes {
        let (time1, time2) = benchmark(n);
        res.push((n, time1, time2));
    }

    // Print results
    for val in res {
        println!("{},{},{}", val.0, val.1, val.2);
    }
}

fn benchmark(size: usize) -> (u128, u128) {
    let mut n = vec![0u32; size];
    let mut rng = rand::thread_rng();
    for i in 0..size {
        let y: f64 = rng.gen(); // generates a float between 0 and 1
        n[i] = (y * 9.0 + 1.0) as u32;
    }

    let mut res = n.clone();

    let start_time = Instant::now();
    fourier::sort(&n, &mut res);
    let end_time = Instant::now();
    let time1 = (end_time - start_time).as_millis();

    let start_time = Instant::now();
    quicksort::quick_sort(&mut n);
    let end_time = Instant::now();
    let time2 = (end_time - start_time).as_millis();

    if n != res {
        panic!("Not correct");
    }

    (time1, time2)
}
