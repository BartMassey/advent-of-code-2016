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

/// Turn on to trace execution.
const TRACE: bool = false;

extern crate aoc;

/// Operand types.
#[derive(Clone, Copy)]
enum Opnd {
    /// Register.
    Reg(usize),
    /// Constant value.
    Const(isize)
}

use self::Opnd::*;

/// Instruction types.
#[derive(Clone, Copy)]
enum Insn {
    /// Copy left to right.
    Cpy(Opnd, Opnd),
    /// Add immediate to right operand.
    Add(isize, Opnd),
    /// Jump if left operand nonzero relative offset by
    /// right operand.
    JNZ(Opnd, Opnd),
    /// Toggle the instruction relative offset by right
    /// operand.
    Tgl(Opnd),
    /// Output the given value to the output vector.
    Out(Opnd)
}

use self::Insn::*;

/// Parse an operand description and return its operand.
fn parse_opnd(opnd: &str) -> Opnd {
    match opnd.parse::<isize>() {
        Ok(i) => Const(i),
        Err(_) => {
            assert!(opnd.len() == 1);
            let name = opnd.chars().next().expect("empty operand");
            let num = name as isize - 'a' as isize;
            assert!(num >= 0 && num < 26);
            Reg(num as usize)
        }
    }
}

/// Machine state.
struct State {
    /// Program counter.
    pc: usize,
    /// Register contents.
    regs: Vec<isize>,
    /// List of outputs.
    out: Vec<isize>
}

/// Return the value of the given operand in the given
/// state.
fn eval(state: &State, opnd: Opnd) -> isize {
    match opnd {
        Reg(r) => state.regs[r],
        Const(c) => c
    }
}


/// Return a string describing the given operand.
fn rcs(opnd: Opnd) -> String {
    match opnd {
        Const(c) => c.to_string(),
        Reg(r) => {
            let mut s = String::new();
            s.push(('a' as u8 + r as u8) as char);
            s
        }
    }
}

/// Execute the instruction at the current pc.
fn step(insns: &mut Vec<Insn>, state: &mut State) {
    match insns[state.pc] {
        Cpy(rc1, Reg(r2)) => {
            if TRACE {
                println!("{}: cpy {} {}", state.pc, rcs(rc1), rcs(Reg(r2)));
            };
            state.regs[r2] = eval(state, rc1);
            state.pc += 1;
        },
        Cpy(rc1, rc2) => {
            if TRACE {
                println!("{}: cpy* {} {}", state.pc, rcs(rc1), rcs(rc2));
            };
            state.pc += 1;
        },
        Add(cnst, Reg(reg)) => {
            if TRACE {
                match cnst {
                    1 => println!("{}: inc {}", state.pc, rcs(Reg(reg))),
                    -1 => println!("{}: dec {}", state.pc, rcs(Reg(reg))),
                    _ => panic!("invalid add")
                }
            };
            state.regs[reg] += cnst;
            state.pc += 1;
        },
        Add(_, _) => {
            panic!("add with const target");
        },
        JNZ(rc1, rc2) => {
            if TRACE {
                println!("{}: jnz {} {}", state.pc, rcs(rc1), rcs(rc2));
            };
            let test = eval(state, rc1);
            if test != 0 {
                let off = eval(state, rc2);
                let new_pc = state.pc as isize + off;
                assert!(new_pc >= 0);
                state.pc = new_pc as usize;
                return;
            }
            state.pc += 1;
        },
        Tgl(rc) => {
            if TRACE {
                println!("{}: tgl {}", state.pc, rcs(rc));
            };
            let off = eval(state, rc);
            if off == 0 {
                state.pc += 1;
                return;
            };
            let new_pc = state.pc as isize + off;
            assert!(new_pc >= 0);
            let new_pc = new_pc as usize;
            if new_pc >= insns.len() {
                state.pc += 1;
                return;
            };
            insns[new_pc] = match &insns[new_pc] {
                &Add(c, reg) => Add(-c, reg),
                &Tgl(reg) => Add(1, reg),
                &Out(rc) => Add(1, rc),
                &JNZ(rc1, rc2) => Cpy(rc1, rc2),
                &Cpy(rc1, rc2) => JNZ(rc1, rc2)
            };
            state.pc += 1;
        },
        Out(rc) => {
            if TRACE {
                println!("{}: out {}", state.pc, rcs(rc));
            };
            let out = eval(state, rc);
            state.out.push(out);
            state.pc += 1;
        }
    }
}

/// Read the program and brute-force the input that will
/// produce the correct output, or at least heuristically
/// look for enough of said infinite output to get
/// reasonable confidence.
pub fn main() {
    /// Input the program.
    let mut insns: Vec<Insn> = Vec::new();
    for target in aoc::input_lines() {
        let words = target.split_whitespace().collect::<Vec<&str>>();
        let insn =
            match words[0] {
                "cpy" => {
                    assert!(words.len() == 3);
                    Cpy(parse_opnd(words[1]), parse_opnd(words[2]))
                },
                "inc" => {
                    assert!(words.len() == 2);
                    Add(1, parse_opnd(words[1]))
                },
                "dec" => {
                    assert!(words.len() == 2);
                    Add(-1, parse_opnd(words[1]))
                },
                "jnz" => {
                    assert!(words.len() == 3);
                    JNZ(parse_opnd(words[1]), parse_opnd(words[2]))
                },
                "tgl" => {
                    assert!(words.len() == 2);
                    Tgl(parse_opnd(words[1]))
                },
                "out" => {
                    assert!(words.len() == 2);
                    Out(parse_opnd(words[1]))
                },
                _ => panic!(format!("unrecognized insn {}", words[0]))
            };
        insns.push(insn);
    };

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
        let mut state = State{
            pc: 0,
            regs: vec![key, 0, 0, 0],
            out: Vec::new()
        };
        let mut steps = 0;
        let mut old_nout = 0;
        while state.pc < insns.len() && steps < NSTEPS {
            step(&mut insns, &mut state);
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
