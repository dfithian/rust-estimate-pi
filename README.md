# Estimating Pi with Rust

This project is an experiment helping me to learn Rust by estimating Pi.

# Modules

* `sample.rs` contains utilities for sampling random numbers within the unit circle.
* `simple.rs` contains a single-threaded estimation mechanic that sums up the average of the
  coordinates from the sample that are in versus not-in the unit circle.
* `parallel.rs` contains a parallelized estimation mechanic of the function in `simple.rs`.
* `time.rs` contains utilities for timing and printing the different estimation mechanics.

# Other features

* `lib.rs` re-exports modules
* `simple/main.rs`, `parallel/main.rs`, `arguments/main.rs` are entrypoints for the different
  functions. `arguments/main.rs` parses command-line arguments but otherwise does the same thing as
  `parallel/main.rs`.
* `Cargo.toml` shows how to use libraries with binaries and to define multiple binaries in a single
  project.
