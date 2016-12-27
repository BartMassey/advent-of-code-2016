# Advent of Code 2016: Day 9
Copyright (c) 2016 Bart Massey

A cute little string processing problem. Rust is kind of
terrible at this. I am slowly getting the hang of its string
mangling capabilities, but it still took me a long time to
write and is kinda ugly.

The hint in part 2 that you want to just count the output
rather than doing actual decompression is something I
realized in part 1 that made it a lot easier. It's arguable
that the hint should have been given in part 1.

The tests given with the problem were great, and helped spot
a couple of bugs. I wish the tests of part 2 had been
counted for me, though.

Part 2 requires 64-bit arithmetic.

Rust has no equivalent of `getchar()`! Let me say that
again, for emphasis: **Rust has no equivalent of
`getchar()`!!** There is no reasonable way to ask for the
next character in a text file. The best one can do is to
iterate over the chars of the text file. This isn't
horrible, except that it is **only in unstable.** Yes,
that's right: iterating over the chars of a textfile is an
unstable feature, because reasons. I don't think Rust is
really ready for primetime yet.

## Usage

* Part 1:

        cargo run 1 <input.txt

* Part 2:

        cargo run 2 <input.txt

---

This program is licensed under the "MIT License".
Please see the file COPYING in this distribution
for license terms.
