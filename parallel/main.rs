use pi::parallel::parallel_estimate_pi;
use pi::time::time_and_print;

fn main() {
    time_and_print(1000, &parallel_estimate_pi)
}
