// I've decided to, over time, move some often used functionality to
// a little library.

use std::fs::read_to_string;

pub fn read_raw_input(file: &str) -> String {
    return read_to_string(file).unwrap();
}

