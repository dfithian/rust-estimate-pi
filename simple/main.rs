use pi::simple::simple_estimate_pi;
use pi::time::time_and_print;

fn main() {
    time_and_print(1000, &simple_estimate_pi)
}
