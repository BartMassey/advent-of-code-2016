// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! Advent of Code Day 23.

extern crate aoc;

/// Read the program and execute it.
pub fn main() {
    let args = aoc::get_args();
    assert!(args.len() == 1);
    let key = args[0].parse().expect("invalid key");

    // Set up state.
    let mut lines = aoc::input_lines();
    let mut insns = aoc::asm(&mut lines);

    // Run the program to completion.
    let mut state = aoc::ExecState::new();
    state.regs[0] = key;
    while state.pc < insns.len() {
        aoc::step(&mut insns, &mut state);
    };

    // Show the contents of register a.
    println!("{}", state.regs[0]);
}
