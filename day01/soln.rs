// Copyright © 2016 Bart Massey
// Advent of Code Day 1

use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

fn traverse(stop_short: bool) -> (i32, i32) {
    let directions = [(0, 1), (-1, 0), (0, -1), (1, 0)];
    let mut stdin = io::stdin();
    let mut s = String::new();
    let _ = stdin.read_to_string(&mut s).unwrap();
    let mut position = (0, 0);
    let mut trail = HashSet::new();
    trail.insert(position);
    let mut facing = 0;
    for t in s.trim().split(", ") {
        let turn = t.chars().nth(0).unwrap();
        let step: i32 = (&t[1..]).parse().unwrap();
        facing = match turn {
            'L' => (facing + 1) % 4,
            'R' => (facing + 3) % 4,
            _ => panic!("bad direction")
        };
        for _ in 0..step {
            position.0 += directions[facing].0;
            position.1 += directions[facing].1;
            if stop_short && trail.contains(&position) {
                return position;
            }
            trail.insert(position);
        }
    };
    position
}

pub fn show_traverse(stop_short: bool) {
    let position = traverse(stop_short);
    print!("{}\n", position.0.abs() + position.1.abs())
}
