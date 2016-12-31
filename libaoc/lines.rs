// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! Iterator over lines of a file (usually `stdin`) for
//! Advent of Code 2016 solutions.
//!
//! None of this is really necessary, since it's a pretty
//! thin wrapper over existing stuff. But it provides
//! a bit of boilerplate removal and isolates a whole
//! bunch of dependencies.

use std::io::*;
use std::fs::File;

/// The `std::io::Lines` iterator, wrapped so that
/// it will `panic!()` on failure rather than returning
/// a `std::io::Result`.
pub struct InputLines<T: Read> {
    lines: Lines<BufReader<T>>
}

impl <T: Read> InputLines<T> {

    /// Return the wrapped `std::io::Lines` iterator.
    pub fn new(file: T) -> Self {
        InputLines { lines: BufReader::new(file).lines() }
    }
}

impl <T: Read> Iterator for InputLines<T> {
    type Item = String;
    /// Return the next line if any.
    ///
    /// # Panics
    ///
    /// Errors in reading the next line (but not EOF) will
    /// cause a panic here.
    fn next(&mut self) -> Option<String> {
        match self.lines.next() {
            Some(result) => Some(result.expect("could not read input line")),
            None => None
        }
    }
}

/// Get a new iterator over lines of `stdin`.
pub fn input_lines() -> InputLines<Stdin> {
    InputLines::new(stdin())
}

/// Get a new iterator over lines of the file with the given
/// filename, returning an error on failure to open the
/// file.
pub fn input_file_lines(filename: &str) -> Result<InputLines<File>> {
    let file = try!(File::open(filename));
    Ok(InputLines::new(file))
}
