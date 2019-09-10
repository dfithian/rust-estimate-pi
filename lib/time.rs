use std::time::{Duration, Instant};

// define a trait that is a thunk that returns a parameterized type
pub trait Thunk<T> {
    fn get(&self) -> T;
}

// time any old thunk and return it along with the duration
pub fn time<T>(f: &Thunk<T>) -> (Duration, T) {
    let start = Instant::now();
    let x = f.get();
    let elapsed = start.elapsed();
    (elapsed, x)
}
