#![allow(unused_assignments)]
#![allow(dead_code)]

mod algorithms;
mod command_line;
mod compression;

use compression::decompress_tarball::run;

fn main() {
    run();
}

// Concurrency and Parallelism
// Crossbeam crate
