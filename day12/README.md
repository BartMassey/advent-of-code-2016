# Advent of Code 2016: Day 12
Copyright (c) 2016 Bart Massey

A straightforward and clean answer after "yesterday's"
fiasco. Now I'm only three days behind.

Note that while the specification doesn't make it at all
clear, the `JNZ` instruction had a constant first operand in
my input. So it can apparently act like an unconditional
jump (in my case) or a `NOP`.

Rust's performance was helpful here: 40ms for part 2.

Note that this machine was copy-pasted, cleaned up and
generalized for later problems.

## Usage

* Part 1:

        cargo run <input1.txt

* Part 2:

        cargo run --release <input2.txt

---

This program is licensed under the "MIT License".
Please see the file COPYING in this distribution
for license terms.
