# Advent Of Code 2016: Tutorial Solutions in Rust
Copyright (c) 2016 Bart Massey

Herein lie solutions to some of the problems of the 2016
[Advent of Code](http://adventofcode.com). Advent of Code is
a fantastic exercise, and I thank the author and others
involved profusely for their excellent work. Thanks also to
`relsqui` for pointing me at this last year.

For each solution, I have included commented and cleaned-up
Rust code. The solution files will always be `part1.rs` and
`part2.rs`, which will normally refer to a `soln.rs`
library in the same directory. There is a `README.md`
in every problem directory containing descriptions and
comments. I have also included the problem descriptions
(`part1.md` and `part2.md`) and my specific
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
the `template` directory and the `mkday.sh` shell script.
This sped up each day's setup considerably.

These solutions deserve a much more thorough top-level
description than I have the energy to write at this point.
I will revise this file in the indefinite future.

I am a novice Rust programmer. Feedback and pull requests
are extremely welcome! Let me know what I should have done,
and I'll try to make it right.

This work is licensed under the "MIT License".  Please see
the file `COPYING` in the source distribution of this software
for license terms.

