# Advent of Code 2016: Day 22
Copyright (c) 2016 Bart Massey

Ok...

Part 1 was a pretty straightforward exercise using the
obvious quadratic algorithm.

I started getting set up for part 2 by transforming the
board into a form that would be suitable for an A\*
search. This probably would have worked, but upon printing
the board I noticed that I could just count the answer out
by hand...

For my input, it took 26 steps to get the blank just to the
left of the goal data (dodging a wall), another 33 * 5 steps
to move the goal data left to just before the target, and
one more step to put it in place. Hooray really simple
sliding tile puzzle.

I went back the next day and constructed the A\* search
anyway, just because. 27ms.

## Usage

Part 1:

        cargo run 1 <input.txt

Part 2:

        cargo run --release 2 <input.txt

---

This program is licensed under the "MIT License".
Please see the file COPYING in this distribution
for license terms.
