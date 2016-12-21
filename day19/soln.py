# Copyright © 2016 Bart Massey

# This program is licensed under the "MIT License".
# Please see the file COPYING in this distribution
# for license terms.

# Advent of Code Day 19 in Python.

from sys import argv

part = int(argv[1])
elves = [i + 1 for i in range(int(argv[2]))]
posn = 0

while len(elves) > 1:
    victim = None
    if part == 1:
        victim = (posn + 1) % len(elves)
    else:
        victim = (posn + len(elves) // 2) % len(elves)
    assert(posn != victim)
    print("%d: elf %d at position %d steals from elf %d at position %d" %
          (len(elves), elves[posn], posn + 1, elves[victim], victim + 1))
    del elves[victim]
    if posn != len(elves):
        posn += 1
    posn %= len(elves)

print(elves[posn])
