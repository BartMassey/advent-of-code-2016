// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

// Advent of Code Day 4

use std::io;
use std::io::prelude::*;

extern crate regex;
extern crate aoc;

fn name_checksum(room_name: &str) -> String {
    // Set up state.
    let mut sum = ['.'; 5];
    let mut nsum = 0;
    let mut counts = [0u32; 26];
    let ncounts = counts.len();
    // Walk over the characters counting them.
    for c in room_name.chars() {
        if c == '-' {
            continue;
        }
        counts[c as usize - 'a' as usize] += 1;
    };
    // XXX This is a pretty gross algorithm whose
    // only real merit is avoiding some sorting hassle.
    let mut m: u32 = *counts.iter().max().unwrap();
    if m == 0 {
        panic!("maximum is zero");
    }
    while nsum < 5 {
        let mut found_i = ncounts;
        for i in 0..ncounts {
            if counts[i] == m {
                found_i = i;
                break;
            }
        }
        if found_i == ncounts {
            if m > 1 {
                m -= 1;
                continue;
            }
            panic!("not enough characters for checksum");
        }
        sum[nsum] = (found_i as u8 + 'a' as u8) as char;
        counts[found_i] = 0;
        nsum += 1;
    };
    sum.iter().cloned().collect::<String>()
}

fn name_decrypt(room_name: &str, sector_id: u32) -> String {
    let mut result = Vec::<char>::new();
    for c in room_name.chars() {
        if c == '-' {
            result.push(' ');
        } else {
            let shift = c as u32 - 'a' as u32 + sector_id;
            result.push(((shift % 26) as u8 + 'a' as u8) as char);
        };
    };
    result.iter().cloned().collect::<String>()
}

pub fn main() {
    let (part1, _) = aoc::parseargs();
    let decrypt = !part1;
    // Set up the regex for room encryption.
    let room_pattern = regex::Regex::new(r"^(.*)-(\d+)\[(.*)\]$").unwrap();
    // Set up state.
    let mut sector_sum: u32 = 0;
    // Read strings from the input file and process them.
    let stdin = io::stdin();
    let reader = io::BufReader::new(stdin);
    for line in reader.lines() {
        let l = line.unwrap();
        let parts = room_pattern.captures(&l).unwrap();
        let room_name = parts.at(1).unwrap();
        let computed_sum = name_checksum(&room_name);
        let given_sum = parts.at(3).unwrap();
        if computed_sum != given_sum {
            continue;
        }
        let sector_id = parts.at(2).unwrap().parse().unwrap();
        if decrypt {
            print!("{} {}\n", name_decrypt(&room_name, sector_id), sector_id);
            continue;
        }
        sector_sum += sector_id;
    }
    if !decrypt {
        print!("{}\n", sector_sum);
    }
}
