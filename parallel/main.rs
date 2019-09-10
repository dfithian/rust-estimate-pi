extern crate rand;
extern crate rayon;

use rand::Rng;
use rayon::prelude::*;

fn random_pt() -> (f64, f64) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0))
}

fn inside(x: f64, y: f64) -> bool {
    x * x + y * y < 1.0
}

fn sample(size: u32) -> Vec<i32> {
    (0..size)
        .into_par_iter()
        .map(|_| {
            let (x, y) = random_pt();
            inside(x, y) as i32
        })
        .collect()
}

fn estimate_pi(num_samples: u32) -> f64 {
    let all_samples = (0..num_samples)
        .into_par_iter()
        .flat_map(sample)
        .collect::<Vec<i32>>();
    let sample_count = num_samples * (num_samples + 1) / 2;
    4.0 * (all_samples.iter().fold(0, |a, &b| a + b) as f64) / (sample_count as f64)
}

fn main() {
    let pi = estimate_pi(1000);
    println!("Estimated pi: {}", pi)
}
