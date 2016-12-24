// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

// Advent of Code Day 22.

use std::cmp::*;
use std::collections::{HashSet, BinaryHeap};

extern crate aoc;
extern crate regex;

#[derive(Clone, Debug)]
struct Usage {
    used: usize,
    avail: usize
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    goal_data: (usize, usize),
    blank: (usize, usize),
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

fn read_usages() -> Vec<(usize, usize, Usage)> {
    let usage_pat =
        regex::Regex::new(
            r"^/dev/grid/node-x(\d+)-y(\d+) *\d+T *(\d+)T *(\d+)T *\d+%$")
            .expect("could not compile usage pattern");
    let mut usages = Vec::new();
    for target in aoc::input_lines() {
        if let Some(args) = try_pat(&usage_pat, &target) {
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

fn count_viable_pairs(umap: &Vec<Vec<Usage>>) -> usize {
    let len_x = umap.len();
    let len_y = umap[0].len();
    let mut viable_pairs = 0;
    for x0 in 0..len_x {
    for y0 in 0..len_y {
        let used_a = umap[x0][y0].used;
        if used_a == 0 {
            continue;
        };
        for x1 in 0..len_x {
        for y1 in 0..len_y {
            if x1 == x0 && y1 == y0 {
                continue;
            };
            let avail_b = umap[x1][y1].avail;
            if avail_b >= used_a {
                viable_pairs += 1;
            }
        }};
    }};
    viable_pairs
}

fn start_info(umap: &Vec<Vec<Usage>>) -> (State, HashSet<(usize, usize)>) {
    let len_x = umap.len();
    let len_y = umap[0].len();
    let mut blank: Option<(usize, usize)> = None;
    let mut tiles: HashSet<(usize, usize)> = HashSet::new();
    for x in 0..len_x {
    for y in 0..len_y {
        if umap[x][y].used == 0 {
            if let Some(_) = blank {
                panic!("two blanks");
            };
            blank = Some((x, y));
            continue;
        };
        let is_tile = || {
            for dx in -1..2isize {
            for dy in -1..2isize {
                if dx == 0 && dy == 0 {
                    continue;
                };
                let x0 = x as isize + dx;
                let y0 = y as isize + dy;
                if x0 < 0 || x0 >= len_x as isize
                   || y0 < 0 || y0 >= len_y as isize {
                    continue;
                };
                let u = &umap[x0 as usize][y0 as usize];
                if umap[x][y].used > u.used + u.avail {
                    return false;
                }
            }};
            true
        };
        if is_tile() {
            tiles.insert((x, y));
        };
    }};
    let blank =
        match blank {
            Some(xy) => xy,
            None => panic!("no blank found")
        };
    (State { goal_data: (len_x - 1, 0),
             blank: blank }, tiles)
}

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
        print!("\n");
    };
}

fn mh_dist(p1: (usize, usize), p2: (usize, usize)) -> usize {
    (p1.0 as isize - p2.0 as isize).abs() as usize +
    (p1.1 as isize - p2.1 as isize).abs() as usize
}

fn hcost(state: &State) -> usize {
    max(0,
        mh_dist(state.goal_data, state.blank) +
        mh_dist(state.goal_data, (0,0)))
}

fn a_star(start: &State, tiles: &HashSet<(usize, usize)>) -> Option<usize> {
    let dirns = [(1, 0), (0, 1), (-1, 0), (0, -1)];
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
                        for &(dx, dy) in dirns.iter() {
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

pub fn main() {
    let part = aoc::get_part();
    assert!(part == 1 || part == 2);
    let usages = read_usages();
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
    let mut umap: Vec<Vec<Usage>> = Vec::new();
    let mut row = Vec::new();
    row.resize(len_y, Usage{used: 0, avail: 0});
    umap.resize(len_x, row);
    for (x, y, u) in usages.iter().cloned() {
        umap[x][y] = u;
    };
    if part == 1 {
        print!("{}\n", count_viable_pairs(&umap));
    } else {
        let (start, tiles) = start_info(&umap);
        print_map(&start, &tiles, len_x, len_y);
        if let Some(g) = a_star(&start, &tiles) {
            print!("{}\n", g);
        } else {
            print!("no solution\n");
        };            
    };
}
