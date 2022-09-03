// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! Advent of Code Day 15.

extern crate aoc;
extern crate captures_at;
use captures_at::CapturesAtExt;
extern crate regex;

/// Read the disc descriptions, returning
/// a vector of positions/start pairs.
fn read_discs() -> Vec<(isize, isize)> {
    // Set up the matching.
    let disc_pat = regex::Regex::new(
        r"^Disc #([0-9]+) has ([0-9]+) positions; at time=0, it is at position ([0-9]+).$",
    )
    .expect("main: could not compile disc pattern");

    // Set up the state.
    let mut discs = Vec::new();

    // Process the disc descriptions.
    for (ndiscs, target) in aoc::input_lines().enumerate() {
        // Parse the description.
        let parts = disc_pat.captures(&target).expect("malformed line");
        let disc_number: isize = parts
            .at(1)
            .expect("main: could not find disc number")
            .parse()
            .expect("main: could not parse disc number");
        let disc_positions = parts
            .at(2)
            .expect("main: could not find disc positions")
            .parse()
            .expect("main: could not parse disc positions");
        let disc_start = parts
            .at(3)
            .expect("main: could not find disc start")
            .parse()
            .expect("main: could not parse disc start");
        if disc_number != ndiscs as isize + 1 {
            panic!("unexpected disc number");
        };

        // Save the disc info.
        discs.push((disc_positions, disc_start));
    }
    discs
}

/// At the given start time `t0`, try to drop the ball
/// through all the discs and return success or failure.
fn search(discs: &[(isize, isize)], t0: isize) -> bool {
    for (j, &d) in discs.iter().enumerate() {
        let (cj, qj) = d;
        let t = t0 + 1 + j as isize;
        let p = (t + qj) % cj;
        if p != 0 {
            return false;
        };
    }
    true
}

/// Solve the problem.
pub fn main() {
    let discs = read_discs();
    for t in 0..std::isize::MAX {
        if search(&discs, t) {
            println!("{}", t);
            return;
        }
    }
    panic!("no solution found");
}
