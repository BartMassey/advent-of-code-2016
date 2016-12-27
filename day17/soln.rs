// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

// Advent of Code Day 17.

extern crate crypto;
use self::crypto::digest::Digest;
use self::crypto::md5::Md5;

extern crate aoc;

#[derive(Clone)]
enum Explo {
    Finished,
    Stopped,
    Completed(String)
}

use self::Explo::*;

fn open_doors(hasher0: &Md5) -> Vec<bool> {
    let mut hasher = (*hasher0).clone();
    let mut output = [0; 16];
    hasher.result(&mut output);
    let nybbles = [(output[0] >> 4) & 0xf, output[0] & 0xf,
                   (output[1] >> 4) & 0xf, output[1] & 0xf];
    let mut result = Vec::new();
    for nybble in nybbles.iter() {
        if *nybble >= 11 {
            result.push(true);
        } else {
            result.push(false);
        }
    };
    result
}

#[test]
fn test_open_doors() {
    let mut hasher = crypto::md5::Md5::new();
    hasher.input("hijkl".as_bytes());
    assert!(open_doors(&hasher) == vec![true, true, true, false]);
}

fn dfid(hasher0: &Md5, limit: usize, posn: (isize, isize),
        path: String, find_longest: bool) -> Explo {
    if posn == (3, 3) {
        return Completed(path);
    };
    if !find_longest && limit == 0 {
        return Stopped;
    }
    let (x, y) = posn;
    let dirns = [
        ('U', (0, -1), 0),
        ('D', (0, 1), 1),
        ('L', (-1, 0), 2),
        ('R', (1, 0), 3) ];
    let doors = open_doors(hasher0);
    let mut result = Finished;
    let mut found_longest = false;
    for &(dirn, (dx, dy), door) in dirns.iter() {
        if !doors[door] {
            continue;
        };
        let next_x = x + dx;
        if next_x < 0 || next_x >= 4 {
            continue;
        };
        let next_y = y + dy;
        if next_y < 0 || next_y >= 4 {
            continue;
        };
        let mut hasher = (*hasher0).clone();
        hasher.input(&[dirn as u8]);
        let mut next_path = path.clone();
        next_path.push(dirn);
        let subresult = dfid(&hasher, limit - 1,
                             (next_x, next_y), next_path, find_longest);
        match subresult {
            Stopped => {
                if !find_longest || !found_longest {
                    result = Stopped;
                }
            },
            Completed(new_path) => {
                if find_longest {
                    match result.clone() {
                        Completed(old_path) => {
                            if new_path.len() >= old_path.len() {
                                result = Completed(new_path);
                                found_longest = true;
                            }
                        },
                        _ => {
                            result = Completed(new_path);
                            found_longest = true;
                        }
                    }
                } else {
                    return Completed(new_path);
                }
            },
            Finished => ()
        };
    };
    result
}

pub fn main() {
    let (part, args) = aoc::get_part_args();
    assert!(args.len() == 1);
    let passcode = args[0].as_bytes();
    let mut hasher0 = crypto::md5::Md5::new();
    hasher0.input(passcode);
    if part == 2 {
        match dfid(&hasher0, 0, (0, 0), "".to_string(), true) {
            Completed(soln) => { println!("longest {}", soln.len()); },
            Finished => { println!("no solution exists"); },
            Stopped => { panic!("stopped in longest"); }
        }
        return;
    }
    for limit in 0..std::usize::MAX {
        let result = dfid(&hasher0, limit, (0, 0), "".to_string(), false);
        match  result {
            Completed(soln) =>  {
                println!("solution {}", soln);
                return;
            },
            Finished => {
                println!("no solution exists");
                return;
            },
            _ => ()
        };
    };
    print!("exploration too shallow");
}
