// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

// Advent of Code Day 5 Part 1

mod soln;
pub use soln::soln;

// XXX I *hate* hardcoding the input, but it seems reasonable here.
fn main() {
    soln(&"wtnhxymk", false);
}
