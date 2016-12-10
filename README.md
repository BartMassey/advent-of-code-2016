# Advent Of Code 2016: Tutorial Solutions in Rust
Copyright (c) 2016 Bart Massey

Herein lie solutions to some of the problems of the 2016
[Advent of Code](http://adventofcode.com). Advent of Code is
a fantastic exercise, and I thank the author and others
involved profusely for their excellent work. Thanks also to
`relsqui` for pointing me at this last year.

For each solution, I have included commented and cleaned-up
Rust code. The solution will be in a file named `soln.rs`,
which can be run as "./soln 1 <input.txt" or "./soln 2
<input.txt" to get the part 1 or part 2 solution. There is a
`README.md` in every problem directory containing
descriptions and comments. I have also included the problem
descriptions (`part1.md` and `part2.md`) and my specific
`input.txt` for posterity.

I assume you have Rust running on a fast-ish UNIX box with a
bunch of memory (although most everything should also work
on other operating systems).  For a few problems you will
also need to install extra packages using Cargo.

The goals of these solutions are to:

* Provide canonical correct solutions with reasonable
  runtimes.

* Illustrate reasonable solution strategies.

* Illustrate the use of Rust in problem-solving.

I learned a ton of Rust and a little bit of software
engineering I should already have known writing these.

There's also some engineering infrastructure in the form of
the `template` directory, the `mkday.sh` and
`process-aoc.sh` shell scripts and the `libaoc` crate.  This
sped up each day's setup considerably. At the beginning of
day 34 I would "sh mkday.sh 34". At the end of the day I
would select and copy the page source of the day 34 AoC page
and then "xclipb -out | sh ../process-aoc.sh" to get
markdown into the problem files for posterity.

These solutions deserve a much more thorough top-level
description than I have the energy to write at this point.
I will revise this file in the indefinite future.

I am a novice Rust programmer. Feedback and pull requests
are extremely welcome! Let me know what I should have done,
and I'll try to make it right.

This work is licensed under the "MIT License".  Please see
the file `COPYING` in the source distribution of this software
for license terms.

