// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! Advent of Code Day 7.

use std::collections::HashSet;

extern crate aoc;

/// Return true iff given segment has an ABBA.
fn segment_abba(segment: &str) -> bool {
    let c: Vec<char> = segment.chars().collect();
    for i in 0usize .. c.len() - 3 {
        if c[i] != c[i + 1] && c[i + 1] == c[i + 2] && c[i] == c[i + 3] {
            return true;
        }
    }
    false
}

/// Return true iff the given string supports TLS.
fn supports_tls(line: String) -> bool {
    // Set up state. has_abba is true iff an
    // ABBA has been seen outside brackets.
    let mut has_abba = false;
    let mut in_brackets = false;
    let mut segment = String::new();
    // Walk the line in segments between brackets.
    for c in line.chars() {
        if c == '[' {
            // Ending a supernet segment.
            assert!(!in_brackets);
            if segment_abba(&segment) {
                has_abba = true;
            };
            segment = String::new();
            in_brackets = true;
        } else if c == ']' {
            // Ending a hypernet segment.
            assert!(in_brackets);
            if segment_abba(&segment) {
                // Can return early here.
                return false;
            };
            segment = String::new();
            in_brackets = false;
        } else {
            segment.push(c);
        }
    }
    // Clean up and return correct answer.
    assert!(!in_brackets);
    has_abba || segment_abba(&segment)
}

/// Return the ABAs contained in the given segment, as a set
/// of two-character strings. If flip is true, the strings will
/// be in reverse order.
fn segment_abas(segment: &str, flip: bool, abas: &mut HashSet<String>) {
    let c: Vec<char> = segment.chars().collect();
    for i in 0usize .. c.len() - 2 {
        if c[i] != c[i + 1] && c[i] == c[i + 2] {
            let mut aba = String::new();
            if flip {
                aba.push(c[i+1]);
                aba.push(c[i]);
            } else {
                aba.push(c[i]);
                aba.push(c[i+1]);
            };
            abas.insert(aba);
        }
    }
}

/// Return true if the given string supports SSL.
/// Strategy: Keep two sets, one of supernet ABAs
/// and one of flipped hypernet ABAs. Intersect
/// at the end. Intersect-as-we-go would allow
/// short-circuit on success.
fn supports_ssl(line: String) -> bool {
    // Set up state.
    let mut supernet_abas = HashSet::new();
    let mut hypernet_abas = HashSet::new();
    let mut in_brackets = false;
    let mut segment = String::new();

    // Same cases as TLS.
    for c in line.chars() {
        if c == '[' {
            assert!(!in_brackets);
            segment_abas(&segment, false, &mut supernet_abas);
            segment = String::new();
            in_brackets = true;
        } else if c == ']' {
            assert!(in_brackets);
            segment_abas(&segment, true, &mut hypernet_abas);
            segment = String::new();
            in_brackets = false;
        } else {
            segment.push(c);
        }
    };

    // Clean up and return result.
    assert!(!in_brackets);
    segment_abas(&segment, false, &mut supernet_abas);
    !supernet_abas.is_disjoint(&hypernet_abas)
}

/// Count and print the number of lines on stdin for which
/// the given test function returns true.
pub fn main() {
    let part = aoc::get_part();
    let tester: fn(String) -> bool = if part == 1 {
        supports_tls
    } else {
        supports_ssl
    };
    // Set up state.
    let mut count = 0;
    // Read strings from the input file and process them.
    for l in aoc::input_lines() {
        if tester(l) {
            count += 1;
        }
    }
    println!("{}", count);
}
