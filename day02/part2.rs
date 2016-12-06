// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

// Advent of Code Day 2 Part 2

mod soln;
pub use soln::print_code;

fn main() {
    let board = vec![
        vec!['.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '1', '.', '.', '.'],
        vec!['.', '.', '2', '3', '4', '.', '.'],
        vec!['.', '5', '6', '7', '8', '9', '.'],
        vec!['.', '.', 'A', 'B', 'C', '.', '.'],
        vec!['.', '.', '.', 'D', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.'] ];
    print_code(&board);
}
