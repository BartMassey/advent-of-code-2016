// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! Advent of Code Day 17.

extern crate md5;
use md5::{Md5, Digest};

extern crate aoc;

/// State of exploration.
#[derive(Clone)]
enum Explo {
    /// Finished without finding a path.
    Finished,
    /// Stopped early.
    Stopped,
    /// Finished with the given path.
    Completed(String),
}

use self::Explo::*;

/// Store true in `doors` for each of up, down, left, right
/// iff the given hasher shows that door open.
fn open_doors(hasher0: &Md5, doors: &mut [bool; 4]) {
    // Run a copy of the hasher (to terminate here).
    let hasher = hasher0.clone();
    let output = hasher.finalize();

    // Match up nybbles of the hash with doors.
    let nybbles = [
        (output[0] >> 4) & 0xf,
        output[0] & 0xf,
        (output[1] >> 4) & 0xf,
        output[1] & 0xf,
    ];
    for d in 0..4 {
        if nybbles[d] >= 11 {
            doors[d] = true;
        } else {
            doors[d] = false;
        }
    }
}

#[test]
fn test_open_doors() {
    let mut hasher = Md5::new();
    hasher.update("hijkl".as_bytes());
    let mut doors = [false; 4];
    open_doors(&hasher, &mut doors);
    assert!(doors == [true, true, true, false]);
}

/// Depth-First search for a path to a given location. The
/// goal position is `posn`, the path to this point is
/// `path`. Search will be for a longest path if
/// `limit` is `None`: otherwise it will be for the
/// first-found path of depth `limit` or less.
fn dfs(
    grid_box: &aoc::GridBox,
    hasher0: &Md5,
    limit: Option<usize>,
    posn: aoc::Point,
    path: String,
) -> Explo {
    // Stop at the goal.
    if posn == (3, 3) {
        return Completed(path);
    };

    // Stop if depth limited.
    if limit == Some(0) {
        return Stopped;
    }
    let find_longest = limit.is_none();

    // Set up the state and check the doors.
    let dirns = [('U', (0, -1)), ('D', (0, 1)), ('L', (-1, 0)), ('R', (1, 0))];
    let mut doors = [false; 4];
    open_doors(hasher0, &mut doors);
    let mut result = Finished;
    let mut found_longest = false;

    // Traverse each open door recursively.
    for door in 0..dirns.len() {
        // Get info on the next door.
        let (dirn, off) = dirns[door];

        // If the door is closed, give up.
        if !doors[door] {
            continue;
        };

        // Look at the next position.
        match grid_box.clip(posn, off) {
            None => {
                continue;
            }
            Some(next_loc) => {
                // Call recursively to explore continuation in this
                // direction.
                let mut hasher = hasher0.clone();
                hasher.update(&[dirn as u8]);
                let mut next_path = path.clone();
                next_path.push(dirn);
                let subresult = dfs(
                    grid_box,
                    &hasher,
                    limit.map(|l| l - 1),
                    next_loc,
                    next_path,
                );

                // Combine the subsearch result with the existing
                // status to get an updated status.
                match subresult {
                    // Only shortest-path search is depth-limited.
                    Stopped => {
                        if !find_longest || !found_longest {
                            result = Stopped;
                        }
                    }

                    // Fit the new path in with the old.
                    Completed(new_path) => {
                        if find_longest {
                            match result.clone() {
                                // Use longer path for find longest.
                                Completed(old_path) => {
                                    if new_path.len() >= old_path.len() {
                                        result = Completed(new_path);
                                        found_longest = true;
                                    }
                                }

                                // Found first path.
                                _ => {
                                    result = Completed(new_path);
                                    found_longest = true;
                                }
                            }
                        } else {
                            // If find-longest is false, return
                            // first path found.
                            return Completed(new_path);
                        }
                    }

                    // Nothing to do with a closed-off subsearch.
                    Finished => (),
                };
            }
        }
    }
    result
}

/// Search for a solution.
pub fn main() {
    let (part, args) = aoc::get_part_args();
    assert!(args.len() == 1);
    let passcode = args[0].as_bytes();

    // Set up state.
    let mut hasher0 = Md5::new();
    hasher0.update(passcode);
    let grid_box = aoc::GridBox::new(4, 4);

    // For part 2, do a single search for longest path.
    if part == 2 {
        let result = dfs(&grid_box, &hasher0, None, (0, 0), "".to_string());
        match result {
            Completed(soln) => {
                println!("{}", soln.len());
            }
            Finished => {
                println!("no solution exists");
            }
            Stopped => {
                panic!("stopped in longest");
            }
        }
        return;
    };

    // For part 1, use Depth-First Iterative Deepening to
    // find a shortest path.
    assert!(part == 1);
    for limit in 0..std::usize::MAX {
        let result = dfs(&grid_box, &hasher0, Some(limit), (0, 0), "".to_string());
        match result {
            Completed(soln) => {
                println!("{}", soln);
                return;
            }
            Finished => {
                println!("no solution exists");
                return;
            }
            _ => (),
        };
    }
    panic!("exploration too shallow");
}
