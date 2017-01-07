// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! Advent of Code Day 6.

extern crate aoc;

/// Return the first index of an element in the input that
/// has maximal value. Input must have at least one element.
fn max_index(elems: &[usize]) -> usize {
    let mut m = elems[0];
    let mut m_i = 0;
    for i in 1..elems.len() {
        if elems[i] > m {
            m = elems[i];
            m_i = i;
        }
    };
    m_i
}

/// Return the first index of an element in the input that
/// has minimal nonzero value. Input must have at least one element.
fn min_index(elems: &[usize]) -> usize {
    let mut m = elems[0];
    let mut m_i = 0;
    for i in 1..elems.len() {
        if elems[i] > 0 {
            if m == 0 {
                m = elems[i];
            };
            if elems[i] < m {
                m = elems[i];
                m_i = i;
            }
        }
    };
    m_i
}

/// Count up and show the answer.
pub fn main() {
    let part = aoc::get_part();

    // Set up the state.
    let mut inited = false;
    let mut counters = Vec::new();

    // Read strings from the input file and process them.
    for l in aoc::input_lines() {
        // Inline initialization: we can't know how long the
        // lines are until we get here.
        if !inited {
            for _ in 0..l.len() {
                counters.push(vec![0usize; 26]);
            };
            inited = true;
        };

        // Bump the relevant counters.
        for (j, c) in l.chars().enumerate() {
            let i = (c as u8 - b'a') as usize;
            counters[j][i] += 1;
        }
    };

    // Find the appropriate characters from the counters
    // and display them.
    for c in &counters {
        let i =
            if part == 1 {
                max_index(c)
            } else {
                min_index(c)
            };
        print!("{}", (i as u8 + b'a') as char);
    }
    println!("");
}
