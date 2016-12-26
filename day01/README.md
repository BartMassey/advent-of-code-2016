# Advent of Code 2016: Day 1
Copyright © 2016 Bart Massey

This problem is pretty clearly intended as a warmup, and
it's a nice one: gets us used to the setup and whatnot.

Parsing this problem's text in Rust is...less bad than I
expected. So that's nice.

The part 2 solution makes good use of Rust's `HashSet` for
tracing a path. I was confused by the problem description
the first time: did not realize that "location" included all
the steps along the path and not just its endpoints.

## Usage

* Part 1:

        cargo run 1 <input.txt

* Part 2:

        cargo run 2 <input.txt

---

This program is licensed under the "MIT License".
Please see the file COPYING in this distribution
for license terms.
