extern crate rayon;

use rayon::prelude::*;

use crate::sample::sample;

pub fn parallel_estimate_pi(num_samples: u32) -> f64 {
    let all_samples = (0..num_samples)
        .into_par_iter()
        .flat_map(sample)
        .collect::<Vec<i32>>();
    let sample_count = num_samples * (num_samples + 1) / 2;
    4.0 * (all_samples.iter().fold(0, |a, &b| a + b) as f64) / (sample_count as f64)
}
