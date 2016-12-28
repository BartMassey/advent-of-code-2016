// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! Directions iterator for Advent of Code 2016 solutions.
//!
//! To use this, make a new `GridBox` to set clipping bounds,
//! then call the `neighbors()` method of the `ClipBox` to get
//! an iterator over clipped neighbors in cardinal directions.
//!
//! # Examples
//!
//! ```rust
//! use aoc::dirns::*;
//! 
//! let clip_box = GridBox::new(3, 4);
//! let neighbors = clip_box.neighbors((2, 0))
//!                 .collect::<Vec<_>>();
//! assert_eq!(neighbors, vec![(2, 1), (1, 0)]);
//! ```


/// The cardinal directions: up, down, left, right in
/// an x-y coordinate system where increasing y is down.
pub static DIRNS: [(isize, isize);4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

/// Type of unsigned coordinates.
pub type Point = (usize, usize);

/// Description of the grid, for possible clipping.
#[derive(Copy, Clone)]
pub enum GridBox {
    /// Grid is clipped on bottom and right.
    ClipBox(Point),
    /// Grid is unclipped.
    Unclipped
}

use self::GridBox::*;

impl GridBox {

    /// Create a clip box for neighbor calculations.
    #[allow(dead_code)]
    pub fn new(x_size: usize, y_size: usize) -> GridBox {
        ClipBox((x_size, y_size))
    }

    /// Create an "unbounded clip box" for neighbor calculations.
    pub fn new_grid() -> GridBox {
        Unclipped
    }

    /// Return an iterator that will produce the neighbors
    /// of the given location, clipped as needed.
    pub fn neighbors(&self, location: Point) -> Neighbors {
        if let &ClipBox((x_size, y_size)) = self {
            let (x, y) = location;
            assert!(x < x_size && y < y_size);
        };
        Neighbors::new(*self, location)
    }
}

/// Iterator over the neighbors of a point in the four cardinal
/// directions, clipped as appropriate.
pub struct Neighbors {
    /// Possible upper bounds on neighbor location.
    bounds: GridBox,
    /// Source location.
    loc: Point,
    /// Iterator for cardinal directions.
    dirns: Box<Iterator<Item=&'static (isize, isize)>>
}

impl Neighbors {

    /// Return an iterator over the neighbors of
    /// the given grid box starting at the given location.
    pub fn new(grid_box: GridBox, location: Point) -> Self {
        Neighbors {
            bounds: grid_box,
            loc: location,
            dirns: Box::new(DIRNS.iter())
        }
    }
}

impl Iterator for Neighbors {
    type Item = Point;

    /// Return the next cardinal neighbor of the source point,
    /// clipped as needed.
    fn next(&mut self) -> Option<Point> {
        loop {
            match self.dirns.next() {
                Some(&(dx, dy)) => {
                    let nx = self.loc.0 as isize + dx;
                    let ny = self.loc.1 as isize + dy;
                    if nx < 0 || ny < 0 {
                        continue;
                    };
                    if let ClipBox((x_size, y_size)) = self.bounds {
                        if nx >= x_size as isize || ny >= y_size as isize {
                            continue;
                        }
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

/// The ["Manhattan Distance"][1] between two points.
///
/// [1]: http://en.wikipedia.org/wiki/Taxicab_geometry
pub fn manhattan_distance(p1: Point, p2: Point) -> usize {
    let dx = (p1.0 as isize - p2.0 as isize).abs();
    let dy = (p1.1 as isize - p2.1 as isize).abs();
    (dx + dy) as usize
}