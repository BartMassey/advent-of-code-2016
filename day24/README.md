# Advent of Code 2016: Day 24
Copyright (c) 2016 Bart Massey

This problem is amenable to straight Dijkstra search. I
spent more time than I should have messing with it and
placed just below 100 on the leaderboard on both parts,
which is disappointing since it was probably my last chance
to get on the leaderboard.

There is probably a reasonable admissible heuristic to be
found for A\*, but at 0.33s for part 1 and 0.65s for part 2
it hardly seems worth the considerable hassle of figuring it
out.

## Usage

Part 1:

        cargo run --release 1 <input.txt

Part 2:

        cargo run --release 2 <input.txt

---

This program is licensed under the "MIT License".
Please see the file COPYING in this distribution
for license terms.
