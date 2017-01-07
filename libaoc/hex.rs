// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! Hexadecimal utilities for Advent of Code 2016 solutions.
//!
//! These are not strictly necessary, but provide some
//! convenience.

/// Given an input between 0 and 15, return the
/// corresponding hex digit. This may be faster than
/// `std::char::from_digit()`.
#[inline]
pub fn hex_digit(n: u8) -> char {
    let d =
        if n <= 9 {
            n as u8 + b'0'
        } else {
            n as u8 - 10 + b'a'
        };
    d as char
}

/// Convert a byte vector to a hexadecimal string.
#[inline]
pub fn hex_string(bytes: &[u8]) -> String {
    let mut s = String::new();
    for b in bytes {
        s.push(hex_digit(b >> 4));
        s.push(hex_digit(b & 0xf));
    };
    s
}
