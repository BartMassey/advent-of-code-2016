# Advent of Code 2016: Day 8
Copyright (c) 2016 Bart Massey

Another problem very reminiscent of one from last year. I
think all of them are likely to be like that.

This was a hard problem for Rust, which always struggles
with 2D arrays and the like. Ultimately, I think I found a
reasonably elegant and extensible solution using function
pointers for the instructions.

## Usage

The array dimensions are passed as extra arguments. Use "7
3" for the test problem (with which I found a bunch of bugs).

* Part 1:

        cargo run 1 50 6 <input.txt

* Part 2:

        cargo run 2 50 6 <input.txt

---

This program is licensed under the "MIT License".
Please see the file COPYING in this distribution
for license terms.
