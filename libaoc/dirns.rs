// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! Directions iterator for Advent of Code 2016 solutions.
//!
//! To use this, make a new `ClipBox` to set the clipping bounds,
//! then call the `neighbors()` method of the `ClipBox` to get
//! an iterator over directions.
//!
//! # Examples
//!
//! ```rust
//! use aoc::dirns::*;
//! 
//! let clip_box = ClipBox::new(3, 4);
//! let neighbors = clip_box.neighbors(2, 0)
//!                 .collect::<Vec<_>>();
//! assert_eq!(neighbors, vec![(2, 1), (1, 0)]);
//! ```


/// The cardinal directions: up, down, left, right in
/// an x-y coordinate system where increasing y is down.
#[allow(dead_code)]
static DIRNS: [(isize, isize);4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

pub type Point = (usize, usize);

/// The dimensions of the playfield, for clippng.
#[allow(dead_code)]
pub struct ClipBox {
    bounds: Point
}

impl ClipBox {
    #[allow(dead_code)]
    pub fn new(x_size: usize, y_size: usize) -> ClipBox {
        ClipBox { bounds: (x_size, y_size) }
    }

    #[allow(dead_code)]
    pub fn neighbors(&self, x: usize, y: usize) -> Neighbors {
        assert!(x < self.bounds.0 && y < self.bounds.1);
        Neighbors::new(self.bounds, (x, y))
    }
}

#[allow(dead_code)]
pub struct Neighbors {
    bounds: Point,
    loc: Point,
    dirns: Box<Iterator<Item=&'static (isize, isize)>>
}

impl Neighbors {
    #[allow(dead_code)]
    pub fn new(bounds: Point, loc: Point) -> Self {
        Neighbors {
            bounds: bounds,
            loc: loc,
            dirns: Box::new(DIRNS.iter())
        }
    }
}

impl Iterator for Neighbors {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        loop {
            match self.dirns.next() {
                Some(&(dx, dy)) => {
                    let nx = self.loc.0 as isize + dx;
                    if nx < 0 || nx >= self.bounds.0 as isize {
                        continue;
                    };
                    let ny = self.loc.1 as isize + dy;
                    if ny < 0 || ny >= self.bounds.1 as isize {
                        continue;
                    };
                    return Some((nx as usize, ny as usize));
                },
                None => {
                    return None;
                }
            }
        }
    }
}

