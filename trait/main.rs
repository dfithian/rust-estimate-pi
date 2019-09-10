use pi::arguments::parse_args;
use pi::parallel::parallel_estimate_pi;
use pi::simple::print;
use pi::time::{Thunk, time};

struct MyThunk { num_samples: u32, estimate: fn(u32) -> f64 }

impl Thunk<f64> for MyThunk {
    fn get(&self) -> f64 { (self.estimate)(self.num_samples) }
}

fn main() {
    match parse_args() {
        Err(err) => panic!("Couldn't estimate pi: {}", err),
        Ok(num_samples) => {
            let my_thunk = MyThunk {
                num_samples: num_samples,
                estimate: parallel_estimate_pi
            };
            let (elapsed, pi) = time(&my_thunk);
            print(num_samples, elapsed, pi)
        }
    }
}
