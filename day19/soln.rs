// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

// Advent of Code Day 19.

#[cfg(debug_assertions)]
const SHOW: bool = true;
#[cfg(not(debug_assertions))]
const SHOW: bool = false;

extern crate aoc;

struct Elf {
    npresents: usize,
    next_elf: usize
}

pub fn main() {
    let (part, args) = aoc::get_part_args();
    assert!(args.len() == 1);
    let nelves: usize = args[0].parse().expect("could not parse nelves");
    let mut elves = Vec::new();
    for i in 0..nelves {
        let elf = Elf { npresents: 1, next_elf: (i + 1) % nelves };
        elves.push(elf);
    };
    let mut cur_elf = 0;
    let mut victim_parent =
        if part == 1 {
            0
        } else {
            (nelves >> 1) - 1
        };
    let mut living_elves = nelves;
    loop {
        if SHOW {
            println!("{} {}", cur_elf + 1, elves[victim_parent].next_elf + 1);
        };
        let victim_elf = elves[victim_parent].next_elf;
        assert!(cur_elf != victim_elf);
        elves[cur_elf].npresents += elves[victim_elf].npresents;
        elves[victim_parent].next_elf = elves[victim_elf].next_elf;
        living_elves -= 1;
        if living_elves == 1 {
            break;
        }
        cur_elf = elves[cur_elf].next_elf;
        if part == 1 {
            victim_parent = elves[victim_parent].next_elf;
        } else {
            if living_elves & 1 == 0 {
                victim_parent = elves[victim_parent].next_elf;
            };
        };
    };
    assert!(elves[cur_elf].npresents == nelves);
    println!("{}", cur_elf + 1);
}
