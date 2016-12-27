# Advent of Code 2016: Day 10
Copyright (c) 2016 Bart Massey

The first algorithmically interesting problem. Also a giant
mess for Rust. 

The problem specification is pretty vague about what to
expect in the input. Is it possible that a bot could be
asked to compare more than two kinds of chip? Is it possible
that an output could receive more than one kind of chip?
Are there possible loops in processing so that a chip could
go in a circle forever never getting processed (presumably
not)?

I didn't like the way the instances were specialized. In the
first one, I felt like the instance-specific target
comparison values should be part of the instance
description. In the second one, it would be nice to require
some function of all the outputs.

I am not proud of my code for this problem. It is
copy-pastey, way too wide and non-modular, and pretty
kludgey in general.  The borrow checker and the weird
iterators and stuff made me do *things* I will regret for a
long time.

I actually ended up giving up for the night and finishing in
the morning: probably four hours' total effort. It felt like
Rust was 70% of the time sink here.

## Usage

Run part 1 with arguments "2 5" for the test instance.

* Part 1:

        cargo run 1 17 61 <input.txt

* Part 2:

        cargo run 2 <input.txt

---

This program is licensed under the "MIT License".
Please see the file COPYING in this distribution
for license terms.
