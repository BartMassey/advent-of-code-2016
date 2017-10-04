# Advent Of Code 2016: Tutorial Solutions in Rust
Copyright (c) 2016 Bart Massey

Herein lie solutions to all of the problems of the 2016
[Advent of Code](http://adventofcode.com). Advent of Code is
a fantastic exercise, and I thank the author and others
involved profusely for their excellent work. Thanks also to
`relsqui` for pointing me at this last year.

The solutions are in directories named `day01` through
`day25`. For each solution, I have included commented and
cleaned-up Rust code. There is a `README.md` in every
problem directory containing descriptions, comments and
usage instructions. I have also included the problem
descriptions (`part1.md` and `part2.md`) and my specific
`input.txt` for posterity.

There is also a `libaoc` directory containing a library used
by all solutions. It includes a mishmash of AoC-specific
stuff and generally-useful functionality. Please see the
`libaoc` rustdoc for the details.

There is a talk in `rust-aoc.p.md` that says some things
about all this.

There are not many tests written for this code. I regard
passing both parts of a day's problem as strong validation.
But more should be there.

I assume you have Rust running on a fast-ish UNIX box with a
bunch of memory (although most everything should also work
on other operating systems).  A few problems are dependent
on common extra packages from Cargo.

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
day 34 I would "`sh mkday.sh 34`". At the end of the day I
would select and copy the page source of the day 34 AoC page
and then

    xclip -selection CLIPBOARD -out | sh ../process-aoc.sh

to get markdown into the problem files for posterity.

To generate the full rustdoc for a day, simply run this
highly intuitive command:

    cargo rustdoc --open -- --no-defaults --passes collapse-docs \
      --passes unindent-comments --passes strip-priv-imports

See Issue
[1520](https://github.com/rust-lang/cargo/issues/1520)
for an explanation.

These solutions deserve a much more thorough top-level
description than I have the energy to write at this point.
I will revise this file in the indefinite future.

There is more refactoring that should be done. Specifically:

* The `try_pat()` function of days 21 and 22 should be
  pulled to `libaoc` and used in other parts as well.

I have run everything through
[Clippy](http://crates.io/crates/clippy) at this point, and
taken much of its advice. I have generally left idiomatic
loops alone, as the iterator versions suggested by Clippy
seem less clear to me. It was gratifying to see the last
three days' solutions pass Clippy without warning:
apparently I learned some Rust.

I am a novice Rust programmer. Feedback and pull requests
are extremely welcome! Let me know what I should have done,
and I'll try to make it right.


---

This work is licensed under the "MIT License".  Please see
the file `COPYING` in the source distribution of this software
for license terms.
