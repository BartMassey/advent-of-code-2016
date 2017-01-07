// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! Advent of Code Day 11.

/// For input checking and processing, it is convenient
/// to hardcode the floor count.
/// 
/// It would be reasonably straightforward to get rid of
/// this constant, but harder if buggy inputs were to be
/// excluded. In particular, one would probably need to
/// build a cardinal to ordinal string converter that would,
/// for example, code 21 as "twenty-first".
const NFLOORS: usize = 4;

use std::collections::BTreeSet;
use std::iter::*;
use std::cmp::*;

extern crate aoc;
extern crate regex;

/// Kind of device.
#[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
enum Dev {
    /// Generator with given material name.
    Gen(String),
    /// Microchip with given material name.
    Chip(String)
}

/// Returns true iff the floor contents can be left together
/// without incident. *Idea:* either there are no generators,
/// or every chip is protected by its generator.
fn contents_are_safe(contents: &BTreeSet<Dev>) -> bool {
    let mut gens = BTreeSet::new();
    let mut chips = BTreeSet::new();
    for c in contents.into_iter().cloned() {
        match c {
            Dev::Gen(name) => gens.insert(name),
            Dev::Chip(name) => chips.insert(name)
        };
    };
    gens.is_empty() || chips.is_subset(&gens)
}


/// Problem state.
#[derive(Clone, Debug, PartialOrd, Ord, Eq, PartialEq)]
struct State {
    /// Current floor.
    location: usize,
    /// Contents of floors.
    floors: Vec<BTreeSet<Dev>>
}

impl State {

    /// Initial state for given problem.
    fn start(floors: Vec<BTreeSet<Dev>>) -> Self {
        State {
            location: 0,
            floors: floors
        }
    }

    /// Check whether the specified traversal is possible.
    /// If so, return current and next floor numbers.
    fn try_move(&self, dirn: isize) -> Option<(usize, usize)> {
        let src = self.location;
        let dest = src as isize + dirn;
        if dest < 0 || dest >= self.floors.len() as isize {
            return None;
        };
        Some((src, dest as usize))
    }
        

    /// 1. Grab specified elevator contents from current floor.
    /// 2. Move in specified direction to next floor.
    /// 3. Drop the contents of the elevator on the new floor.
    ///
    /// If the traverse is legal, return the state resulting
    /// from these operations.
    fn try_traverse(&self, dirn: isize, grab: &BTreeSet<Dev>)
    -> Option<State> {
        // Can't move off either end of the elevator shaft.
        let (src, dest) =
            match self.try_move(dirn) {
                Some(ns) => ns,
                None => { return None; }
            };

        // Can't move with an empty elevator.
        if grab.is_empty() {
            return None;
        };

        // Can't move if we leave the source floor unsafe.
        let new_src = self.floors[src].difference(grab).cloned().collect();
        if !contents_are_safe(&new_src) {
            return None;
        };

        // Can't move if we leave the destination floor unsafe.
        let new_dest = self.floors[dest].union(grab).cloned().collect();
        if !contents_are_safe(&new_dest) {
            return None;
        };

        // Return the state resulting from the traversal.
        let mut new_floors = self.floors.clone();
        new_floors[src] = new_src;
        new_floors[dest] = new_dest;
        let new_state = State {
            location: dest,
            floors: new_floors
        };
        Some(new_state)
    }

    /// Set of states reachable from this state by legal
    /// traversal.
    fn traversals(&self) -> BTreeSet<State> {
        // Try moving both down and up.
        let mut ts = BTreeSet::new();
        for dirn in &[-1, 1] {
            // Precheck for efficiency to avoid working
            // through all possible grabs on an illegal
            // traversal.
            if self.try_move(*dirn).is_none() {
                continue;
            };
            // Try all possible grabs for traversal in
            // given direction.
            for grab in aoc::choose_le(&self.floors[self.location], 2) {
                if let Some(t) = self.try_traverse(*dirn, &grab) {
                    assert!(ts.insert(t));
                }
            };
        };
        ts
    }
}

impl aoc::SearchState for State {
    /// We do not use labels here.
    type Label = ();

    /// There is no global data for this problem.
    type Global = ();

    /// We do not use labels here.
    fn label(&self) -> () { () }

    /// State-space neighbors.
    fn neighbors(&self, _: &()) -> Vec<(usize, Box<State>)> {
        self.traversals().into_iter()
            .map(move |s|{(1usize, Box::new(s))}).collect()
    }

    /// Check whether the current state has the right
    /// contents.
    fn is_goal(&self, _: &()) -> bool {
        // Have to be on the top floor.
        if self.location != self.floors.len() - 1 {
            return false;
        }

        // Have to have all the stuff on the top floor.
        for i in 0..self.floors.len()-1 {
            if !self.floors[i].is_empty() {
                return false;
            }
        }
        
        true
    }

