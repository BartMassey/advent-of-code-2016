# Advent of Code 2016: Day 14
Copyright (c) 2016 Bart Massey

OK, part 1 of this was reasonably straightforward. I started
with a bunch of the day 5 code and wrote the obvious
loop. It got the right answer in a few seconds.

Upon looking at part 2, it was clear that the solution was
not fast enough. The first obvious thing was to build a
cache of hashes to take care of the lookahead. This got me
to 0.1s. Great.

I opted for an "elegant" solution to part 2, using a wrapped
`StretchedMd5` class implementing `crypto::Digest`. This was
*awfully* hard to get right, because the compiler accepted
the code, but it wasn't doing the right thing with the
wrapped hasher: it was mutating a copy rather than the
original. Some unit tests and careful thought later I
corrected the implementation by boxing the hasher. The final
solution then came in at 18s, which seems surprisingly
reasonable, considering.

Took a lot more debugging time than I really had.
Was still three days behind at this point.

## Usage

* Part 1:

        cargo run --release 1 ngcjuoqr

* Part 2:

        cargo run --release 2 ngcjuoqr

---

This program is licensed under the "MIT License".
Please see the file COPYING in this distribution
for license terms.
