// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

// Iterator for lines of stdin for Advent of Code 2016 solutions.

use std::io::{Stdin, Read, BufRead, BufReader, stdin, Lines, Result};
use std::fs::File;

#[allow(dead_code)]
pub struct InputLines<T: Read> {
    lines: Lines<BufReader<T>>
}

#[allow(dead_code)]
impl <T: Read> InputLines<T> {
    pub fn new(file: T) -> Self {
        InputLines { lines: BufReader::new(file).lines() }
    }
}

impl <T: Read> Iterator for InputLines<T> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        match self.lines.next() {
            Some(result) => Some(result.expect("could not read input line")),
            None => None
        }
    }
}

#[allow(dead_code)]
pub fn input_lines() -> InputLines<Stdin> {
    InputLines::new(stdin())
}

#[allow(dead_code)]
pub fn input_file_lines(filename: &str) -> Result<InputLines<File>> {
    let file = try!(File::open(filename));
    Ok(InputLines::new(file))
}
