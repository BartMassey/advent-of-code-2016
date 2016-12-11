// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

// Advent of Code Day 6

use std::io;
use std::io::prelude::*;
use std::vec::Vec;

extern crate aoc;

// Return the last index of an element in the input that
// has maximal value.
fn max_index(elems: &Vec<usize>) -> usize {
    let mut m = 0;
    let mut m_i = -1;
    for i in 0..elems.len() {
        if elems[i] >= m {
            m = elems[i];
            m_i = i as isize;
        }
    };
    m_i as usize
}

// Return the last index of an element in the input that has
// minimal nonzero value.
fn min_index(elems: &Vec<usize>) -> usize {
    let mut m = 0;
    let mut m_i = -1;
    for i in 0..elems.len() {
        if elems[i] > 0 {
            if m == 0 {
                m = elems[i];
            };
            if elems[i] <= m {
                m = elems[i];
                m_i = i as isize;
            }
        }
    };
    m_i as usize
}

// Count up and show the answer.
pub fn main() {
    let part = aoc::get_part();
    // Set up the state.
    let mut inited = false;
    let mut counters = Vec::new();
    // Read strings from the input file and process them.
    let stdin = io::stdin();
    let reader = io::BufReader::new(stdin);
    for line in reader.lines() {
        let l = line.expect("main: could not read line");
        // Inline initialization: we can't know how long the
        // lines are until we get here.
        if !inited {
            for _ in 0..l.len() {
                counters.push(vec![0usize; 26]);
            };
            inited = true;
        };
        // Bump the relevant counters.
        let mut j = 0;
        for c in l.chars() {
            let i = (c as u8 - 'a' as u8) as usize;
            counters[j][i] += 1;
            j += 1;
        }
    };
    // Find the appropriate characters from the counters
    // and display them.
    for c in counters.iter() {
        let i =
            if part == 1 {
                max_index(&c)
            } else {
                min_index(&c)
            };
        print!("{}", (i as u8 + 'a' as u8) as char);
    }
    print!("\n");
}
