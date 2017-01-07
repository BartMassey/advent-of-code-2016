// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! Advent of Code Day 4.

extern crate regex;
extern crate aoc;

/// Return the "checksum" of the given room name.
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
        counts[c as usize - b'a' as usize] += 1;
    };
    // XXX This is a pretty gross algorithm whose
    // only real merit is avoiding some sorting hassle.
    let mut m: u32 = *counts.iter().max()
        .expect("name_checksum: could not find max count");
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
        sum[nsum] = (found_i as u8 + b'a') as char;
        counts[found_i] = 0;
        nsum += 1;
    };
    sum.iter().cloned().collect::<String>()
}

/// Return the decryption of the given name.
fn name_decrypt(room_name: &str, sector_id: u32) -> String {
    let mut result = Vec::<char>::new();
    for c in room_name.chars() {
        if c == '-' {
            result.push(' ');
        } else {
            let shift = c as u32 - b'a' as u32 + sector_id;
            result.push(((shift % 26) as u8 + b'a') as char);
        };
    };
    result.iter().cloned().collect::<String>()
}

/// Process the input and solve the specified problem.
pub fn main() {
    let part = aoc::get_part();
    assert!(part == 1 || part == 2);
    let decrypt = part == 2;

    // Set up the regex for room encryption.
    let room_pattern = regex::Regex::new(r"^(.*)-(\d+)\[(.*)\]$")
        .expect("main: could not compile regex");

    // Set up state.
    let mut sector_sum: u32 = 0;

    // Read strings from the input file and process them.
    for l in aoc::input_lines() {
        let parts = room_pattern.captures(&l)
            .expect("main: could not match line");
        let room_name = parts.at(1)
            .expect("main: could not find room name");
        let computed_sum = name_checksum(room_name);
        let given_sum = parts.at(3).expect("main: could not find checksum");
        if computed_sum != given_sum {
            continue;
        }
        let sector_id = parts.at(2).expect("main: could not find sector id")
            .parse().expect("main: could not parse sector id");

        // Handle part 2 by printing all the decryptions.
        if decrypt {
            println!("{} {}", name_decrypt(&room_name, sector_id), sector_id);
        }
        sector_sum += sector_id;
    };

    // Handle part 1 by printing the sum.
    if !decrypt {
        println!("{}", sector_sum);
    };
}
