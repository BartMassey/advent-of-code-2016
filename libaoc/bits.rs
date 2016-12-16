// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

// Bit operations for Advent of Code 2016 solutions.

use std::mem;

pub fn popcount(mut v: usize) -> usize {
    let mut count = 0;
    for _ in 0..8*mem::size_of::<usize>() {
        count += v & 1;
        v >>= 1;
    }
    return count;
}
