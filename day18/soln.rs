// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

// Advent of Code Day 18.

const SHOW: bool = false;

extern crate aoc;

fn ca_step(patv: &[bool]) -> bool {
    assert!(patv.len() == 3);
    let pat = (patv[0], patv[1], patv[2]);
    match pat {
        (false, false, true) => false,
        (true, false, false) => false,
        (false, true, true) => false,
        (true, true, false) => false,
        _ => true
    }
}

fn count_ca(row0: &Vec<bool>, nrows: usize) -> usize {
    let ncells = row0.len();
    let mut count = 0;
    let mut cur_row = (*row0).clone();
    for _ in 0..nrows {
        if SHOW {
            println!("{}", decode_row(&cur_row));
        };
        for b in cur_row.iter() {
            if *b {
                count += 1;
            }
        };
        let mut next_row = Vec::new();
        let left_pat = [true, cur_row[0], cur_row[1]];
        let cell = ca_step(&left_pat);
        next_row.push(cell);
        for posn in 1..ncells-1 {
            let cell = ca_step(&cur_row[posn-1..posn+2]);
            next_row.push(cell);
        };
        let right_pat = [cur_row[ncells - 2], cur_row[ncells - 1], true];
        let cell = ca_step(&right_pat);
        next_row.push(cell);
        cur_row = next_row;
    };
    count
}

fn decode_row(row: &[bool]) -> String {
    let mut result = String::new();
    for b in row.iter() {
        match b {
            &true => result.push('.'),
            &false => result.push('^')
        }
    };
    result
}

fn encode_row(row: &str) -> Vec<bool> {
    let mut result = Vec::new();
    for c in row.chars() {
        let e = match c {
            '^' => false,
            '.' => true,
            _ => { panic!("unexpected char in row"); }
        };
        result.push(e);
    };
    result
}

pub fn main() {
    let (_, args) = aoc::get_part_args();
    assert!(args.len() == 1);
    let nrows = args[0].parse().expect("could not parse nrows");
    let mut lines = aoc::input_lines();
    let row0 = lines.next().expect("could not read row");
    match lines.next() {
        Some(_) => { panic!("more than one row"); },
        None => ()
    };
    println!("{}", count_ca(&encode_row(&row0), nrows));
}
