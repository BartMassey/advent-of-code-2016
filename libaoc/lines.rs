// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

// Iterator for lines of stdin for Advent of Code 2016 solutions.

use std::io::{Stdin, BufReader, stdin, Lines};
use std::io::prelude::*;

#[allow(dead_code)]
pub struct InputLines {
    lines: Lines<BufReader<Stdin>>
}

#[allow(dead_code)]
impl InputLines {
    pub fn new() -> Self {
        InputLines { lines: BufReader::new(stdin()).lines() }
    }
}

impl Iterator for InputLines {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        match self.lines.next() {
            Some(result) => Some(result.expect("could not read input line")),
            None => None
        }
    }
}

#[allow(dead_code)]
pub fn input_lines() -> InputLines {
    InputLines::new()
}
