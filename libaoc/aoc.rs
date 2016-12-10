// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

// Library for Advent of Code 2016 solutions.

pub fn parseargs() -> (bool, Vec<String>) {
    // Find out whether we're handling part1 or part2.
    let mut argv = std::env::args();
    assert!(argv.len() == 2);
    let part1 = match argv.nth(1).unwrap().parse().unwrap() {
        1 => true,
        2 => false,
        _ => panic!("bad problem number")
    };
    // Pile up the rest and give it back.
    let rest = argv.collect::<Vec<String>>();
    (part1, rest)
}
