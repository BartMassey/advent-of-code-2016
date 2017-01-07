// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! Advent of Code Day 13.

use std::collections::HashSet;

extern crate aoc;

/// Return true if there is a wall at the given location.
fn is_wall(k: usize, (x, y): aoc::Point) -> bool {
    let h = x*x + 3*x + 2*x*y + y + y*y + k;
    let hc = aoc::popcount(h as u64);
    hc & 1 == 1
}

/// Data about the maze itself.
struct Maze {
    grid_box: aoc::GridBox,
    goal: aoc::Point,
    key: usize,
}

/// Location within the maze. Has to be wrapped
/// so that traits can be implemented on it.
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
enum State { Loc((usize, usize)) }
use self::State::*;
        
impl aoc::SearchState for State {
    /// We do not use labels here.
    type Label = ();

    /// The maze data is the global data for this problem.
    type Global = Maze;

    /// We do not use labels here.
    fn label(&self) -> () { () }

    /// State-space neighbors iterator.
    fn neighbors(&self, maze: &Maze) -> Vec<(usize, Box<State>)> {
        let &Loc(loc) = self;
        let mut result = Vec::new();
        let nbs = aoc::Neighbors::new(maze.grid_box, loc);
        for nb in nbs {
            if is_wall(maze.key, nb) {
                continue;
            };
            result.push((1usize, Box::new(Loc(nb))));
        };
        result
    }

    /// Goal state is part of maze data.
    fn is_goal(&self, maze: &Maze) -> bool {
        let &Loc(loc) = self;
        loc == maze.goal
    }

    /// The heuristic assumes no walls.
    fn hcost(&self, maze: &Maze) -> usize {
        let &Loc(loc) = self;
        aoc::manhattan_distance(loc, maze.goal)
    }
}

/// Solve part 1. Strategy: A\* search.
fn part1(key: usize, goal: (usize, usize)) -> usize {
    // Set up problem state.
    let maze = Maze {
        key: key,
        goal: goal,
        grid_box: aoc::GridBox::new_grid()
    };
        
    // Run the A\* search.
    match aoc::a_star(&maze, &Loc((1, 1)), false) {
        None => panic!("no solution"),
        Some((dist, _)) => dist
    }
}

/// Solve part 2. Strategy: flood fill with
/// distance recording.
fn part2(key: usize, max_g: usize) -> usize {
    // Set up the state.
    let grid_box = aoc::GridBox::new_grid();
    let mut fringe = HashSet::new();
    fringe.insert((1, 1));
    let mut stop = HashSet::new();

    // Keep flooding while there are new nodes to explore.
    for _ in 0..max_g {
        // Fringe for next round.
        let mut new_fringe = HashSet::new();

        // Explore existing fringe.
        for loc in fringe {
            for nb in aoc::Neighbors::new(grid_box, loc) {
                // Do not run into walls.
                if is_wall(key, nb) {
                    continue;
                };

                // Do not revisit already-explored locations.
                if stop.contains(&nb) {
                    continue;
                };

                // Found a new location.
                new_fringe.insert(nb);
                stop.insert(nb);
            };
        };

        // Update the fringe.
        fringe = new_fringe;
    };
    stop.len()
}


/// Read the problem description and run the search.
pub fn main() {
    let (part, args) = aoc::get_part_args();
    let key = args[0].parse::<usize>().expect("key not a number");

    match part {
        1 => {
            assert!(args.len() == 3);
            let goal_x =
                args[1].parse::<usize>().expect("x not a number");
            let goal_y =
                args[2].parse::<usize>().expect("y not a number");
            let n = part1(key, (goal_x, goal_y));
            println!("{}", n);
        },
        2 => {
            assert!(args.len() == 2);
            let max_g = args[1].parse::<usize>().expect("max_g not a number");
            let n = part2(key, max_g);
            println!("{}", n);
        },
        _ => {
            panic!("unknown part");
        }
    };
}
