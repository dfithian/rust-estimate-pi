use pi::arguments::parse_args;
use pi::parallel::parallel_estimate_pi;
use pi::time::time_and_print;

fn main() {
    match parse_args() {
        Err(err) => panic!("Couldn't estimate pi: {}", err),
        Ok(num_samples) => time_and_print(num_samples, &parallel_estimate_pi)
    }
}
