// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

// Advent of Code Day 11.

use std::io;
use std::io::prelude::*;
use std::collections::{BTreeSet, BinaryHeap};
use std::iter::*;
use std::cmp::*;

extern crate aoc;
extern crate regex;

#[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
enum Dev {
    Gen(String),
    Chip(String)
}

// Returns true iff the contents can be left together without incident.
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


// Based on https://doc.rust-lang.org/std/collections/binary_heap

#[derive(Clone, Debug, PartialOrd, Ord)]
struct State {
    location: usize,
    floors: Vec<BTreeSet<Dev>>
}

impl State {

    // Initial state.
    fn start(floors: Vec<BTreeSet<Dev>>) -> Self {
        State {
            location: 0,
            floors: floors
        }
    }

    fn floor_numbers(&self, dirn: isize) -> Option<(usize, usize)> {
        let src = self.location;
        let dest = src as isize + dirn;
        if dest < 0 || dest >= self.floors.len() as isize {
            return None;
        };
        assert!(dest >= 0 && dest < self.floors.len() as isize);
        Some((src, dest as usize))
    }
        

    // 1. Grab specified elevator contents from current floor.
    // 2. Move in specified direction to next floor.
    // 3. Drop the contents of the elevator on the new floor.
    // If the traverse is legal, return the state resulting from these operations.
    fn try_traverse(&self, dirn: isize, grab: &BTreeSet<Dev>) -> Option<State> {
        if grab.is_empty() {
            return None;
        };
        let (src, dest) =
            match self.floor_numbers(dirn) {
                Some(ns) => ns,
                None => { return None; }
            };
        let new_src = self.floors[src].difference(grab).cloned().collect();
        if !contents_are_safe(&new_src) {
            return None;
        };
        let new_dest = self.floors[dest].union(grab).cloned().collect();
        if !contents_are_safe(&new_dest) {
            return None;
        };
        let mut new_floors = self.floors.clone();
        new_floors[src] = new_src;
        new_floors[dest] = new_dest;
        let new_state = State {
            location: dest,
            floors: new_floors
        };
        Some(new_state)
    }

    // Return the set of states that can result from legal traversals
    // from the current state.
    fn traversals(&self) -> BTreeSet<State> {
        let mut ts = BTreeSet::new();
        for dirn in [-1, 1].iter() {
            let src =
                match self.floor_numbers(*dirn) {
                    Some((src, _)) => src,
                    None => continue
                };
            for grab in aoc::choose_le(&self.floors[src], 2) {
                match self.try_traverse(*dirn, &grab) {
                    Some(t) => assert!(ts.insert(t)),
                    None => ()
                };
            };
        };
        ts
    }

    fn is_goal(&self) -> bool {
        if self.location != self.floors.len() - 1 {
            return false;
        }
        for i in 0..self.floors.len()-1 {
            if !self.floors[i].is_empty() {
                return false;
            }
        }
        return true;
    }

    fn hcost(&self) -> usize {
        let mut dists = 0;
        for i in 0..self.floors.len()-1 {
            dists += (3 - i) * self.floors[i].len();
        };
        max(0, 2 * (dists as isize - 4)) as usize
    }
}

impl Eq for State {}

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        self.location == other.location &&
        self.floors == other.floors
    }
}

#[derive(Clone, Debug)]
struct PQElem {
    cost: usize,
    fcost: usize,
    state: State
}

impl PartialEq for PQElem {
    fn eq(&self, other: &PQElem) -> bool {
        other.fcost == self.fcost && other.cost == self.cost
    }
}

impl Eq for PQElem {}

impl PQElem {
    fn start(state: &State) -> PQElem {
        PQElem{cost: 0, fcost: 0, state: state.clone()}
    }
}

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

fn read_start_state() -> State {
    let device_pat = regex::Regex::new(r"an? ([-a-z]+) ([a-z]+)")
        .expect("main: could not compile device pattern");
    let compatible_pat = regex::Regex::new(r"^([a-z]+)(-compatible)?$")
        .expect("main: could not compile compatible pattern");
    let floors: Vec<&str> = vec!["first", "second", "third", "fourth"];
    let mut floors0: Vec<BTreeSet<Dev>> = Vec::new();
    let mut floor = 0;
    let stdin = io::stdin();
    let reader = io::BufReader::new(stdin);
    for line in reader.lines() {
        let target = line.expect("main: could not read line");
        assert!(floor <= 3);
        floors0.push(BTreeSet::new());
        if floor == 3 {
            assert!(target == "The fourth floor contains nothing relevant.");
            continue;
        };
        let floor_pat =
            regex::Regex::new(&format!(r"^The {} floor contains .*\.$", floors[floor]))
            .expect("main: could not compile floor pattern");
        assert!(floor_pat.is_match(&target));
        for parts in device_pat.captures_iter(&target) {
            let dev_desc = parts.at(1)
                .expect("main: could not find device description");
            let dev_mat = String::from(compatible_pat.captures(&dev_desc)
                                       .expect("main: could not parse device description")
                                       .at(1).expect("main: could not find device description"));
            let dev_class = parts.at(2)
                .expect("main: could not find device class");
            let dev =
                if dev_class == "generator" {
                    Dev::Gen(dev_mat)
                } else if dev_class == "microchip" {
                    Dev::Chip(dev_mat)
                } else {
                    panic!("main: unknown device")
                };
            if !floors0[floor].insert(dev) {
                panic!("main: device inserted twice");
            }
        };
        floor += 1;
    };
    State::start(floors0)
}

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
            None => { panic!(format!("could not traverse {} {:?} in state {:?}", dirn, *grab, state)); }
        };
    };
    if !state.is_goal() {
        panic!("state {:?} is not goal\n", state);
    };
}

pub fn main() {
    let state0 = read_start_state();
    let mut stop_list = BTreeSet::new();
    let mut pq = BinaryHeap::new();
    pq.push(PQElem::start(&state0));
    loop {
        match pq.pop() {
            Some(PQElem{cost: gcost, fcost: _, state}) => {
                if state.is_goal() {
                    print!("cost {} for {:?}\n", gcost, state);
                    return;
                };
                match stop_list.insert(state.clone()) {
                    false => { continue; },
                    true => {
                        for next_state in state.traversals() {
                            if !stop_list.contains(&next_state) {
                                let hcost = state.hcost();
                                let gcost = gcost + 1;
                                pq.push(PQElem{fcost: gcost + hcost, cost: gcost, state: next_state});
                            }
                        }
                    }
                };
            },
            None => { panic!("found no solution"); },
        }
    };
}
