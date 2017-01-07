// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! Advent of Code Day 25.

/// For part 2, number of cycles of correct output to
/// require before declaring victory.
const NCYCLES: usize = 10;

/// For part 2, number of steps to run without success
/// before declaring defeat.
const NSTEPS: usize = 1000000;

extern crate aoc;


/// Read the program and brute-force the input that will
/// produce the correct output, or at least heuristically
/// look for enough of said infinite output to get
/// reasonable confidence.
pub fn main() {
    /// Input the program.
    let mut lines = aoc::input_lines();
    let mut insns = aoc::asm(&mut lines);

    /// Load up the heuristic target vector.
    let mut target = Vec::new();
    for _ in 0..NCYCLES {
        target.push(0);
        target.push(1);
    };

    /// Try executing program with successive keys,
    /// failing that key when it produces an incorrect
    /// output, and stopping when the key has produced
    /// enough correct outputs.
    for key in 1..std::isize::MAX {
        let mut state = aoc::ExecState::new();
        state.regs[0] = key;
        let mut steps = 0;
        let mut old_nout = 0;
        while state.pc < insns.len() && steps < NSTEPS {
            aoc::step(&mut insns, &mut state);
            let nout = state.out.len();
            if nout > old_nout {
                if target[nout - 1] != state.out[nout - 1] {
                    break;
                };
                if nout >= 2 * NCYCLES {
                    assert!(state.out == target);
                    println!("{}", key);
                    return;
                };
                old_nout = nout;
            };
            steps += 1;
        };
    };
    panic!("no solution found");
}
