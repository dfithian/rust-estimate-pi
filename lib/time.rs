use std::time::{Duration, Instant};

use crate::parallel::parallel_estimate_pi;

// define a trait that is a thunk that returns a parameterized type
pub trait Thunk<T> {
    fn get(&self, _: u32) -> T;
}

pub trait ParameterizedThunk<I, O> {
    fn get(&self, _: I) -> O;
}
pub trait ParameterizedThunk2<I, O> {
    fn get(&self, _: I) -> O;
}

pub struct Sampler<I, O> { pub f: fn(I) -> O }

impl<I, O> ParameterizedThunk<I, O> for Sampler<I, O> {
    fn get(&self, input: I) -> O {
        (self.f)(input)
    }
}
impl<I, O> ParameterizedThunk2<I, O> for Sampler<I, O> {
    fn get(&self, input: I) -> O {
        (self.f)(input)
    }
}

impl<I, O> Thunk<O> for Sampler<I, O> {
    fn get(&self, input: u32) -> O {
        panic!("omg")
    }
}

// pub struct SampleInvoker { num_samples: u32, invoker: fn(u32) -> f64 }
// 
// impl Thunk<f64> for u32 {
//     fn get(&self) -> f64 {
//         parallel_estimate_pi(*self)
//     }
// }
// 
// impl Thunk<f64> for SampleInvoker {
//     fn get(&self) -> f64 {
//         (self.invoker)(self.num_samples)
//     }
// }
// 
// pub fn call_thunk(samples: u32) -> (Duration, f64) {
//     let my_invoker = SampleInvoker { num_samples: samples, invoker: parallel_estimate_pi };
//     time(&my_invoker)
// }
// 
// pub fn call_u32_thunk(samples: u32) -> (Duration, f64) {
//     time(&samples)
// }

// time any old thunk and return it along with the duration
pub fn time2<I, O>(input: I, f: &Sampler<I, O>) -> (Duration, O) {
    let start = Instant::now();
    let output = f.get(input);
    let elapsed = start.elapsed();
    (elapsed, output)
}
pub fn time<I, O>(input: I, f: &ParameterizedThunk<I, O>) -> (Duration, O) {
    let start = Instant::now();
    let output = f.get(input);
    let elapsed = start.elapsed();
    (elapsed, output)
}
