extern crate rand;
extern crate rayon;

use std::io::{Error, ErrorKind};
use std::num::ParseIntError;

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

fn help() -> Result<u32, Error> {
    Err(Error::new(ErrorKind::Other, "usage:
pi <int>
Estimate pi with the number of samples"))
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let result = match args.len() {
        2 => { args[1].parse().map_err(|err: ParseIntError| Error::new(ErrorKind::Other, err.to_string())) },
        _ => { help() }

    }.map(estimate_pi);
    match result {
        Err(err) => panic!("Couldn't estimate pi: {}", err),
        Ok(pi) => println!("Estimated pi: {}", pi)
    }
}
