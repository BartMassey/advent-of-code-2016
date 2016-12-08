// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

// Advent of Code Day 8 Part 1

// Size of display.
const X_SIZE: usize = 50;
const Y_SIZE: usize = 6;

use std::io;
use std::io::prelude::*;

#[macro_use] extern crate lazy_static;
extern crate regex;

// Textual patterns for instructions.
lazy_static! {
    static ref RECT_PATTERN: regex::Regex =
        regex::Regex::new(r"^rect (\d+)x(\d+)$").unwrap();
    static ref ROTATE_ROW_PATTERN: regex::Regex =
        regex::Regex::new(r"^rotate row y=(\d+) by (\d+)$").unwrap();
    static ref ROTATE_COLUMN_PATTERN: regex::Regex =
        regex::Regex::new(r"^rotate column x=(\d+) by (\d+)$").unwrap();
}

// Instruction to draw a rectangle. Returns true iff it was
// chosen.
fn insn_rect(insn: &str, m: &mut Vec<Vec<char>>) -> bool {
    match (*self::RECT_PATTERN).captures(insn) {
        None => return false,
        Some(parts) => {
            let x: usize = parts.at(1).unwrap().parse().unwrap();
            let y: usize = parts.at(2).unwrap().parse().unwrap();
            for i in 0..x {
                for j in 0..y {
                    m[i][j] = '#';
                }
            }
        }
    };
    true
}

// Instruction to rotate a row right. Returns true iff it
// was chosen.
fn insn_rotate_row(insn: &str, m: &mut Vec<Vec<char>>) -> bool {
    match (*self::ROTATE_ROW_PATTERN).captures(insn) {
        None => return false,
        Some(parts) => {
            let y: usize = parts.at(1).unwrap().parse().unwrap();
            let n: usize = parts.at(2).unwrap().parse().unwrap();
            let d = m.len();
            for _ in 0..n {
                let tmp = m[d-1][y];
                for x in (1..d).rev() {
                    m[x][y] = m[x-1][y];
                }
                m[0][y] = tmp;
            }
        }
    };
    true
}

// Instruction to rotate a column down. Returns true iff it
// was chosen.
fn insn_rotate_column(insn: &str, m: &mut Vec<Vec<char>>) -> bool {
    match (*self::ROTATE_COLUMN_PATTERN).captures(insn) {
        None => return false,
        Some(parts) => {
            let x: usize = parts.at(1).unwrap().parse().unwrap();
            let n: usize = parts.at(2).unwrap().parse().unwrap();
            let d = m[0].len();
            for _ in 0..n {
                let tmp = m[x][d-1];
                for y in (1..d).rev() {
                    m[x][y] = m[x][y-1];
                }
                m[x][0] = tmp;
            }
        }
    };
    true
}

// Display the given screen.
fn display(m: &Vec<Vec<char>>) {
    for y in 0..m[0].len() {
        for x in 0..m.len() {
            print!("{}", m[x][y]);
        };
        print!("\n");
    }
}

// Run the instructions, displaying as we go, and
// then print the number of on pixels at the end.
fn main() {
    // Set up state.
    let mut m = vec![vec!['.';Y_SIZE];X_SIZE];
    let insns: &[fn(&str, &mut Vec<Vec<char>>) -> bool] =
        &[insn_rect, insn_rotate_column, insn_rotate_row];
    // Read strings from the input file and process them.
    let stdin = io::stdin();
    let reader = io::BufReader::new(stdin);
    for line in reader.lines() {
        let l = line.unwrap();
        // Search through the instructions until finding one
        // that works.
        let mut processed = false;
        for f in insns.iter() {
            if f(&l, &mut m) {
                processed = true;
                print!("\n");
                display(&m);
                break;
            }
        };
        if !processed {
            panic!("undentified instruction");
        }
    };
    // Count up the on pixels.
    let mut count = 0;
    for x in 0..m.len() {
        for y in 0..m[0].len() {
            if m[x][y] == '#' {
                count += 1;
            }
        }
    };
    // Show final answer.
    print!("\n");
    display(&m);
    print!("{}\n", count);
}
