// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! Advent of Code Day 13.

use std::collections::{BTreeSet, BinaryHeap};
use std::cmp::*;

extern crate aoc;

/// Return true if there is a wall at the given location.
fn is_wall(k: usize, (x, y): (usize, usize)) -> bool {
    let h = x*x + 3*x + 2*x*y + y + y*y + k;
    let hc = aoc::popcount(h as u64);
    return hc & 1 == 1;
}

/// Node for A\* search.
#[derive(Clone, Debug)]
struct PQElem {
    /// Actual cost.
    cost: usize,
    /// Heuristic cost.
    fcost: usize,
    /// Underlying state.
    state: (usize, usize)
}

impl PartialEq for PQElem {
    /// Two nodes are equal if their costs are equal.
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
    /// Order by least heuristic cost, breaking
    /// ties by larger actual cost.
    fn cmp(&self, other: &PQElem) -> Ordering {
        match other.fcost.cmp(&self.fcost) {
            Ordering::Equal =>
                self.cost.cmp(&other.cost),
            c => c
        }
    }
}

/// Read the problem description and run the search.
pub fn main() {
    let (part, args) = aoc::get_part_args();
    let k = args[0].parse::<usize>().expect("k not a number");

    // Get the search set up.
    let (goal, max_g) =
        match part {
            1 => {
                assert!(args.len() == 3);
                let goal_x =
                    args[1].parse::<usize>().expect("x not a number");
                let goal_y =
                    args[2].parse::<usize>().expect("y not a number");
                ((goal_x, goal_y), 0)
            },
            2 => {
                assert!(args.len() == 2);
                let max_g =
                    args[1].parse::<usize>().expect("max_g not a number");
                // We cheat a bit here by making the goal large
                // and thus hopefully unreachably far away.
                ((max_g + 1, max_g + 1), max_g)
            },
            _ => {
                panic!("unknown part");
            }
        };

    // Set up problem state.
    let grid = aoc::GridBox::new_grid();
    let start = PQElem{cost: 0, fcost: 0, state: (1, 1)};
    let mut stop_list = BTreeSet::new();
    let mut pq = BinaryHeap::new();

    // Run the A* search.
    pq.push(start);
    loop {
        match pq.pop() {
            Some(PQElem{cost: g, fcost: _, state}) => {
                // We should only ever reach the goal in part 1.
                if state == goal {
                    assert!(part == 1);
                    println!("{}", g);
                    return;
                };
                match stop_list.insert(state) {
                    false => { continue; },
                    true => {
                        if part == 2 && g >= max_g {
                            continue;
                        }
                        // Process neighbor in each direction.
                        for next_state in grid.neighbors(state) {
                            // Clip on walls.
                            if is_wall(k, next_state) {
                                continue;
                            }

                            // Push the new neighbor.
                            if !stop_list.contains(&next_state) {
                                let h = aoc::manhattan_distance(
                                        next_state, goal);
                                let g = g + 1;
                                pq.push(PQElem{
                                    fcost: g + h as usize,
                                    cost: g,
                                    state: next_state
                                });
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
