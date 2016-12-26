# Advent of Code 2016: Day 4
Copyright (c) 2016 Bart Massey

Another string-bashing exercise. Sigh. The good news is that
it was a great excuse to learn how to use Rust's standard
Regex package, which is pretty nice.

Part two is a bit weak, as it isn't clear there's any way in
advance to programmatically find the answer. I cheated and
just printed all the decryptions of valid room names and
grepped for "north". Knowing that this search works, I could
modify the Rust to do the search...but I won't.

## Usage

* Part 1:

        cargo run 1 <input.txt

* Part 2 (UNIX):

        cargo run 2 <input.txt | grep north | awk '{print $NF}'

---

This program is licensed under the "MIT License".
Please see the file COPYING in this distribution
for license terms.
