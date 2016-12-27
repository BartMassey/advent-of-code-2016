# Advent of Code 2016: Day 15
Copyright (c) 2016 Bart Massey

So...

I looked at this problem and said to myself "Oh! A nice
modular arithmetic puzzle. I will construct a solution
linear in the number of discs!"

This was the wrong way to solve this, and cost me an entire
day. The solution idea was to repeatedly replace the top two
discs with a single equivalent disc until only one disc
remained. It can probably work. I wasn't good enough to do
the math.

Fortunately, the brute-force search I eventually built runs
instantly on part 1 and in 35ms on part 2.

See the prototype `soln.py` and the LaTeX document in the
`math-docs` directory for my attempt at something clever.

I think I'm going to cry.

## Usage

Part 1:

        cargo run --release 1


Part 2:

        cargo run --release 2

---

This program is licensed under the "MIT License".
Please see the file COPYING in this distribution
for license terms.
