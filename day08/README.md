# Advent of Code 2016: Day 8
Copyright (c) 2016 Bart Massey

Another problem very reminiscent of one from last year. I
think all of them are likely to be like that.

Today's problem didn't work very well with my part1/part2
program format. The pieces were tightly integrated, and
besides I couldn't figure out how to get Rust's
`lazy_static` crate to do what I needed for regexps without
creating a separate crate or putting the macro include in
the same file with the main program. Huh. So there's a
single solution to both parts in soln.rs.

This was a hard problem for Rust, which always struggles
with 2D arrays and the like. Ultimately, I think I found a
reasonably elegant and extensible solution using function
pointers for the instructions.

The array dimensions are hardcoded in. I set it for 7x3 for
the test problem, found a bunch of bugs, and set it to 50x6
for the actual input.

---

This program is licensed under the "MIT License".
Please see the file COPYING in this distribution
for license terms.