// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! Hexadecimal utilities for Advent of Code 2016 solutions.

/// Given an input between 0 and 15, return the
/// corresponding hex digit. This is substantially
/// faster than `std::char::from_digit()`.
pub fn hex_digit(n: u8) -> char {
    let d =
        if n <= 9 {
            n as u8 + '0' as u8
        } else {
            n as u8 - 10 + 'a' as u8
        };
    d as char
}

/// Convert a byte vector to a hexadecimal string.
pub fn hex_string(bytes: &[u8]) -> String {
    let mut s = String::new();
    for i in 0..bytes.len() {
        s.push(hex_digit(bytes[i] >> 4));
        s.push(hex_digit(bytes[i] & 0xf));
    };
    s
}
