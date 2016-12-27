# Advent of Code 2016: Day 5
Copyright (c) 2016 Bart Massey

This problem is *very* similar to a problem from last
year. I cribbed much of the code from somebody's last-year
[solution](https://gist.github.com/gkbrk/2e4835e3a17b3fb6e1e7),
because life's too short to figure out how to use Rust's
most-popular `crypto` crate properly (although it seems
quite nice).

This sort of problem is always CPU-intensive.  When compiled
with "cargo build --release", part 1 ran in 1.7 seconds,
part 2 in 4.9 seconds on my 3.5 GHz i7-4770K (Haswell). When
compiled without "--release", performance was
untenable. Rust at its best is pretty competitive with C,
which is nice in this challenge.

Of course, the bonus for part 2 was necessary. So you get
some mild cinematics with this one.

## Usage

* Part 1:

        cargo run --release 1 <input.txt

* Part 2:

        cargo run --release 2 <input.txt

---

This program is licensed under the "MIT License".
Please see the file COPYING in this distribution
for license terms.
