# Advent of Code 2016: Day 19
Copyright (c) 2016 Bart Massey

Damn this problem. Specifically, damn part 2 of this
problem. Seriously. This is the kind of problem that just
sucks all the fun out of this activity. I almost quit Advent
of Code over it for good.

Rust lacks a standard sequence type with efficient interior
deletion. OK. More problematic is that its
[`LinkedList`](http://doc.rust-lang.org/std/collections/struct.LinkedList.html)
type also does not do efficient interior deletion (splitting
a list is O(*n*) because memory management). OK. I'm a
grownup. I implemented part 1 using a `Vec` with linked
indices. This worked fine for part 2, as well, except the
obvious plan for calculating the next victim each time
incurred an O(*n^2*) running time, which is not ok.

Failing to see an easy fix, I decided to write my own `Seq`
type using a binary tree. I eventually found out why Rust
doesn't already have one of these: it would require quite a
bit of unsafe code. So I abandoned that plan.

Next, I decided to just give up and write it in
Haskell. Within an hour, I had a reasonably performant
solution using
[`Data.Sequence`](http://hackage.haskell.org/package/containers-0.5.9.1/docs/Data-Sequence.html)
that gave the correct answer on part 1.  It also ran in a
couple of seconds for part 2, but gave a wrong answer. Many
hours of debugging later, I still don't know why.

I built a Python prototype to compare with the Haskell. I
got it giving the same right and wrong answers for the same
reasons. The only things it taught me are that (a) it wasn't
just a failure to use Haskell correctly, and (b) Python is
indeed way too slow for a problem like this.

I finally gave up for a second time, and went back to
Reddit's [AoC Forum](http://www.reddit.com/r/adventofcode/)
to get help. Cheating, but I was done.
User
[/u/__Abigail__](https://www.reddit.com/user/__Abigail__)
[pointed out](https://www.reddit.com/r/adventofcode/comments/5j8e97/2016_day_19_part_2c_feels_like_im_waiting_for/dbe7hgs/)
that you can just keep a victim pointer and advance it at
half speed. Thus enlightened, I spent a couple of hours
chasing parity bugs in the initial setup and the pointer
advancement and hey presto, a working solution in Rust.

I would estimate I've spent close to 20 hours on this
problem in the past two days. I'm not sure I can take
another one of these. The Rust solution is ugly and just
kind of sad. But it's fast. Rust runs both parts in 80ms.

Haskell runs on both parts in 1.4s, producing a wrong answer
for part 2.  The Python solution scales superlinearly even
when run with PyPy and is untenable for the size of input
demanded.

I'm quite sure there's an O(1) solution here using nothing
but mathematics. But life is too short. I'm moving on.

## Usage

Part 1:

        cargo run 1 3018458

Part 2:

        cargo run 2 3018458

---

This program is licensed under the "MIT License".
Please see the file COPYING in this distribution
for license terms.
