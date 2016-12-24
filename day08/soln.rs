// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

// Advent of Code Day 8 Part 1

// Turn on for display tracing.
const TRACING: bool = false;

extern crate aoc;
#[macro_use] extern crate lazy_static;
extern crate regex;

// Textual patterns for instructions.
lazy_static! {
    static ref RECT_PATTERN: regex::Regex =
        regex::Regex::new(r"^rect (\d+)x(\d+)$")
        .expect("could not compile rect pattern");
    static ref ROTATE_ROW_PATTERN: regex::Regex =
        regex::Regex::new(r"^rotate row y=(\d+) by (\d+)$")
        .expect("could not compile row pattern");
    static ref ROTATE_COLUMN_PATTERN: regex::Regex =
        regex::Regex::new(r"^rotate column x=(\d+) by (\d+)$")
        .expect("could not compile column pattern");
}

// Instruction to draw a rectangle. Returns true iff it was
// chosen.
fn insn_rect(insn: &str, m: &mut Vec<Vec<char>>) -> bool {
    match (*self::RECT_PATTERN).captures(insn) {
        None => return false,
        Some(parts) => {
            let x: usize = parts.at(1)
                .expect("insn_rect: could not find x")
                .parse().expect("insn_rect: could not parse x");
            let y: usize = parts.at(2)
                .expect("insn_rect: could not find y")
                .parse().expect("insn_rect: could not parse y");
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
            let y: usize = parts.at(1)
                .expect("insn_rotate_row: could not find y")
                .parse().expect("insn_rotate_row: could not parse y");
            let n: usize = parts.at(2)
                .expect("insn_rotate_row: could not find n")
                .parse().expect("insn_rotate_row: could not parse n");
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
            let x: usize = parts.at(1)
                .expect("insn_rotate_column: could not find x")
                .parse().expect("insn_rotate_column: could not parse x");
            let n: usize = parts.at(2)
                .expect("insn_rotate_row: could not find n")
                .parse().expect("insn_rotate_row: could not parse n");
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

// Run the instructions and print the number of on pixels or
// the pixels themselves at the end.
fn main() {
    let (part, dims) = aoc::get_part_args();
    assert!(dims.len() == 2);
    let x_size: usize = dims[0].parse().expect("main: could not parse x_size");
    let y_size: usize = dims[1].parse().expect("main: could not parse y_size");
    // Set up state.
    let mut m = Vec::new();
    for _ in 0..x_size {
        let mut v = Vec::with_capacity(y_size);
        v.resize(y_size, '.');
        m.push(v);
    }
    let insns: &[fn(&str, &mut Vec<Vec<char>>) -> bool] =
        &[insn_rect, insn_rotate_column, insn_rotate_row];
    // Read strings from the input file and process them.
    for l in aoc::input_lines() {
        // Search through the instructions until finding one
        // that works.
        let mut processed = false;
        for f in insns.iter() {
            if f(&l, &mut m) {
                processed = true;
                if TRACING {
                    print!("\n");
                    display(&m);
                };
                break;
            }
        };
        if !processed {
            panic!("undentified instruction");
        }
    };
    // Count up the on pixels.
    let mut count = 0;
    for x in 0..x_size {
        for y in 0..y_size {
            if m[x][y] == '#' {
                count += 1;
            }
        }
    };
    if TRACING {
        print!("\n");
    };
    // Show final answer.
    if part == 1 {
        print!("{}\n", count);
    } else {
        display(&m);
    };
}
