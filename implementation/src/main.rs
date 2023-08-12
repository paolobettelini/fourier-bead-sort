extern crate num_complex;

use std::f64::consts::PI;
use num_complex::*;
use std::time::{Instant, Duration};
use rand::prelude::*;

fn main() {
    //let n = [0, 500, 10, 5, 7, 3, 9, 10, 10, 10, 10, 10, 10, 11];
    const N: usize = 100000;
    let mut n = vec![0u32; N];
    let mut rng = rand::thread_rng();
    for i in 0..N {
        let y: f64 = rng.gen(); // generates a float between 0 and 1
        n[i] = (y  * 2.0 + 1.0) as u32;
    }
    
    let mut res = n.clone();
    
    let start_time = Instant::now();
    sort(&n, &mut res);
    let end_time = Instant::now();
    println!("Time: {} ms", (end_time - start_time).as_millis());

    let start_time = Instant::now();
    n.sort();
    let end_time = Instant::now();
    println!("Time: {} ms", (end_time - start_time).as_millis());

    println!("Correctness: {}", n == res);

   // println!("{n:?}");
    //println!("{res:?}");
}

pub fn sort(n: &[u32], res: &mut [u32]) {
    let max = *n.iter().max().unwrap();

    let mut last_freq = -1;
    let mut last_value = 0;
    let mut current_index = 0;

    for freq in 1..=(max + 1) { // or =max and know that the last one is the max value itself
        let v = get_frequency(&n, freq as f64);
        println!("{freq} -- {v}");
        let v = v.round() as i32;

        if last_freq != -1 {
            if v != last_value {
                for _ in 0..(last_value - v) {
                    res[current_index] = last_freq as u32;
                    current_index += 1;
                }
            }
        }

        last_freq = freq as i32;
        last_value = v;
    }
} 

fn get_frequency(n: &[u32], freq: f64) -> f64 {
    let mut sum =0.0;

    for k in 0..n.len() {
        sum += fourier_integral(n[k] as f64, freq);
    }

    sum
}

fn f(t: f64, n_k: f64) -> f64 {
    (t * 0.5 * n_k).sin()
    / (0.5 * t).sin()
    * ((n_k + 1.0) * 0.5 * t).sin()
}

fn fourier_integral(n_k: f64, freq: f64) -> f64 {
    let n = 50;
    let a = 0.0;
    let b = 2.0 * PI;
    
    let delta_t = (b - a) / (n as f64);
    let mut sum = Complex::new(0.0, 0.0);

    for k in 0..n {
        let t = a + (k as f64 + 0.5) * delta_t;

        let f = f(t, n_k);
        let exp_term = Complex::new((-freq * t).cos(), (-freq * t).sin());

        sum += exp_term * f;
    }

    let approx = sum * delta_t;
    let approx = approx.norm() / (b-a) * 2.0;

    approx
}