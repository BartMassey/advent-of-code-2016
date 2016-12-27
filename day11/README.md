# Advent of Code 2016: Day 11
Copyright (c) 2016 Bart Massey

So here it is...the first of the *problem problems*. I'm
going to estimate I put 20 effort-hours into this one.

First of all, the spec for this one is quite fiddly. The
details of what constitutes a legal move / state are really
hard to follow. I ended up co-developing my understanding of
the problem along with the solution, which is never a great
idea. The worked example for this problem was absolutely
necessary: without it I never would have got the code right.

Secondly, doing an algorithm of any complexity with Rust is
hard anyway. This was my first really serious tangle with
the borrow checker: I got to clarify all the places where
`clone()`, copying, constructors and refs could make Rust OK
with what I was doing.

Sets in Rust are the usual mess, or maybe a little more
so. The two set types in `std::collections` are `HashSet`
and `BTreeSet`. I started using the former, but at some
point I needed a set of sets (gasp!) and unfortunately
`HashSets` will not themselves hash. Why? I'm guessing
because ordering: implementors haven't realized that
you can just exclusive-or the element hashes together. So
`BTreeSet` it was.

I wrote my usual algorithm for choosing all subsets of size
less than or equal to *n* from a given set. It was quite
spectacularly challenging to get right. Along the way I
wrote a Python prototype to try to clarify the situation,
and it was hard to get right too because Python sets are
also not hashable. Python provides a `frozenset` type for
this, but bleah.

Finally, the problem is combinatorial and requires an
efficient solution. I'm guessing there's some pruning trick
I was supposed to find, but as things stand at the end of
part 1 I was praying that part 2 wouldn't require more
CPU. Of course it did. They made me implement A\*. A\*!
Also, the code had to be compiled with optimizations
(`--release`) or it was hopeless. Anyway, after fixing a
couple of lame bugs in the heuristic, A\* turned out to be a
big win as expected.

By this point I had implemented the example as an inline
Rust test. That was a lot of work, but was really nice for
telling me that I had a reasonable understanding of the
rules (found a bunch of bugs that way) and that my code
would run.

Using Dijkstra's algorithm, part 1 finished in 54s on my box
and part 2 was still running after 20m. With A\*, part 1
took 0.3s and part 2 took 82s. That latter number is longer
than I'd normally like, but I'll take it.

## Usage

* Part 1:

        cargo run --release <input1.txt

* Part 2:

        cargo run --release <input2.txt

---

This program is licensed under the "MIT License".
Please see the file COPYING in this distribution
for license terms.
