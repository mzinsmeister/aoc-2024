// I've decided to, over time, move some often used functionality to
// a little library.

use std::{hash::Hash, fs::read_to_string, ops::{Deref, DerefMut}};


#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn into_dxdy(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0)
        }
    }

    pub fn rotate_clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down
        }
    }

    pub fn rotate_counter_clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up
        }
    }

    pub fn into_char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>'
        }
    }
}


pub struct AocInput {
    pub raw: String,
}

impl AocInput {
    pub fn split_once(&self, delim: &str) -> (AocInput, AocInput) {
        let (left, right) = self.trim_last_newline().split_once(delim).unwrap();

        return (AocInput { raw: left.to_string() }, AocInput { raw: right.to_string() });
    }

    pub fn trim_last_newline(&self) -> &str {
        return &self.raw.trim_end_matches('\n');
    }

    pub fn into_2d_chars(&self) -> Vec<Vec<char>> {
        return self.trim_last_newline().lines().map(|l| l.chars().collect()).collect();
    }

    pub fn into_2d_nums<T>(&self) -> Vec<Vec<T>> where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug {
        return self.trim_last_newline().lines().map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect()).collect();
    }

    pub fn into_mapping<K: Eq+Hash, KF, V, VF>(&self, kf: KF, vf: VF, map_delim: &str, values_delim: &str) -> Vec<(K, Vec<V>)>
            where KF: Fn(&str) -> K, VF: Fn(&str) -> V {

        let mut map = Vec::new();
        for line in self.trim_last_newline().lines() {
            let (k, v) = line.split_once(map_delim).unwrap();
            let k = kf(k);
            let mut values = Vec::new();
            for v in v.split(values_delim) {
                let v = vf(v);
                values.push(v);
            }
            map.push((k, values));
        }
        return map;
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

