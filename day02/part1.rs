// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

// Advent of Code Day 2 Part 1

mod soln;
pub use soln::print_code;

fn main() {
    // XXX Yes, this is gross. But easy.
    let board = vec![
        vec!['.', '.', '.', '.', '.'],
        vec!['.', '1', '2', '3', '.'],
        vec!['.', '4', '5', '6', '.'],
        vec!['.', '7', '8', '9', '.'],
        vec!['.', '.', '.', '.', '.'] ];
    print_code(&board);
}
