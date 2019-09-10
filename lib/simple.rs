use std::time::{Duration, Instant};

// internal crate defined in this library
use crate::sample::sample;

// estimate pi in a single-threaded manner by invoking the sampler for the specified number of
// samples and summing up the results - this will return the average number of points that are
// inside the unit circle, which is equivalent to pi/4, so we multiply by 4 when returning
pub fn simple_estimate_pi(num_samples: u32) -> f64 {
    let all_samples = (0..num_samples)
        .flat_map(sample)
        .collect::<Vec<i32>>();
    let sample_count = num_samples * (num_samples + 1) / 2;
    4.0 * (all_samples.iter().fold(0, |a, &b| a + b) as f64) / (sample_count as f64)
}

// time an estimation of pi given an input of the number of samples and print the elapsed time and
// estimated value
pub fn time_and_print(num_samples: u32, f: &Fn(u32) -> f64) {
    let start = Instant::now();
    let pi = f(num_samples);
    let elapsed = start.elapsed();
    print(num_samples, elapsed, pi)
}

pub fn print(num_samples: u32, elapsed: Duration, pi: f64) {
    println!("Estimated pi with {} samples in {} ms: {}", num_samples, elapsed.as_millis(), pi)
}
