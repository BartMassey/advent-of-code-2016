// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! Advent of Code Day 21.

/// Turn on to trace solutions.
const SHOW: bool = false;

extern crate aoc;
extern crate regex;
extern crate permutohedron;

use permutohedron::Heap;

/// Instructions.
#[derive(Debug)]
enum Insn {
    /// Swap at given indices.
    SwPos(usize, usize),
    /// Swap given characters.
    SwChar(char, char),
    /// Rotate left
    RotL(usize),
    /// Rotate right by given amount.
    RotR(usize),
    /// Rotate based on position of given character.
    RotPos(char),
    /// Reverse the range bounded by the given indices.
    Rev(usize, usize),
    /// Move the character at the given source position
    /// to the given destination position.
    Mov(usize, usize)
}

use self::Insn::*;

/// If the given pattern matches the given target, return a
/// vector of its submatches (parts).
fn try_pat(pat: &regex::Regex, target: &str) -> Option<Vec<String>> {
    match pat.captures(target) {
        Some(parts) =>  {
            let mut v = Vec::new();
            let mut iter = parts.iter();
            iter.next();
            for p in iter {
                v.push(p.expect("part did not match").to_string());
            };
            Some(v)
        },
        None => None
    }
}

/// Return the position of the given character in the given
/// slice. The character must be present to win.
fn find_char(c: char, v: &[char]) -> usize {
    for i in 0..v.len() {
        if v[i] == c {
            return i;
        }
    };
    panic!("find_char(): off end");
}

/// Rotate the given slice to the left *r* times.
fn rotate_left(r: usize, chars: &mut Vec<char>) {
    for _ in 0..r {
        let n = chars.len();
        let tmp = chars[0];
        for i in 1..n {
            chars[i-1] = chars[i];
        };
        chars[n - 1] = tmp;
    }
}

/// Rotate the given slice to the right *r* times.
fn rotate_right(r: usize, chars: &mut Vec<char>) {
    for _ in 0..r {
        let n = chars.len();
        let tmp = chars[n - 1];
        for i in (1..n).rev() {
            chars[i] = chars[i-1];
        };
        chars[0] = tmp;
    }
}

/// Read the input instructions from `stdin`, parse and return them.
fn read_insns() -> Vec<Insn> {
    /// Set up instruction patterns.
    let swpos_pat =
        regex::Regex::new(r"^swap position ([0-9]+) with position ([0-9]+)$")
        .expect("could not compile swpos pattern");
    let swchar_pat =
        regex::Regex::new(r"^swap letter ([a-z]) with letter ([a-z])$")
        .expect("could not compile swchar pattern");
    let rot_pat =
        regex::Regex::new(r"^rotate (left|right) ([0-9+]) steps?$")
        .expect("could not compile rot pattern");
    let rotpos_pat =
        regex::Regex::new(r"^rotate based on position of letter ([a-z])$")
        .expect("could not compile rotpos pattern");
    let rev_pat =
        regex::Regex::new(r"^reverse positions ([0-9]+) through ([0-9]+)$")
        .expect("could not compile rev pattern");
    let mov_pat =
        regex::Regex::new(r"^move position ([0-9]+) to position ([0-9]+)$")
        .expect("could not compile mov pattern");

    // Set up state and process lines.
    let mut insns = Vec::new();
    for target in aoc::input_lines() {
        if SHOW {
            println!("{}", target);
        };

        // Try to find and translate a matching instruction.
        if let Some(args) = try_pat(&swpos_pat, &target) {
            let p1 = args[0].parse::<usize>().unwrap();
            let p2 = args[1].parse::<usize>().unwrap();
            insns.push(SwPos(p1, p2));
            continue;
        };
        if let Some(args) = try_pat(&swchar_pat, &target) {
            let c1 = args[0].chars().nth(0).expect("swchar: op1 error");
            let c2 = args[1].chars().nth(0).expect("swchar: op1 error");
            insns.push(SwChar(c1, c2));
            continue;
        };
        if let Some(args) = try_pat(&rot_pat, &target) {
            let r = args[1].parse::<usize>().unwrap();
            if args[0] == "left" {
                insns.push(RotL(r));
            } else if args[0] == "right" {
                insns.push(RotR(r));
            } else {
                panic!("unknown rotation direction");
            };
            continue;
        };
        if let Some(args) = try_pat(&rotpos_pat, &target) {
            let c = args[0].chars().nth(0).expect("rotpos: op1 error");
            insns.push(RotPos(c));
            continue;
        };
        if let Some(args) = try_pat(&rev_pat, &target) {
            let p1 = args[0].parse::<usize>().unwrap();
            let p2 = args[1].parse::<usize>().unwrap();
            insns.push(Rev(p1, p2));
            continue;
        };
        if let Some(args) = try_pat(&mov_pat, &target) {
            let p1 = args[0].parse::<usize>().unwrap();
            let p2 = args[1].parse::<usize>().unwrap();
            insns.push(Mov(p1, p2));
            continue;
        };

        // Didn't work.
        panic!("could not read instruction");
    };
    insns
}

/// Run the sequence `insns` of instructions on the
/// sequence of characters in `chars0`. On exit,
/// `chars0` will contain the result.
fn run_insns(insns: &[Insn], chars0: &mut Vec<char>) {
    // Make a copy of the input for scratch.
    let mut chars = chars0.clone();

    // Run each instruction.
    for insn in insns {
        if SHOW {
            println!("{}: {:?}",
                   chars.iter().cloned().collect::<String>(),
                   insn);
        };

        // Interpreter dispatch.
        match *insn {
            SwPos(p1, p2) => {
                chars.swap(p1, p2);
            },
            SwChar(c1, c2) =>  {
                let p1 = find_char(c1, &chars);
                let p2 = find_char(c2, &chars);
                chars.swap(p1, p2);
            },
            RotL(r) => {
                rotate_left(r, &mut chars);
            },
            RotR(r) => {
                rotate_right(r, &mut chars);
            },
            RotPos(c) => {
                let mut p = find_char(c, &chars);
                if p >= 4 {
                    p += 1;
                };
                p += 1;
                rotate_right(p, &mut chars);
            },
            Rev(p1, p2) => {
                let tmp = chars.clone();
                for i in p1..p2+1 {
                    chars[i] = tmp[p2 + p1 - i];
                };
            },
            Mov(p1, p2) => {
                if p1 < p2 {
                    let tmp = chars[p1];
                    for i in p1..p2 {
                        chars[i] = chars[i + 1];
                    };
                    chars[p2] = tmp;
                } else if p1 > p2 {
                    let tmp = chars[p1];
                    for i in (p2..p1).rev() {
                        chars[i + 1] = chars[i];
                    };
                    chars[p2] = tmp;
                }
            }
        };
    };

    // Save the result.
    *chars0 = chars;
}

/// Solve the problem.
pub fn main() {
    let (part, args) = aoc::get_part_args();
    assert!(part == 1 || part == 2);

    // Set up state.
    let insns = read_insns();
    let mut chars = args[0].chars().collect::<Vec<char>>();

    if part == 1 {
        // Run the permutation forward.
        run_insns(&insns, &mut chars);
        println!("{}", chars.into_iter().collect::<String>());
    } else {
        // Run every possible permutation forward and see
        // what sticks.
        let mut heap_chars = chars.clone();
        let perms = Heap::new(&mut heap_chars);
        for p in perms {
            let mut pchars = p.clone();
            run_insns(&insns, &mut pchars);
            if chars == pchars {
                println!("{}", p.iter().cloned().collect::<String>());
                return;
            };
        };
        panic!("no password found");
    };
}
