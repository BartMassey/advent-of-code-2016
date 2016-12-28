// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! Advent of Code Day 23.

/// Set to true to trace execution.
const TRACE: bool = false;

extern crate aoc;

/// Operand types.
#[derive(Clone, Copy)]
enum Opnd {
    /// Register.
    Reg(usize),
    /// Constant.
    Const(isize)
}

use self::Opnd::*;

/// Instruction types.
#[derive(Clone, Copy)]
enum Insn {
    /// Copy left-to-right.
    Cpy(Opnd, Opnd),
    /// Add left constant to right operand.  Usually
    /// constant is either 1 (INC) or -1 (DEC).
    Add(isize, Opnd),
    /// Jump relative with offset given by right operand if
    /// left operand is nonzero.
    JNZ(Opnd, Opnd),
    /// Toggle the instruction at given offset.
    Tgl(Opnd)
}

use self::Insn::*;

/// Parse and construct an operand.
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
    /// Current program counter.
    pc: usize,
    /// Current register values.
    regs: Vec<isize>
}

/// Return the value of given operand in given state.
fn eval(state: &State, opnd: Opnd) -> isize {
    match opnd {
        Reg(r) => state.regs[r],
        Const(c) => c
    }
}

/// Make printable string describing operand.
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

/// Execute the instruction at the current PC in the given
/// state.
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
                &JNZ(rc1, rc2) => Cpy(rc1, rc2),
                &Cpy(rc1, rc2) => JNZ(rc1, rc2)
            };
            state.pc += 1;
        },
    }
}

/// Read the program and execute it.
pub fn main() {
    let args = aoc::get_args();
    assert!(args.len() == 1);
    let key = args[0].parse().expect("invalid key");

    // Set up state.
    let mut insns: Vec<Insn> = Vec::new();

    // Parse instructions.
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
                _ => panic!(format!("unrecognized insn {}", words[0]))
            };
        insns.push(insn);
    };

    // Run the program to completion.
    let mut state = State{pc: 0, regs: vec![key, 0, 0, 0]};
    while state.pc < insns.len() {
        step(&mut insns, &mut state);
    };

    // Show the contents of register a.
    println!("{}", state.regs[0]);
}
