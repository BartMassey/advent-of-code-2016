// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! Advent of Code Day 1.

use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

extern crate aoc;

/// Read the instance description from stdin and walk the
/// route from the origin, returning the endpoint. If
/// `stop_short` is true, stop at the first point at which the
/// path self-intersects and return that as the endpoint,
/// failing if the path does not self-intersect.
///
/// This function probably needs to be split up.
fn traverse(stop_short: bool) -> (isize, isize) {
    // North, West, East, South.
    let directions = [(0, 1), (-1, 0), (0, -1), (1, 0)];

    // Read the single string from the input file.
    let mut stdin = io::stdin();
    let mut s = String::new();
    let _ = stdin.read_to_string(&mut s).unwrap();

    // Set up state variables.
    let mut position = (0, 0);
    let mut trail = HashSet::new();
    trail.insert(position);
    let mut facing = 0;

    // Walk the turns.
    for t in s.trim().split(", ") {
        // Calculate the new facing and distance.
        let turn = t.chars().nth(0).unwrap();
        facing = match turn {
            'L' => (facing + 1) % 4,
            'R' => (facing + 3) % 4,
            _ => panic!("bad direction")
        };
        let step: usize = (&t[1..]).parse().unwrap();

        // Walk the line from current position to new.
        for _ in 0..step {
            position.0 += directions[facing].0;
            position.1 += directions[facing].1;
            if stop_short && trail.contains(&position) {
                // Trail has self-intersected.
                return position;
            }
            trail.insert(position);
        }
    };
    // Finish up.
    if stop_short {
        panic!("did not find self-intersection")
    };
    position
}

/// Do the traversal, then print the manhattan distance from
/// the origin to the endpoint.
pub fn main() {
    let part = aoc::get_part();
    let position = traverse(part == 2);
    println!("{}", position.0.abs() + position.1.abs());
}
