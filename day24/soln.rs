// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! Advent of Code Day 24.

use std::collections::{HashSet, BTreeSet, HashMap};

extern crate aoc;

/// Map to be explored.
struct Map {
    open: HashSet<aoc::Point>,
    goals: HashMap<aoc::Point, u32>,
    grid_box: aoc::GridBox,
    part: usize,
    start_posn: aoc::Point
}

/// Problem state.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    /// Current position.
    posn: aoc::Point,
    /// Set of previously-visited positions.
    visited: BTreeSet<u32>
}

impl aoc::SearchState for State {
    type Label = ();
    type Global = Map;

    fn label(&self) -> () {
        ()
    }

    fn neighbors(&self, global: &Self::Global) -> Vec<(usize, Box<Self>)> {
        let mut result = Vec::new();
        for next_posn in global.grid_box.neighbors(self.posn) {
            // Can't go through walls.
            if !global.open.contains(&next_posn) {
                continue;
            };

            // Add to the visited list if needed.
            let mut next_visited = self.visited.clone();
            match global.goals.get(&next_posn) {
                None => (),
                Some(d) => {
                    next_visited.insert(*d);
                }
            };

            // Push the neighbor state.
            let next_state = State{ posn: next_posn,
                                    visited: next_visited };
            result.push((1, Box::new(next_state)))
        };
        result
    }

    /// Stop when everything has been visited,
    /// but in part 2 only when we then get home.
    fn is_goal(&self, global: &Map) -> bool {
        self.visited.len() == global.goals.len() &&
        (global.part == 1 || self.posn == global.start_posn)
    }
}

/// Run the search, optionally returning the robot to its
/// starting point.
pub fn main() {
    let part = aoc::get_part();

    // Set up state.
    let mut open: HashSet<aoc::Point> = HashSet::new();
    let mut goals: HashMap<aoc::Point, u32> = HashMap::new();
    let mut goal_labels: BTreeSet<u32> = BTreeSet::new();
    let mut maybe_start: Option<aoc::Point> = None;
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
    let start_posn = match maybe_start {
        Some(p) => p,
        None => panic!("no start position found")
    };
    let map = Map {
        open: open,
        goals: goals,
        grid_box: aoc::GridBox::new_grid(),
        part: part,
        start_posn: start_posn
    };
    let start_state = State {
        posn: start_posn,
        visited: BTreeSet::new()
    };
    match aoc::a_star(&map, &start_state, false) {
        Some((g, _)) => println!("{}", g),
        None => panic!("no solution")
    };
}
