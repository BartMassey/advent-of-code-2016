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
encryption process by also reversing the operands. That
probably would have worked, but rotate by position is a pain
to invert. I finally figured out what was up, and just
brute-forced it with all 40K permutations.

Instantaneous for part1, of course. 30ms for part 2.

## Usage

Part 1:

        cargo run 1 abcdefgh <input.txt

Part 2:

        cargo run 2 fbgdceah <input.txt

---

This program is licensed under the "MIT License".
Please see the file COPYING in this distribution
for license terms.
