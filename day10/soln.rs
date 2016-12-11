// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

// Advent of Code Day 10

// Turn this on to see the closure operations.
const TRACE: bool = false;

use std::cmp::*;
use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate aoc;

lazy_static! {
    static ref VALUE_PAT: regex::Regex =
        regex::Regex::new(r"^value (\d+) goes to (.+)$")
        .expect("could not compile regex for value");
    static ref COMPARE_PAT: regex::Regex =
        regex::Regex::new(r"^bot (\d+) gives low to (.+) and high to (.+)$")
        .expect("could not compile regex for compare");
    static ref BOT_PAT: regex::Regex =
        regex::Regex::new(r"^bot (\d+)$")
        .expect("could not compile regex for bot");
    static ref OUTPUT_PAT: regex::Regex =
        regex::Regex::new(r"^output (\d+)$")
        .expect("could not compile regex for output");
}

enum Dest {
    Bot(usize),
    Output(usize)
}

enum Insn {
    Value(usize, Dest),           // (<value>, <dest>)
    Compare(usize, Dest, Dest)   // (<robot>, <lowdest>, <highdest>)
}

// Take a destination description string to a dest.
fn parse_dest(desc: &str) -> Dest {
    match (*self::BOT_PAT).captures(desc) {
        None => (),
        Some(parts) => {
            return Dest::Bot(parts.at(1).expect("parse_dest: could not find dest bot")
                             .parse().expect("parse_dest: could not parse dest bot"));
        }
    };
    match (*self::OUTPUT_PAT).captures(desc) {
        None => (),
        Some(parts) => {
            return Dest::Output(parts.at(1).expect("parse_dest: could not find dest output")
                                .parse().expect("parse_dest: could not parse dest output"));
        }
    };
    panic!("unrecognized dest");
}

