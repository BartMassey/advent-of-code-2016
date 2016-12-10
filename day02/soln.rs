// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

// Advent of Code Day 2

use std::io;
use std::io::prelude::*;

extern crate aoc;

// Take a starting point and a line of traversal instructions
// and return the ending position.
fn walk_line(board: &Vec<Vec<char>>, n_start: (usize, usize),
             insns: &str) -> (usize, usize) {
    let mut n = n_start;
    // Walk over the instructions trying moves.
    for c in insns.chars() {
        let next_n = match c {
            'U' => (n.0, n.1 - 1),
            'D' => (n.0, n.1 + 1),
            'L' => (n.0 - 1, n.1),
            'R' => (n.0 + 1, n.1),
            _ => panic!("bad direction")
        };
        if board[next_n.1][next_n.0] != '.' {
            n = next_n;
        }
    };
    return n
}

// Print the bathroom code for the instance on stdin.
pub fn print_code(board: &Vec<Vec<char>>) {
    let c = board.len() / 2 + 1;
    let mut n = (c, c);
    // Read strings from the input file and process them.
    let stdin = io::stdin();
    let reader = io::BufReader::new(stdin);
    for line in reader.lines() {
        n = walk_line(board, n, &line.unwrap());
        print!("{}", board[n.1][n.0]);
    }
    print!("\n");
}

pub fn main() {
    let (part1, _) = aoc::parseargs();
    // It's gross, but it works.
    let board = if part1 {
        vec![ vec!['.', '.', '.', '.', '.'],
              vec!['.', '1', '2', '3', '.'],
              vec!['.', '4', '5', '6', '.'],
              vec!['.', '7', '8', '9', '.'],
              vec!['.', '.', '.', '.', '.']]
    } else {
        vec![vec!['.', '.', '.', '.', '.', '.', '.'],
             vec!['.', '.', '.', '1', '.', '.', '.'],
             vec!['.', '.', '2', '3', '4', '.', '.'],
             vec!['.', '5', '6', '7', '8', '9', '.'],
             vec!['.', '.', 'A', 'B', 'C', '.', '.'],
             vec!['.', '.', '.', 'D', '.', '.', '.'],
             vec!['.', '.', '.', '.', '.', '.', '.']]
    };
    print_code(&board);
}
