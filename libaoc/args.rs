// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! Argument handling for Advent of Code 2016 solutions.
//!
//! This module is just an AoC-specific argument parser.
//! It handles part specifiers specially, and provides
//! some convenience for processing normal arguments.

use std;

/// Find out whether we're handling part 1 or part 2.
/// Fail if extra arguments are passed.
pub fn get_part() -> usize {
    let mut argv = std::env::args();
    assert!(argv.len() == 2);
    let part = argv.nth(1).expect("get_part: bad part argument")
               .parse().expect("get_part: part argument is not a number");
    assert!(part == 1 || part == 2);
    part
}

/// Find out whether we're handling part 1 or part 2, and what
/// other arguments have been supplied.
pub fn get_part_args() -> (usize, Vec<String>) {
    let mut argv = std::env::args();
    assert!(argv.len() >= 2);
    let part = argv.nth(1).expect("get_part_args: bad part argument")
               .parse().expect("get_part_args: part argument is not a number");
    assert!(part == 1 || part == 2);
    let args = argv.collect::<Vec<String>>();
    (part, args)
}

/// Get just arguments for a problem where both parts are the same
/// (i.e., there is no "part" argument).
pub fn get_args() -> Vec<String> {
    let mut argv = std::env::args();
    argv.next();
    argv.collect::<Vec<String>>()
}

