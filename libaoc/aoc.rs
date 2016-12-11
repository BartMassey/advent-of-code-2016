// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

// Library for Advent of Code 2016 solutions.

// Find out whether we're handling part1 or part2.
pub fn get_part() -> usize {
    let mut argv = std::env::args();
    assert!(argv.len() == 2);
    let part = argv.nth(1).expect("get_part: bad part argument")
               .parse().expect("get_part: part argument is not a number");
    assert!(part == 1 || part == 2);
    part
}

// Find out whether we're handling part1 or part2, and what
// other arguments have been supplied.
pub fn get_part_args() -> (usize, Vec<String>) {
    let mut argv = std::env::args();
    assert!(argv.len() >= 2);
    let part = argv.nth(1).expect("get_part_args: bad part argument")
               .parse().expect("get_part_args: part argument is not a number");
    assert!(part == 1 || part == 2);
    let args = argv.collect::<Vec<String>>();
    (part, args)
}
