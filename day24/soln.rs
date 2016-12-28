// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! Advent of Code Day 24.

use std::cmp::*;
use std::collections::{HashSet, BTreeSet, BinaryHeap, HashMap};

extern crate aoc;

/// Type of 2D points.
type Point = (usize, usize);

/// Problem state.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    /// Current position.
    posn: Point,
    /// Set of previously-visited positions.
    visited: BTreeSet<u32>
}

/// A\* / Dijkstra node.
#[derive(Clone, Debug)]
struct PQElem {
    /// Cost to get to this state.
    cost: usize,
    /// Problem state.
    state: State
}

impl PartialEq for PQElem {
    fn eq(&self, other: &PQElem) -> bool {
        other.cost == self.cost
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
        other.cost.cmp(&self.cost)
    }
}

/// Run the search, optionally returning the robot to its
/// starting point.
pub fn main() {
    let part = aoc::get_part();

    // Set up state.
    let mut open: HashSet<Point> = HashSet::new();
    let mut goals: HashMap<Point, u32> = HashMap::new();
    let mut goal_labels: BTreeSet<u32> = BTreeSet::new();
    let mut maybe_start: Option<Point> = None;
    let mut x = 0;
    let mut y = 0;
    // Process the map.
    for row in aoc::input_lines() {
        for c in row.chars() {
            match c {
                '.' => { open.insert((x, y)); },
                '#' => (),
                dc if dc.is_digit(10) => {
                    match dc.to_digit(10).unwrap() {
                        0 => {
                            // Location 0 is special.
                            maybe_start = Some((x, y));
                            open.insert((x, y));
                        },
                        d => {
                            goals.insert((x, y), d);
                            goal_labels.insert(d);
                            open.insert((x, y));
                        }
                    }
                },
                _ => {
                    panic!("bad char in input");
                }
            };
            x += 1;
        }
        x = 0;
        y += 1;
    };
    let goals_len = goals.len();
    let start_posn = match maybe_start {
        Some(p) => p,
        None => panic!("no start position found")
    };
    let dirns = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    // Run the Dijkstra (A\*) search.
    let start = State{ posn: start_posn, visited: BTreeSet::new() };
    let mut stop_list: HashSet<State> = HashSet::new();
    let mut pq: BinaryHeap<PQElem> = BinaryHeap::new();
    pq.push(PQElem{ state: start, cost: 0 } );
    loop {
        match pq.pop() {
            Some(PQElem{cost: g, state}) => {
                // Stop when everything has been visited,
                // but in part 2 only when we then get home.
                if state.visited.len() == goals_len {
                    if part == 1 || state.posn == start_posn {
                        println!("{}", g);
                        return;
                    }
                };

                // Process all the neighbors.
                match stop_list.insert(state.clone()) {
                    false => { continue; },
                    true => {
                        // Search each neighbor in turn.
                        let (x, y) = state.posn;
                        for &(dx, dy) in dirns.iter() {
                            // Find the neighbor location.
                            let next_x = x as isize + dx;
                            let next_y = y as isize + dy;
                            let next_posn = (next_x as usize,
                                              next_y as usize);

                            // Can't go through walls.
                            if !open.contains(&next_posn) {
                                continue;
                            };

                            // Add to the visited list if needed.
                            let mut next_visited = state.visited.clone();
                            match goals.get(&next_posn) {
                                None => (),
                                Some(d) => {
                                    next_visited.insert(*d);
                                }
                            };

                            // Push the neighbor state.
                            let next_state = State{ posn: next_posn,
                                                    visited: next_visited };
                            pq.push( PQElem{ state: next_state,
                                             cost: g + 1 } );
                        }
                    }
                }
            },
            None => {
                panic!("no solution");
            }
        }
    }
}
