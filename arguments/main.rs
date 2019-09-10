use std::io::{Error, ErrorKind};
use std::num::ParseIntError;

use pi::parallel::parallel_estimate_pi;
use pi::time::time_and_print;

fn help() -> Result<u32, Error> {
    Err(Error::new(ErrorKind::Other, "usage:
pi <int>
Estimate pi with the number of samples"))
}

fn parse_args() -> Result<u32, Error> {
    let args = std::env::args().collect::<Vec<String>>();
    match args.len() {
        2 => { args[1].parse().map_err(|err: ParseIntError| Error::new(ErrorKind::Other, err.to_string())) },
        _ => { help() }
    }
}

fn main() {
    match parse_args() {
        Err(err) => panic!("Couldn't estimate pi: {}", err),
        Ok(num_samples) => time_and_print(num_samples, &parallel_estimate_pi)
    }
}
