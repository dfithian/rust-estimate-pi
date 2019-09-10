extern crate rand;

use rand::Rng;

fn random_pt() -> (f64, f64) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0))
}

fn inside(x: f64, y: f64) -> bool {
    x * x + y * y < 1.0
}

pub fn sample(size: u32) -> Vec<i32> {
    (0..size)
        .map(|_| {
            let (x, y) = random_pt();
            inside(x, y) as i32
        })
        .collect()
}
