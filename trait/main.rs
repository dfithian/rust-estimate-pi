use pi::arguments::parse_args;
use pi::parallel::parallel_estimate_pi;
use pi::simple::print;
use pi::time::{Sampler, time2};

fn main() {
    match parse_args() {
        Err(err) => panic!("Couldn't estimate pi: {}", err),
        Ok(num_samples) => {
            let sampler = Sampler { f: parallel_estimate_pi };
            let (elapsed, pi) = time2(num_samples, &sampler);
            print(num_samples, elapsed, pi)
        }
    }
}