// There's a lot here. General strategy is to compute the
// transitive closure of the value instructions over all
// possible bot processing. It turns out that in the
// instance as given no bot ever does more than one job (each bot
// compares exactly two values of chip), but this code will
// handle the more general case.
// 
// XXX I've violated a long-standing rule and let this code
// go more than 80 columns wide. It should probably be
// broken up into functions, but Rust and the problem conspire
// to make that hard.
pub fn main() {
    let (part, target) = aoc::get_part_args();
    let mut target_low = 0;
    let mut target_high = 0;
    if part == 1 {
        assert!(target.len() == 2);
        target_low = target[0].parse().expect("main: could not parse low target");
        target_high = target[1].parse().expect("main: could not parse high target");
    } else {
        assert!(target.len() == 0);
    }
    // Set up the instruction table and prepare to set up the rest of the state.
    let mut insns: Vec<Insn> = Vec::new();
    let mut max_bot = 0;
    let mut max_output = 0;
    // Read strings from the input file and process them.
    let stdin = io::stdin();
    let reader = io::BufReader::new(stdin);
    for line in reader.lines() {
        let l = line.expect("main: could not read line");
        // Parse the instruction and add it to the instruction table.
        match (*self::VALUE_PAT).captures(&l) {
            None => (),
            Some(parts) => {
                let value_part = parts.at(1).expect("main: could not find value")
                    .parse().expect("main: could not parse value");
                let dest_part = parse_dest(parts.at(2).expect("main: could not find dest"));
                match dest_part {
                    Dest::Bot(b) => { max_bot = max(b, max_bot); },
                    Dest::Output(q) => { max_output = max(q, max_output); },
                };
                insns.push(Insn::Value(value_part, dest_part));
                continue;
            }
        };
        match (*self::COMPARE_PAT).captures(&l) {
            None => (),
            Some(parts) => {
                let bot_part = parts.at(1).expect("main: could not find bot")
                    .parse().expect("main: could not parse bot");
                max_bot = max(bot_part, max_bot);
                let dest_low_part = parse_dest(parts.at(2).expect("main: could not find low"));
                match dest_low_part {
                    Dest::Bot(b) => { max_bot = max(b, max_bot); },
                    Dest::Output(q) => { max_output = max(q, max_output); },
                };
                let dest_high_part = parse_dest(parts.at(3).expect("main: could not find high"));
                match dest_high_part {
                    Dest::Bot(b) => { max_bot = max(b, max_bot); },
                    Dest::Output(q) => { max_output = max(q, max_output); },
                };
                insns.push(Insn::Compare(bot_part, dest_low_part,
                                         dest_high_part));
                continue;
            }
        };
        panic!("unrecognized instruction");
    };
    // Set up the rest of the state. The holds table is used to
    // store the set of values each robot could hold.
    let mut holds: Vec<HashSet<usize>> = Vec::with_capacity(max_bot + 1);
    holds.resize(max_bot + 1, HashSet::new());
    // The outputs table is used to store the value that could eventually
    // be present on each output. XXX It is assumed that each output is unique.
    let mut outputs: Vec<Option<usize>> = Vec::with_capacity(max_output + 1);
    outputs.resize(max_output + 1, None);
    // Process the value instructions. They will never be referenced again.
    for i in insns.iter() {
        match *i {
            Insn::Value(v, Dest::Bot(b)) => { let _ = holds[b].insert(v); },
            Insn::Value(v, Dest::Output(q)) => { outputs[q] = Some(v); },
            _ => ()
        }
    };
    // Compute the reflexive transitive closure of the possible bot operations.
    // Stop when the part 1 solution is found, or when the closure is complete.
    let mut changed = true;
    while changed {
        changed = false;
        // Consider each bot in turn and see if there's something new there.
        for b in 0..holds.len() {
            // Consider each possible pair of distinct values bot b could hold.
            // Note that this does the right thing for bots holding 0 or 1 values.
            for vl in holds[b].clone().iter() {
                for vh in holds[b].clone().iter() {
                    // Consider only canonical pairs.
                    if *vl >= *vh {
                        continue;
                    }
                    // Check for part 1 solution if needed.
                    if part == 1 && holds[b].contains(&target_low) && holds[b].contains(&target_high) {
                        print!("{}\n", b);
                        return;
                    }
                    // Run the compare instruction over the current canonical compare.
                    for i in insns.iter() {
                        match *i {
                            Insn::Compare(b0, ref dl, ref dh) => if b == b0 {
                                match *dl {
                                    Dest::Bot(bl) => {
                                        if holds[bl].insert(*vl) {
                                            changed = true;
                                            if TRACE {
                                                print!("{} low {} to {}({})\n",
                                                       b, *vl, bl, holds[bl].len());
                                            }
                                        }
                                    },
                                    Dest::Output(ql) => {
                                        let unchanged = outputs[ql] == Some(*vl);
                                        assert!(outputs[ql] == None || unchanged);
                                        if !unchanged {
                                            outputs[ql] = Some(*vl);
                                            changed = true;
                                            if TRACE {
                                                print!("{} low {} to output {}\n",
                                                       b, *vl, ql);
                                            }
                                        }
                                    }
                                };
                                match *dh {
                                    Dest::Bot(bh) => if holds[bh].len() < 2 {
                                        if holds[bh].insert(*vh) {
                                            changed = true;
                                            if TRACE {
                                                print!("{} high {} to {}({})\n",
                                                       b, *vh, bh, holds[bh].len());
                                            }
                                        }
                                    },
                                    Dest::Output(qh) => {
                                        let unchanged = outputs[qh] == Some(*vh);
                                        assert!(outputs[qh] == None || unchanged);
                                        if !unchanged {
                                            outputs[qh] = Some(*vh);
                                            changed = true;
                                            if TRACE {
                                                print!("{} high {} to output {}\n",
                                                       b, *vh, qh);
                                            }
                                        }
                                    }
                                };
                                break;
                            },
                            _ => ()
                        }
                    }
                }
            }
        }
    };
    // Handle the two parts separately, reporting errors for debugging.
    if part == 1 {
        // Part 1 has definitely failed.
        print!("no soln found\n");
        for b in 0..holds.len() {
            print!("{}:", b);
            for v in holds[b].iter() {
                print!(" {}", *v);
            };
            print!("\n");
        }
    } else {
        // Part 2 may have succeeded.
        match (outputs[0], outputs[1], outputs[2]) {
            (Some(v1), Some(v2), Some(v3)) => { print!("{}\n", v1 * v2 * v3); },
            _ => {
                // Part 2 has definitely failed.
                print!("no solution found\n");
                for q in 0..outputs.len() {
                    print!("{}: ", q);
                    match outputs[q] {
                        None => { print!("-\n"); },
                        Some(v) => { print!("{}\n", v); }
                    }
                }
            }
        }
    };
}
