use std::time::Instant;

// time an estimation of Pi given an input of the number of samples and print the elapsed time and
// estimated value
pub fn time_and_print(num_samples: u32, f: &Fn(u32) -> f64) {
    let start = Instant::now();
    let pi = f(num_samples);
    let elapsed = start.elapsed();
    println!("Estimated pi with {} samples in {} ms: {}", num_samples, elapsed.as_millis(), pi)
}
