// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

// Advent of Code Day 9

use std::io;
use std::io::prelude::*;
use std::str;

// Walk a string of bytes, computing the length of its
// expansion. If recurse is true, include the length of
// recursive subexpansions as new patterns are encountered.
fn parse_expansion(pat: &[u8], recurse: bool) -> usize {
    let nchars = pat.len();
    let mut nemit = 0;
    let mut i = 0;
    while i < nchars {
        match (*pat)[i] as char {
            '(' => {
                // Pattern start found. First find the end.
                let mut end = i;
                while (*pat)[end] as char != ')' {
                    end += 1;
                }
                // Grab the "coords" out of the pattern.
                let target: &str = str::from_utf8(&(*pat)[i+1..end]).unwrap();
                let coords = target.split('x').collect::<Vec<&str>>();
                assert!(coords.len() == 2);
                let replen: usize = coords[0].parse().unwrap();
                let repcount: usize = coords[1].parse().unwrap();
                // Advance over the pattern.
                i = end + 1;
                // Process the target text.
                match recurse {
                    false => {
                        nemit += replen * repcount;
                    },
                    true => {
                        let subemit =
                            parse_expansion(&(*pat)[i..i + replen], recurse);
                        nemit += subemit * repcount;
                    }
                };
                // Advance over the target text.
                i += replen;
            },
            '\n' => {
                // A newline at the end is harmless.
                assert!(i == nchars - 1);
                break;
            },
            _ => {
                // Just emit ordinary characters.
                i += 1;
                nemit += 1;
            }
        }
    };
    // Report the amount of emitted text.
    nemit
}

// Expand the top-level. XXX We first read the whole file
// into memory, because it's easier, and because Rust has no
// way to do better, really.  XXX If Unicode were an issue,
// we should really transform the vector of bytes into a
// vector of chars before passing it.
pub fn soln(recurse: bool) {
    let mut chars = Vec::new();
    let _ = io::stdin().read_to_end(&mut chars).unwrap();
    let nemit = parse_expansion(&chars, recurse);
    print!("{}\n", nemit);
}
