// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

// Advent of Code Day 3 Part 1

mod soln;
pub use soln::ok_triangle;

use std::io;
use std::io::prelude::*;

fn main() {
    // Read strings from the input file and process them.
    let stdin = io::stdin();
    let reader = io::BufReader::new(stdin);
    let mut count = 0;
    for line in reader.lines() {
        let strs = line.unwrap();
        let line_strs = strs.split_whitespace().collect::<Vec<_>>();
        if line_strs.len() != 3 {
            panic!("line does not have three lengths");
        };
        let mut tri: [i32; 3] = [0; 3];
        for i in 0..3 {
            tri[i] = line_strs[i].parse().unwrap();
        };
        if ok_triangle(&mut tri) {
            count += 1;
        }
    };
    print!("{}\n", count);
}
