// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! Advent of Code Day 20.

use std::cmp::*;

extern crate aoc;

/// Parse each range and return them as a vector of
/// inclusive start-end tuples. `u32` would probably be
/// sufficient here, but paranoia dictates not worrying
/// about strange end cases.
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

/// Walk over the ranges merging adjacent and overlapping
/// ranges into single ranges. After the call, the ranges
/// are merged and in sorted order.
fn merge_ranges(ranges: &mut Vec<(u64, u64)>) {
    // Set up state. That clone is expensive, but life
    // is hard.
    let mut old_ranges = ranges.clone();
    *ranges = Vec::new();
    old_ranges.sort();
    let mut range_iter = old_ranges.iter();
    let mut cur_range =
        match range_iter.next() {
            None => return,
            Some(range) => *range
        };

    // Try to merge each successive range with the current
    // range.
    for &(left, right) in range_iter  {
        let (cur_left, cur_right) = cur_range;
        if left > cur_right + 1 {
            ranges.push((cur_left, cur_right));
            cur_range = (left, right);
        } else {
            cur_range = (cur_left, max(cur_right, right));
        };
    };
    ranges.push(cur_range);
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
        println!("{:?}", ranges);
        panic!("ranges mismatch");
    }
}

/// Run the problem.
pub fn main() {
    let part = aoc::get_part();

    // Read and merge the ranges.
    let mut ranges = read_ranges();
    merge_ranges(&mut ranges);

    // Show the answer.
    if part == 1 {
        println!("{}", ranges[0].1 + 1);
    } else {
        // Add up the blacklisted addresses.
        let mut count = 0;
        for i in 0..ranges.len() {
            count += ranges[i].1 - ranges[i].0 + 1;
        };

        // The remaining addresses are open. Note that this
        // only works because `u64` since 4294967296 is
        // 2**32.
        println!("{}", 4294967296u64 - count);
    };
}
