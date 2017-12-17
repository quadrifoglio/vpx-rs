//! Example usage of the vpx-rs library to build a simple VP8 decoder.
//! This program takes as input the path to an IVF file that contains VP8 frames, and writes the
//! decoded frames to the specified output folder.

extern crate vpx;

use std::env;
use vpx::Decoder;

fn main() {
    let input = env::args().nth(1).expect("Please specify an input IVF file");
    let output = env::args().nth(2).expect("Please specify an output folder for the decoded frames");

    let decoder = Decoder::new();
}
