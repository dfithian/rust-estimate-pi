// external crate defined in Cargo.toml
extern crate rand;

// import functions or structs
use rand::Rng;

// generate two coordinates within the square of area 1 in the top-right quadrant
fn random_pt() -> (f64, f64) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0))
}

// determine whether the hypotenuse of the triangle defined by the points at (0, 0), (x, 0), (x, y)
// is within the unit circle using the Pythagorean theorem
fn inside(x: f64, y: f64) -> bool {
    x * x + y * y < 1.0
}

// generate a number of samples by which to evaluate whether or not randomized points are within the
// unit circle
pub fn sample(size: u32) -> Vec<i32> {
    (0..size)
        .map(|_| {
            let (x, y) = random_pt();
            inside(x, y) as i32
        })
        .collect()
}
