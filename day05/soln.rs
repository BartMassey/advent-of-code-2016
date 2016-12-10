// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

// Advent of Code Day 5

// Portions borrowed from
// https://gist.github.com/gkbrk/2e4835e3a17b3fb6e1e7

// The hardcoded room code.
static ROOM_CODE: &'static str = "wtnhxymk";

extern crate aoc;
extern crate crypto;

use self::crypto::digest::Digest;
use std::io::Write;
use std::io::stdout;

// Given an input between 0 and 15, return the
// corresponding hex digit.
fn hex_digit(n: usize) -> char {
    let d =
        if n <= 9 {
            n as u8 + '0' as u8
        } else {
            n as u8 - 10 + 'a' as u8
        };
    d as char
}
    
// Pile up the password byte array into a string and return
// to the start of the line and show it.
pub fn cinema_string(marquee: &[char]) {
    let marquee_string = marquee.iter().cloned().collect::<String>();
    print!("\r{}", marquee_string);
    stdout().flush().unwrap();
}


// Find the password matching the room code, using the
// algorithm of part 2 of the problem if "positional" is
// true.
pub fn main() {
    let (part1, _) = aoc::parseargs();
    let positional = !part1;
    // Set up the password storage.
    let mut password = ['.'; 8];
    cinema_string(&password);
    // Set up the rest of the state.
    let mut hasher = crypto::md5::Md5::new();
    let prefix = ROOM_CODE.as_bytes();
    let mut count = 0;
    // This loop should never finish.
    for i in 0..std::u64::MAX {
        // Get the current hash.
        hasher.reset();
        hasher.input(prefix);
        hasher.input(i.to_string().as_bytes());
        let mut output = [0; 16];
        hasher.result(&mut output);
        // If the first five hex digits are not zero, it's uninteresting.
        if output[0] != 0 || output[1] != 0 || (output[2] >> 4) != 0 {
            continue;
        }
        // Check for digit six in bounds.
        let digit_six = (output[2] & 0xf) as usize;
        if positional && digit_six >= password.len() {
            continue;
        }
        // Compute the current password position and digit.
        let (posn, digit) =
            if positional {
                (digit_six, (output[3] >> 4) as usize)
            } else {
                (count, digit_six)
            };
        // If the position is already filled, leave it.
        if password[posn] != '.' {
            continue;
        }
        // Update and display the password.
        password[posn] = hex_digit(digit);
        cinema_string(&password);
        // If we have all the characters, it's over.
        count += 1;
        if count >= password.len() {
            print!("\n");
            return;
        }
    };
    panic!("ran out of hashes");
}
