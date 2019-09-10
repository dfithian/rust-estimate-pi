// internal crate defined in this library
use crate::sample::sample;

// estimate pi in a single-threaded manner by invoking the sampler for the specified number of
// samples and summing up the results - this will return the average number of points that are
// inside the unit circle, which is equivalent to pi/4, so we multiply by 4 when returning
pub fn simple_estimate_pi(num_samples: u32) -> f64 {
    let all_samples = (0..num_samples) // this is the range defined by [0, num_samples)
        .flat_map(sample)              // flat_map will flatten the return value from a list of lists to a single list
        .collect::<Vec<i32>>();        // convert from iterator to vector
    let sample_count = num_samples * (num_samples + 1) / 2; // Gauss ftw
    4.0 * (all_samples.iter().fold(0, |a, &b| a + b) as f64) / (sample_count as f64)
}
