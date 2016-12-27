// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! Advent of Code Day 12.

extern crate aoc;

/// Operand type.
enum Opnd {
    /// Register.
    Reg(usize),
    /// Constant value.
    Const(isize)
}

use self::Opnd::*;

/// Instruction types.
enum Insn {
    /// Copy constant to register.
    CpyConst(isize, usize),
    /// Copy register to register.
    CpyReg(usize, usize),
    /// Add constant to register. Constant should be either
    /// 1 (`INC`) or -1 (`DEC`).
    Add(isize, usize),
    /// Jump relative if register non-zero.
    JNZ(usize, isize),
    /// Unconditional jump relative.
    Jump(isize),
    /// Do nothing.
    Nop
}

use self::Insn::*;

/// Return the operand associated with the given
/// description.
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

/// Check that the description is a register, and
/// return the register number.
fn parse_reg(opnd: &str) -> usize {
    match parse_opnd(opnd) {
        Reg(r) => r,
        _ => panic!(format!("unexpected register {}", opnd))
    }
}

/// Check that the description is a constant, and
/// return it.
fn parse_const(opnd: &str) -> isize {
    match parse_opnd(opnd) {
        Const(i) => i,
        _ => panic!(format!("unexpected const {}", opnd))
    }
}

/// Machine state.
struct State {
    /// Current program counter.
    pc: usize,
    /// Current register values. Note that
    /// the number of registers is solely
    /// a function of the initial state.
    regs: Vec<isize>
}

/// Execute a machine instruction.
fn step(state: &mut State, insn: &Insn) {
    match insn {
        &CpyConst(cnst, reg) => {
            state.regs[reg] = cnst;
            state.pc += 1;
        },
        &CpyReg(reg1, reg2) => {
            state.regs[reg2] = state.regs[reg1];
            state.pc += 1;
        },
        &Add(cnst, reg) => {
            state.regs[reg] += cnst;
            state.pc += 1;
        },
        &JNZ(reg, off) => {
            if state.regs[reg] == 0 {
                state.pc += 1;
            } else {
                let new_pc = state.pc as isize + off;
                assert!(new_pc >= 0);
                state.pc = new_pc as usize;
            }
        },
        &Jump(off) => {
            let new_pc = state.pc as isize + off;
            assert!(new_pc >= 0);
            state.pc = new_pc as usize;
        },
        &Nop => {
            state.pc += 1;
        }
    }
}

/// Run a program..
pub fn main() {
    /// Read the program instructions.
    let mut insns: Vec<Insn> = Vec::new();
    for target in aoc::input_lines() {
        let words = target.split_whitespace().collect::<Vec<&str>>();
        let insn =
            match words[0] {
                "cpy" => {
                    assert!(words.len() == 3);
                    match parse_opnd(words[1]) {
                        Const(i) =>
                            CpyConst(i, parse_reg(words[2])),
                        Reg(n) =>
                            CpyReg(n, parse_reg(words[2]))
                    }
                },
                "inc" => {
                    assert!(words.len() == 2);
                    Add(1, parse_reg(words[1]))
                },
                "dec" => {
                    assert!(words.len() == 2);
                    Add(-1, parse_reg(words[1]))
                },
                "jnz" => {
                    assert!(words.len() == 3);
                    let off = parse_const(words[2]);
                    match parse_opnd(words[1]) {
                        Reg(reg) => JNZ(reg, off),
                        Const(cnst) => match cnst {
                            0 => Nop,
                            _ => Jump(off)
                        }
                    }
                },
                _ => panic!(format!("unrecognized insn {}", words[0]))
            };
        insns.push(insn);
    };

    // Set up the start state and run the instructions.
    let mut state = State{pc: 0, regs: vec![0, 0, 0, 0]};
    while state.pc < insns.len() {
        let pc = state.pc;
        step(&mut state, &insns[pc]);
    };

    // Show register a.
    println!("{}", state.regs[0]);
}
