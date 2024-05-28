extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;

fn main() {
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        return;
    }
}
