// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

// Advent of Code Day 12.

use std::io;
use std::io::prelude::*;

extern crate aoc;

enum Opnd {
    Reg(usize),
    Const(isize)
}

enum Insn {
    CpyConst(isize, usize),
    CpyReg(usize, usize),
    Add(isize, usize),
    JNZ(usize, isize),
    Jump(isize),
    Nop
}

fn parse_opnd(opnd: &str) -> Opnd {
    match opnd.parse::<isize>() {
        Ok(i) => Opnd::Const(i),
        Err(_) => {
            assert!(opnd.len() == 1);
            let name = opnd.chars().next().expect("empty operand");
            let num = name as isize - 'a' as isize;
            assert!(num >= 0 && num < 26);
            Opnd::Reg(num as usize)
        }
    }
}

fn parse_reg(opnd: &str) -> usize {
    match parse_opnd(opnd) {
        Opnd::Reg(r) => r,
        _ => panic!(format!("unexpected register {}", opnd))
    }
}

fn parse_const(opnd: &str) -> isize {
    match parse_opnd(opnd) {
        Opnd::Const(i) => i,
        _ => panic!(format!("unexpected const {}", opnd))
    }
}

struct State {
    pc: usize,
    regs: Vec<isize>
}

fn step(state: &mut State, insn: &Insn) {
    match insn {
        &Insn::CpyConst(cnst, reg) => {
            state.regs[reg] = cnst;
            state.pc += 1;
        },
        &Insn::CpyReg(reg1, reg2) => {
            state.regs[reg2] = state.regs[reg1];
            state.pc += 1;
        },
        &Insn::Add(cnst, reg) => {
            state.regs[reg] += cnst;
            state.pc += 1;
        },
        &Insn::JNZ(reg, off) => {
            if state.regs[reg] == 0 {
                state.pc += 1;
            } else {
                let new_pc = state.pc as isize + off;
                assert!(new_pc >= 0);
                state.pc = new_pc as usize;
            }
        },
        &Insn::Jump(off) => {
            let new_pc = state.pc as isize + off;
            assert!(new_pc >= 0);
            state.pc = new_pc as usize;
        },
        &Insn::Nop => {
            state.pc += 1;
        }
    }
}

pub fn main() {
    let mut insns: Vec<Insn> = Vec::new();
    let stdin = io::stdin();
    let reader = io::BufReader::new(stdin);
    for line in reader.lines() {
        let target = line.expect("main: could not read line");
        let words = target.split_whitespace().collect::<Vec<&str>>();
        let insn =
            match words[0] {
                "cpy" => {
                    assert!(words.len() == 3);
                    match parse_opnd(words[1]) {
                        Opnd::Const(i) => Insn::CpyConst(i, parse_reg(words[2])),
                        Opnd::Reg(n) => Insn::CpyReg(n, parse_reg(words[2]))
                    }
                },
                "inc" => {
                    assert!(words.len() == 2);
                    Insn::Add(1, parse_reg(words[1]))
                },
                "dec" => {
                    assert!(words.len() == 2);
                    Insn::Add(-1, parse_reg(words[1]))
                },
                "jnz" => {
                    assert!(words.len() == 3);
                    let off = parse_const(words[2]);
                    match parse_opnd(words[1]) {
                        Opnd::Reg(reg) => Insn::JNZ(reg, off),
                        Opnd::Const(cnst) => match cnst {
                            0 => Insn::Nop,
                            _ => Insn::Jump(off)
                        }
                    }
                },
                _ => panic!(format!("unrecognized insn {}", words[0]))
            };
        insns.push(insn);
    };
    let mut state = State{pc: 0, regs: vec![0, 0, 0, 0]};
    while state.pc < insns.len() {
        let pc = state.pc;
        step(&mut state, &insns[pc]);
    };
    print!("{}\n", state.regs[0]);
}
