// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! Advent of Code Day 3.

extern crate aoc;

/// Return true if the given three lengths form
/// a triangle. Sort the lengths as a side effect.
fn ok_triangle(tri: &mut[isize; 3]) -> bool {
    tri.sort();
    tri[0] + tri [1] > tri[2]
}

/// Count the number of triangles in the given input lines.
fn count_triangles() -> usize {
    let mut count = 0;
    for strs in aoc::input_lines() {
        // Read and parse the input.
        let line_strs = strs.split_whitespace().collect::<Vec<_>>();
        if line_strs.len() != 3 {
            panic!("line does not have three lengths");
        };
        let mut tri: [isize; 3] = [0; 3];
        for i in 0..3 {
            tri[i] = line_strs[i].parse().expect("part1: tri had non-integer");
        };

        // Adjust the count.
        if ok_triangle(&mut tri) {
            count += 1;
        }
    };
    count
}

/// Process the triangle specs in the transposed way.
fn count_transposed_triangles() -> usize {
    // State for algorithm.
    let mut count = 0;
    let mut index = 0;
    let mut buffer = [[0isize; 3]; 3];
    // Read strings from the input file and process them.
    // Group lines into threes and process each group.
    for strs in aoc::input_lines() {
        // Read and parse lines.
        let line_strs = strs.split_whitespace().collect::<Vec<_>>();
        if line_strs.len() != 3 {
            panic!("line does not have three lengths");
        };
        for i in 0..3 {
            buffer[i][index] = line_strs[i].parse()
                .expect("part2: tri contained non-integer");
        };

        // Process a group if one is ready.
        index += 1;
        if index == 3 {
            for i in 0..3 {
                if ok_triangle(&mut buffer[i]) {
                    count += 1;
                }
            }
            index = 0;
        }
    };

    // Check state and return count.
    if index != 0 {
        panic!("uneven number of lines in input");
    };
    count
}

/// Count some triangles.
pub fn main() {
    let part = aoc::get_part();
    let count =
        if part == 1 {
            count_triangles()
        } else if part == 2 {
            count_transposed_triangles()
        } else {
            panic!("unknown part");
        };
    println!("{}", count);
}
