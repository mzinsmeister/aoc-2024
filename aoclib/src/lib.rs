// I've decided to, over time, move some often used functionality to
// a little library.

use std::{fs::read_to_string, ops::{Deref, DerefMut}};

pub struct AocInput {
    pub raw: String,
}

impl AocInput {
    pub fn trim_last_newline(&self) -> &str {
        return &self.raw.trim_end_matches('\n');
    }

    pub fn into_2d_chars(&self) -> Vec<Vec<char>> {
        return self.trim_last_newline().lines().map(|l| l.chars().collect()).collect();
    }
}

pub fn read_input(file: &str) -> AocInput {
    return AocInput { raw: read_to_string(file).unwrap() };
}

impl Deref for AocInput {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl DerefMut for AocInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.raw
    }
}

