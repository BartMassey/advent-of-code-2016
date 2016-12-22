# Advent of Code 2016: Day 22
Copyright (c) 2016 Bart Massey

Ok...

Part 1 was a pretty straightforward exercise using the
obvious quadratic algorithm.

For part 2, I was getting all set up and started by
transforming the board into a form that would be suitable
for an A\* search. This probably would have worked, but upon
printing the board I noticed that I could just count the
answer out by hand...

For my input, it took 26 steps to get the blank just to the
left of the goal data (dodging a wall), another 33 * 5 steps
to move the goal data left to just before the target, and
one more step to put it in place. Hooray really simple
sliding tile puzzle. I really should do the A\* search as
well, but for now I'll just declare victory.

---

This program is licensed under the "MIT License".
Please see the file COPYING in this distribution
for license terms.
