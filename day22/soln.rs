// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

// Advent of Code Day 22.

use std::cmp::*;
use std::collections::{HashSet, BinaryHeap};

extern crate aoc;
extern crate regex;

/// Turn on to show board for part 2.
const SHOW: bool = false;

/// Four cardinal directions.
static DIRNS: [(isize, isize);4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];


/// Disk usage per node.
#[derive(Clone, Debug)]
struct Usage {
    /// In-use.
    used: usize,
    /// Free to use.
    avail: usize
}

/// The game state is just the position of the
/// target data and blank.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    /// Position of target data.
    goal_data: (usize, usize),
    /// Position of blank.
    blank: (usize, usize),
}

/// Node in A\* search.
#[derive(Clone, Debug)]
struct PQElem {
    /// Cost so far.
    cost: usize,
    /// Total heuristic cost.
    fcost: usize,
    /// Actual state.
    state: State
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
    /// Break ties in ordering by smallest heuristic cost
    /// using largest actual cost.
    fn cmp(&self, other: &PQElem) -> Ordering {
        match other.fcost.cmp(&self.fcost) {
            Ordering::Equal =>
                self.cost.cmp(&other.cost),
            c => c
        }
    }
}

/// Try to match given regex, returning submatches if successful.
fn try_pat(pat: &regex::Regex, target: &str) -> Option<Vec<String>> {
    match pat.captures(target) {
        Some(parts) =>  {
            let mut v = Vec::new();
            let mut iter = parts.iter();
            iter.next();
            for p in iter {
                v.push(p.expect("part did not match").to_string());
            };
            Some(v)
        },
        None => None
    }
}

/// Read usages from `stdin` and return a vector with
/// position and usage information.
fn read_usages() -> Vec<(usize, usize, Usage)> {
    // Set up state.
    let usage_pat =
        regex::Regex::new(
            r"^/dev/grid/node-x(\d+)-y(\d+) *\d+T *(\d+)T *(\d+)T *\d+%$")
            .expect("could not compile usage pattern");
    let mut usages = Vec::new();

    // Process each line.
    for target in aoc::input_lines() {
        if let Some(args) = try_pat(&usage_pat, &target) {
            assert!(args.len() == 4);
            let mut argv = [0usize;4];
            for i in 0..4 {
                argv[i] = args[i].parse::<usize>().unwrap();
            }
            usages.push((argv[0], argv[1],
                         Usage{ used: argv[2], avail: argv[3]}));
        };
    };
    usages
}

/// Use the obvious quadratic algorithm to test each pair
/// For viability. Pairs are directed, so no need to divide
/// by two or anything like that.
fn count_viable_pairs(umap: &Vec<Vec<Usage>>) -> usize {
    // Set up state.
    let len_x = umap.len();
    let len_y = umap[0].len();
    let mut viable_pairs = 0;

    // Loop over source coordinates.
    for x0 in 0..len_x {
    for y0 in 0..len_y {
        // Get source use for later.
        let used_a = umap[x0][y0].used;

        // Never pull from empty disc.
        if used_a == 0 {
            continue;
        };

        // Loop over destination coordinates.
        for x1 in 0..len_x {
        for y1 in 0..len_y {
            // Source and dest must be different.
            if x1 == x0 && y1 == y0 {
                continue;
            };

            // Get dest space, and bump count if source
            // fits.
            let avail_b = umap[x1][y1].avail;
            if avail_b >= used_a {
                viable_pairs += 1;
            }
        }};
    }};
    viable_pairs
}

/// Heuristically transform the given resource usage map
/// into a sliding tile puzzle using the methodology
/// detailed in the description of part 2. Return
/// a starting state and a set of coordinates that
/// are "wall squares".
fn start_info(umap: &Vec<Vec<Usage>>) -> (State, HashSet<(usize, usize)>) {
    // Set up state.
    let len_x = umap.len();
    let len_y = umap[0].len();
    let mut blank: Option<(usize, usize)> = None;
    let mut tiles: HashSet<(usize, usize)> = HashSet::new();

    // Loop over source coordinates.
    for x in 0..len_x {
    for y in 0..len_y {
        // An empty disc represents the blank. There can
        // be only one.
        if umap[x][y].used == 0 {
            if let Some(_) = blank {
                panic!("two blanks");
            };
            blank = Some((x, y));
            continue;
        };

        // Heuristically decide whether the given square is
        // a tile or a wall. Block because early return.
        let is_tile = || {
            // Loop over immediate neighbors.
            for &(dx, dy) in DIRNS.iter() {
                // Compute neighbor coordinate.
                let x0 = x as isize + dx;
                let y0 = y as isize + dy;
                if x0 < 0 || x0 >= len_x as isize
                   || y0 < 0 || y0 >= len_y as isize {
                    continue;
                };

                // Heuristic: If this neighbor could not
                // accept the data if empty, it is a wall.
                let u = &umap[x0 as usize][y0 as usize];
                if umap[x][y].used > u.used + u.avail {
                    return false;
                }
            };
            true
        };

        // Run the block.
        if is_tile() {
            tiles.insert((x, y));
        };
    }};

    // Check on the blank.
    let blank =
        match blank {
            Some(xy) => xy,
            None => panic!("no blank found")
        };

    // Return the new state and tiles.
    (State { goal_data: (len_x - 1, 0), blank: blank }, tiles)
}

