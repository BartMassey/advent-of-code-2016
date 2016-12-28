// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! Library for Advent of Code 2016 solutions.
//!
//! This library contains support routines for the Advent of
//! Code 2016 Rust solutions. An attempt has been made to put
//! common code here rather than copy-pasting. In addition,
//! material of possible general use has been put here rather
//! than burying it in a solution.

// For an explanation of the structure of this file, see
// http://stackoverflow.com/questions/22596920/

mod args;
pub use self::args::*;

mod sets;
pub use self::sets::*;

mod bits;
pub use self::bits::*;

mod lines;
pub use self::lines::*;

mod hex;
pub use self::hex::*;

mod astar;
pub use self::astar::*;
