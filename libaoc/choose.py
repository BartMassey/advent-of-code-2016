#!/usr/bin/python3
# Copyright © 2016 Bart Massey
# This program is licensed under the "MIT License".
# Please see the file COPYING in this distribution
# for license terms.

# Prototype version of Rust choose function.

# Return all sets of exactly n elements chosen from a set s.
# XXX Because Python, a set of frozensets is actually returned.
def choose(s, n):
    if n == 0:
        r = set()
        r.add(frozenset())
        return r
    r = set()
    t = set(s)
    for e in s:
        t.remove(e)
        for t1 in choose_n(t, n - 1):
            t2 = set(t1)
            t2.add(e)
            r.add(frozenset(t2))
    return r

c = choose_n(set(range(5)), 3)
print(len(c))
for s in c:
    print(set(s))