// Display the part 2 map for debugging.
fn print_map(s: &State, tiles: &HashSet<(usize, usize)>,
             len_x: usize, len_y: usize) {
    for y in 0..len_y {
        for x in 0..len_x {
            if (x, y) == s.blank {
                print!("_");
            } else if (x, y) == s.goal_data {
                print!("G");
            } else if tiles.contains(&(x, y)) {
                print!(".");
            } else {
                print!("#");
            };
        };
        println!("");
    };
}

// Manhattan distance between coordinates.
fn mh_dist(p1: (usize, usize), p2: (usize, usize)) -> usize {
    (p1.0 as isize - p2.0 as isize).abs() as usize +
    (p1.1 as isize - p2.1 as isize).abs() as usize
}

// An admissible heuristic cost for a state is the cost of
// getting the blank next to the data plus the cost of
// getting the data to the goal.
fn hcost(state: &State) -> usize {
    max(0,
        mh_dist(state.goal_data, state.blank) +
        mh_dist(state.goal_data, (0,0)))
}

// Generic A\* search.
fn a_star(start: &State, tiles: &HashSet<(usize, usize)>) -> Option<usize> {
    let mut stop_list = HashSet::new();
    let mut pq = BinaryHeap::new();
    pq.push(PQElem{ state: start.clone(), cost: 0, fcost: hcost(start) });
    loop {
        match pq.pop() {
            Some(PQElem{cost: g, fcost: _, state}) => {
                if state.goal_data == (0, 0) {
                    return Some(g);
                };
                match stop_list.insert(state.clone()) {
                    false => { continue; },
                    true => {
                        let (x, y) = state.blank;
                        for &(dx, dy) in DIRNS.iter() {
                            let next_x = x as isize + dx;
                            let next_y = y as isize + dy;
                            let next_blank = (next_x as usize,
                                              next_y as usize);
                            if !tiles.contains(&next_blank) {
                                continue;
                            };
                            let mut next_goal_data = state.goal_data;
                            if next_blank ==  state.goal_data {
                                next_goal_data = state.blank;
                            };
                            let next_state = State {
                                goal_data: next_goal_data,
                                blank: next_blank
                            };
                            let h = hcost(&next_state);
                            let g = g + 1;
                            pq.push(PQElem{fcost: g + h as usize, cost: g,
                                           state: next_state});
                        };
                    }
                };
            },
            None => {
                return None
            }
        }
    };
}

// Do the appropriate search through the disc array.
pub fn main() {
    let part = aoc::get_part();
    assert!(part == 1 || part == 2);

    // Process input.
    let usages = read_usages();

    // Find maximum x, y and total use.
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_t = 0;
    for (x, y, u) in usages.iter().cloned() {
        max_x = max(max_x, x);
        max_y = max(max_y, y);
        max_t = max(max_t, u.avail + u.used);
    };
    let len_x = max_x + 1;
    let len_y = max_y + 1;

    // Build the map.
    let mut umap: Vec<Vec<Usage>> = Vec::new();
    let mut row = Vec::new();
    row.resize(len_y, Usage{used: 0, avail: 0});
    umap.resize(len_x, row);
    for (x, y, u) in usages.iter().cloned() {
        umap[x][y] = u;
    };

    if part == 1 {
        // Just count the pairs.
        println!("{}", count_viable_pairs(&umap));
    } else {
        // Solve the sliding tile puzzle.
        let (start, tiles) = start_info(&umap);
        if SHOW {
            print_map(&start, &tiles, len_x, len_y);
        };
        if let Some(g) = a_star(&start, &tiles) {
            println!("{}", g);
        } else {
            println!("no solution");
        };            
    };
}
