// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! Advent of Code Day 19.

/// Turn this on to show stealing.
const SHOW: bool = false;

extern crate aoc;

/// Info about an elf.
struct Elf {
    /// Number of presents currently held.
    npresents: usize,
    /// Elves are in a circularly-linked list of indices.
    next_elf: usize
}

/// Build the circularly-linked elf list and steal until
/// done.  There can be only one.
pub fn main() {
    let (part, args) = aoc::get_part_args();
    assert!(args.len() == 1);
    let nelves: usize = args[0].parse().expect("could not parse nelves");

    // Build the list.
    let mut elves = Vec::new();
    for i in 0..nelves {
        let elf = Elf { npresents: 1, next_elf: (i + 1) % nelves };
        elves.push(elf);
    };

    // Set up the starting state.
    let mut cur_elf = 0;
    let mut victim_parent =
        if part == 1 {
            0
        } else {
            (nelves >> 1) - 1
        };
    let mut living_elves = nelves;

    // Repeatedly steal.
    loop {
        if SHOW {
            println!("{} {}", cur_elf + 1, elves[victim_parent].next_elf + 1);
        };

        // Steal from current victim.
        let victim_elf = elves[victim_parent].next_elf;
        assert!(cur_elf != victim_elf);
        elves[cur_elf].npresents += elves[victim_elf].npresents;
        elves[victim_parent].next_elf = elves[victim_elf].next_elf;
        living_elves -= 1;
        if living_elves == 1 {
            break;
        }

        // Set up next victim.
        cur_elf = elves[cur_elf].next_elf;
        // To steal "across" for part 2, only advance the
        // victim parent pointer every other turn.
        if part == 1 || living_elves & 1 == 0 {
            victim_parent = elves[victim_parent].next_elf;
        };
    };

    // Show the result.
    assert!(elves[cur_elf].npresents == nelves);
    println!("{}", cur_elf + 1);
}
