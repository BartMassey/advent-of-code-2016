// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

// Advent of Code Day 3

pub fn ok_triangle(tri: &mut[i32; 3]) -> bool {
    tri.sort();
    tri[0] + tri [1] > tri[2]
}

