use pi::arguments::parse_args;
use pi::parallel::parallel_estimate_pi;
use pi::simple::time_and_print;

fn main() {
    let num_samples = parse_args().unwrap();
    time_and_print(num_samples, &parallel_estimate_pi)
    // match parse_args() {
    //     Err(err) => panic!("Ahh! {}", err),
    //     Ok(num_samples) => time_and_print(num_samples, &parallel_estimate_pi)
    // }
}
