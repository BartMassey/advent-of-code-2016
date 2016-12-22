// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

// Advent of Code Day 22.

use std::io;
use std::io::prelude::*;
use std::cmp::*;
use std::collections::HashSet;
//use std::collections::{BTreeSet, BinaryHeap};

extern crate aoc;
extern crate regex;

#[derive(Clone, Debug)]
struct Usage {
    used: usize,
    avail: usize
}

#[derive(Clone, Debug)]
struct State {
    goal_data: (usize, usize),
    blank: (usize, usize),
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
    let stdin = io::stdin();
    let reader = io::BufReader::new(stdin);
    for line in reader.lines() {
        let target = line.expect("main: could not read line");
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
    let mut viable_pairs = 0;
    if part == 1 {
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
                    }
                }
            }
        };
        print!("{}\n", viable_pairs);
    } else {
        let mut blank: Option<(usize, usize)> = None;
        let mut blocks: HashSet<(usize, usize)> = HashSet::new();
        for x in 0..len_x {
            for y in 0..len_y {
                if umap[x][y].used == 0 {
                    if let Some(_) = blank {
                        panic!("two blanks");
                    };
                    blank = Some((x, y));
                    continue;
                };
                let mut find_blocks = || {
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
                                blocks.insert((x, y));
                                return;
                            }
                        }
                    }
                };
                find_blocks();
            }
        };
        let blank =
            match blank {
                Some(xy) => xy,
                None => panic!("no blank found")
            };
        for y in 0..len_y {
            for x in 0..len_x {
                if (x, y) == blank {
                    print!("_");
                } else if blocks.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                };
            }
            print!("\n");
        };
    };
}