    /// Admissible heuristic for remaining number
    /// of traversals to solve the problem.
    ///
    /// *Idea:* Except for the last batch of stuff,
    /// everything can move only individually, by way of
    /// carrying to the top floor with something else in the
    /// elevator, then carrying that something down to pick
    /// up the next thing.
    ///
    /// This isn't quite right, because moves could involve
    /// shuffling stuff around on the way up and down, but
    /// that doesn't actually lengthen the required
    /// distance.
    fn hcost(&self, _: &()) -> usize {
        let mut dists = 0;
        let nfloors = self.floors.len();
        for i in 0..nfloors-1 {
            dists += (nfloors - i - 1) * self.floors[i].len();
        };
        max(0, 2 * (dists as isize - nfloors as isize)) as usize
    }

}

/// Pull in the problem description.
fn read_start_state() -> State {
    // Set up the patterns needed to parse the description.
    let device_pat = regex::Regex::new(r"an? ([-a-z]+) ([a-z]+)")
        .expect("main: could not compile device pattern");
    let compatible_pat = regex::Regex::new(r"^([a-z]+)(-compatible)?$")
        .expect("main: could not compile compatible pattern");
    let floors: Vec<&str> = vec!["first", "second", "third", "fourth"];
    assert!(floors.len() == NFLOORS);

    // Set up the state.
    let mut floors0: Vec<BTreeSet<Dev>> = Vec::new();
    let mut floor = 0;

    // Walk over the target description, placing things as
    // specified.
    for target in aoc::input_lines() {
        // Start the new floor.
        floors0.push(BTreeSet::new());

        // Check for top floor (or greater).
        assert!(floor < NFLOORS);
        if floor == NFLOORS - 1 {
            let top_floor =
                format!("The {} floor contains nothing relevant.",
                        floors[NFLOORS - 1]);
            assert!(target == top_floor);
            continue;
        };

        // Get the list of stuff on the floor.
        let floor_pat =
            regex::Regex::new(&format!(
                r"^The {} floor contains .*\.$", floors[floor]
            )).expect("main: could not compile floor pattern");
        assert!(floor_pat.is_match(&target));

        // Add all the stuff on the floor.
        for parts in device_pat.captures_iter(&target) {
            // Get the device type and class.
            let dev_desc = parts.at(1)
                .expect("main: could not find device description");
            let dev_mat = String::from(
                compatible_pat.captures(dev_desc)
                .expect("main: could not parse device description")
                .at(1).expect("main: could not find device description")
            );
            let dev_class = parts.at(2)
                            .expect("main: could not find device class");

            // Build the device.
            let dev =
                if dev_class == "generator" {
                    Dev::Gen(dev_mat)
                } else if dev_class == "microchip" {
                    Dev::Chip(dev_mat)
                } else {
                    panic!("main: unknown device")
                };

            // Put the device on the floor.
            if !floors0[floor].insert(dev) {
                panic!("main: device inserted twice");
            }
        };
        floor += 1;
    };
    State::start(floors0)
}

// Try the test traversal given with the problem.
#[test]
fn test1() {
    let hydrogen = String::from("hydrogen");
    let lithium = String::from("lithium");
    let chip_hydrogen = Dev::Chip(hydrogen.clone());
    let chip_lithium = Dev::Chip(lithium.clone());
    let gen_hydrogen = Dev::Gen(hydrogen.clone());
    let gen_lithium = Dev::Gen(lithium.clone());
    let mut state = State::start(vec![
        aoc::make_set(&[chip_hydrogen.clone(), chip_lithium.clone()]),
        aoc::make_set(&[gen_hydrogen.clone()]),
        aoc::make_set(&[gen_lithium.clone()]),
        BTreeSet::new() ]);
    let steps: [(isize, BTreeSet<Dev>); 11] = [
        ( 1, aoc::make_set(&[chip_hydrogen.clone()])),
        ( 1, aoc::make_set(&[chip_hydrogen.clone(), gen_hydrogen.clone()])),
        (-1, aoc::make_set(&[chip_hydrogen.clone()])),
        (-1, aoc::make_set(&[chip_hydrogen.clone()])),
        ( 1, aoc::make_set(&[chip_hydrogen.clone(), chip_lithium.clone()])),
        ( 1, aoc::make_set(&[chip_hydrogen.clone(), chip_lithium.clone()])),
        ( 1, aoc::make_set(&[chip_hydrogen.clone(), chip_lithium.clone()])),
        ( -1, aoc::make_set(&[chip_hydrogen.clone()])),
        ( 1, aoc::make_set(&[gen_hydrogen.clone(), gen_lithium.clone()])),
        ( -1, aoc::make_set(&[chip_lithium.clone()])),
        ( 1, aoc::make_set(&[chip_hydrogen.clone(), chip_lithium.clone()]))];
    for step in steps.iter() {
        let (dirn, ref grab) = *step;
        match state.try_traverse(dirn, grab) {
            Some(next_state) => { state = next_state; },
            None => { panic!(format!("bad traverse {} {:?} in state {:?}",
                                     dirn, *grab, state)); }
        };
    };
    if !state.is_goal() {
        panic!("state {:?} is not goal\n", state);
    };
}

/// Grab the input, run the A\* search, show the result.
pub fn main() {
    let args = aoc::get_args();
    assert!(args.len() == 0);
    let start = read_start_state();
    match aoc::a_star(&(), &start, false) {
        None => { panic!("no solution"); },
        Some((cost, None)) => { println!("{}", cost); },
        _ => { panic!("internal error: weird astar return"); }
    };
}
