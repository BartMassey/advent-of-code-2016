// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

// Advent of Code Day 3 Part 2

mod soln;
pub use soln::ok_triangle;

use std::io;
use std::io::prelude::*;

fn main() {
    // State for algorithm.
    let mut count = 0;
    let mut index = 0;
    let mut buffer = [[0i32; 3]; 3];
    // Read strings from the input file and process them.
    // Group lines into threes and process each group.
    let stdin = io::stdin();
    let reader = io::BufReader::new(stdin);
    for line in reader.lines() {
        let strs = line.unwrap();
        let line_strs = strs.split_whitespace().collect::<Vec<_>>();
        if line_strs.len() != 3 {
            panic!("line does not have three lengths");
        };
        for i in 0..3 {
            buffer[i][index] = line_strs[i].parse().unwrap();
        };
        index += 1;
        if index == 3 {
            // Ready to process group.
            for i in 0..3 {
                if ok_triangle(&mut buffer[i]) {
                    count += 1;
                }
            }
            index = 0;
        }
    };
    if index != 0 {
        panic!("uneven number of lines in input");
    }
    print!("{}\n", count);
}
