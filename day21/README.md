# Advent of Code 2016: Day 21
Copyright (c) 2016 Bart Massey

Huh. That took longer than I thought. This whole thing is
starting to feel a bit...grindy.

Anyhow, I eventually ended up: getting the supplied test to
pass; splitting instruction decode from instruction
execution; using an external crate for permutations.

It wasn't at all obvious to me what part 2 was asking for at
first. I thought it wanted me to literally reverse the
instructions; then I thought it wanted me to reverse the
encryption process by also reversing the operands. When I
realized that some of the instructions are non-invertible
(move, rotate by position) I finally figured out what was
up.

Instantaneous for part1, of course. 30ms for part 2.

---

This program is licensed under the "MIT License".
Please see the file COPYING in this distribution
for license terms.
