use std::io::{Error, ErrorKind};
use std::num::ParseIntError;

// print usage as an error, since we'll want to return non-zero exit code
fn help<A>(name: &str) -> Result<A, Error> {
    Err(Error::new(ErrorKind::Other, format!("usage:
{} <int>
Estimate pi with the number of samples", name)))
}

// parse args stupidly and expect the size parameter to be in the second position
pub fn parse_args() -> Result<u32, Error> {
    // why can't we use &str
    // why can't we use &String
    let args = std::env::args().collect::<Vec<String>>();
    match args.len() {
        2 => { args[1].parse().map_err(|err: ParseIntError| Error::new(ErrorKind::Other, err.to_string())) },
        _ => { help(&args[0]) },
    }
}
