// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! Advent of Code Day 12.

extern crate aoc;

/// Run a program..
pub fn main() {
    /// Read the program instructions.
    let mut lines = aoc::input_lines();
    let mut insns = aoc::asm(&mut lines);

    // Set up the start state and run the instructions.
    let mut state = aoc::ExecState::new();
    while state.pc < insns.len() {
        aoc::step(&mut insns, &mut state);
    };

    // Show register a.
    println!("{}", state.regs[0]);
}
