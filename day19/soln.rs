// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

// Advent of Code Day 19.

const SHOW: bool = false;

extern crate aoc;

struct Elf {
    npresents: usize,
    next_elf: usize
}

pub fn main() {
    let (part, args) = aoc::get_part_args();
    assert!(args.len() == 1);
    let nelves = args[0].parse().expect("could not parse nelves");
    let mut living_elves = nelves;
    let mut elves = Vec::new();
    for i in 0..nelves {
        let elf = Elf { npresents: 1, next_elf: (i + 1) % nelves };
        elves.push(elf);
    };
    let mut cur_elf = 0;
    while living_elves > 1 {
        let mut victim_elf = cur_elf;
        let mut victim_parent_elf = cur_elf;
        if part == 1 {
            victim_elf = elves[victim_elf].next_elf;
        } else {
            for _ in 0..living_elves>>1 {
                victim_parent_elf = victim_elf;
                victim_elf = elves[victim_elf].next_elf;
            };
            if SHOW {
                print!("{} {}\n", cur_elf + 1, victim_elf + 1);
            };
        };
        elves[cur_elf].npresents += elves[victim_elf].npresents;
        elves[victim_parent_elf].next_elf = elves[victim_elf].next_elf;
        living_elves -= 1;
        cur_elf = elves[cur_elf].next_elf;
    };
    assert!(elves[cur_elf].npresents == nelves);
    print!("{}\n", cur_elf + 1);
}
