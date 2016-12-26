// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! Iterator over lines of file (usually `stdin`) for Advent
//! of Code 2016 solutions.

use std::io::{Stdin, Read, BufRead, BufReader, stdin, Lines, Result};
use std::fs::File;

/// The `std::io::Lines` iterator, wrapped so that
/// it will `panic!()` on failure rather than returning
/// a `std::io::Result`.
#[allow(dead_code)]
pub struct InputLines<T: Read> {
    lines: Lines<BufReader<T>>
}

#[allow(dead_code)]
impl <T: Read> InputLines<T> {

    /// Return the wrapped `std::io::Lines` iterator.
    pub fn new(file: T) -> Self {
        InputLines { lines: BufReader::new(file).lines() }
    }
}

impl <T: Read> Iterator for InputLines<T> {
    type Item = String;

    /// Return the next line if any, calling `panic!()` on
    /// errors in reading.
    fn next(&mut self) -> Option<String> {
        match self.lines.next() {
            Some(result) => Some(result.expect("could not read input line")),
            None => None
        }
    }
}

/// Get a new iterator over lines of `stdin`.
#[allow(dead_code)]
pub fn input_lines() -> InputLines<Stdin> {
    InputLines::new(stdin())
}

/// Get a new iterator over lines of the file with the given filename,
/// returning an error on failure to open the file.
#[allow(dead_code)]
pub fn input_file_lines(filename: &str) -> Result<InputLines<File>> {
    let file = try!(File::open(filename));
    Ok(InputLines::new(file))
}
