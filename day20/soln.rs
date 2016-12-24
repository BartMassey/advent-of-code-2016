// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

// Advent of Code Day 20.

use std::cmp::*;

extern crate aoc;

fn read_ranges() -> Vec<(u64, u64)> {
    let mut result = Vec::new();
    for target in aoc::input_lines() {
        let desc = target.split('-').collect::<Vec<&str>>();
        let start = desc[0].parse().expect("could not parse start");
        let end = desc[1].parse().expect("could not parse end");
        result.push((start, end));
    };
    result
}

fn merge_ranges(ranges: &mut Vec<(u64, u64)>) {
    let mut result = Vec::new();
    ranges.sort();
    let mut cur_range = None;
    for &(left, right) in ranges.iter() {
        match cur_range {
            None => {
                cur_range = Some((left, right));
            },
            Some((cur_left, cur_right)) => {
                if left > cur_right + 1 {
                    result.push((cur_left, cur_right));
                    cur_range = Some((left, right));
                } else {
                    cur_range = Some((cur_left, max(cur_right, right)));
                }
            }
        }
    };
    match cur_range {
        None => (),
        Some(r) => result.push(r)
    };
    *ranges = result;
}

#[test]
fn test_merge_ranges() {
    let mut ranges = vec![
        (11, 13),
        (0, 2),
        (3, 5),
        (7, 8),
        (6, 9),
        (15, 15) ];
    merge_ranges(&mut ranges);
    if ranges != vec![(0,9), (11, 13), (15, 15)] {
        print!("{:?}\n", ranges);
        panic!("ranges mismatch");
    }
}

pub fn main() {
    let part = aoc::get_part();
    let mut ranges = read_ranges();
    merge_ranges(&mut ranges);
    if part == 1 {
        print!("{}\n", ranges[0].1 + 1);
    } else {
        let mut count = 0;
        for i in 0..ranges.len()-1 {
            count += ranges[i+1].0 - ranges[i].1 - 1;
        };
        count += 4294967295 - ranges[ranges.len()-1].1;
        print!("{}\n", count);
    };
}
