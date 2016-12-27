// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

// Advent of Code Day 13.

use std::collections::{BTreeSet, BinaryHeap};
use std::cmp::*;

extern crate aoc;

fn is_wall(k: usize, (x, y): (usize, usize)) -> bool {
    let h = x*x + 3*x + 2*x*y + y + y*y + k;
    let hc = aoc::popcount(h);
    return hc & 1 == 1;
}

#[derive(Clone, Debug)]
struct PQElem {
    cost: usize,
    fcost: usize,
    state: (usize, usize)
}

impl PartialEq for PQElem {
    fn eq(&self, other: &PQElem) -> bool {
        other.fcost == self.fcost && other.cost == self.cost
    }
}

impl Eq for PQElem {}

impl PartialOrd for PQElem {
    fn partial_cmp(&self, other: &PQElem) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for PQElem {
    fn cmp(&self, other: &PQElem) -> Ordering {
        match other.fcost.cmp(&self.fcost) {
            Ordering::Equal =>
                self.cost.cmp(&other.cost),
            c => c
        }
    }
}

pub fn main() {
    let (part, args) = aoc::get_part_args();
    let k = args[0].parse::<usize>().expect("k not a number");
    let (goal, max_g) =
        match part {
            1 => {
                assert!(args.len() == 3);
                let goal_x = args[1].parse::<usize>().expect("x not a number");
                let goal_y = args[2].parse::<usize>().expect("y not a number");
                ((goal_x, goal_y), 0)
            },
            2 => {
                assert!(args.len() == 2);
                let max_g = args[1].parse::<usize>().expect("max_g not a number");
                ((max_g + 1, max_g + 1), max_g)
            },
            _ => {
                panic!("unknown part");
            }
        };
    let (goal_x, goal_y) = goal;
    let dirns = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let start = PQElem{cost: 0, fcost: 0, state: (1, 1)};
    let mut stop_list = BTreeSet::new();
    let mut pq = BinaryHeap::new();
    pq.push(start);
    loop {
        match pq.pop() {
            Some(PQElem{cost: g, fcost: _, state}) => {
                if state == goal {
                    println!("cost {} for {:?}", g, state);
                    return;
                };
                match stop_list.insert(state) {
                    false => { continue; },
                    true => {
                        if part == 2 && g >= max_g {
                            continue;
                        }
                        let (x, y) = state;
                        for &(dx, dy) in dirns.iter() {
                            let next_x = x as isize + dx;
                            let next_y = y as isize + dy;
                            if next_x < 0 || next_y < 0 {
                                continue;
                            }
                            let next_state = (next_x as usize, next_y as usize);
                            if is_wall(k, next_state) {
                                continue;
                            }
                            if !stop_list.contains(&next_state) {
                                let h = (goal_x as isize - next_x).abs() + (goal_y as isize - next_y).abs();
                                let g = g + 1;
                                pq.push(PQElem{fcost: g + h as usize, cost: g, state: next_state});
                            }
                        }
                    }
                };
            },
            None => {
                if part == 2 {
                    println!("{}", stop_list.len());
                    return;
                };
                panic!("found no solution");
            }
        }
    };
}
