#!/usr/bin/python3
# Copyright (c) 2016 Bart Massey
# This program is licensed under the "MIT License".
# Please see the file COPYING in this distribution
# for license terms.

# Advent of Code Day 15. Broken prototype in Python.

from math import *

# https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
def multinv(a, n):
    (t, new_t) = (0, 1)
    (r, new_r) = (n, a)
    while new_r != 0:
        q = r // new_r
        (t, new_t) = (new_t, t - q * new_t)
        (r, new_r) = (new_r, r - q * new_r)
    if r > 1:
        return None
    if t < 0:
        t += n
    return t

def lcm(x, y):
    return x * y // gcd(x, y)

test = None
if test == None:
    coeffs = [
        (7, 0),
        (13, 0),
        (3, 2),
        (5, 2),
        (17, 0),
        (19, 7) ]
elif test == 1:
    coeffs = [
        (5, 4),
        (2, 1) ]
elif test == 2:
    coeffs = [
        (5, 4),
        (2, 1),
        (3, 2) ]
elif test == 3:
    coeffs = [
        (5, 4),
        (2, 1),
        (17, 1),
        (3, 2),
        (11, 0) ]
else:
    assert(False)

c1 = coeffs[0][0]
q1 = coeffs[0][1]
t = None
for j in range(1, len(coeffs)):
    cj = coeffs[j][0]
    qj = coeffs[j][1]
    k1 = multinv(c1, cj) * (q1 - qj - 1)
    while k1 * c1 < q1 + j:
        k1 += cj
    t = k1 * c1 - q1 - j
    x = k1 * c1 + qj - q1 + 1
    assert(x % cj == 0)
    kj = x // cj
    q1 = c1 * cj - t
    c1 *= cj
print("math", t)

def search(t0):
    for j in range(len(coeffs)):
        cj = coeffs[j][0]
        qj = coeffs[j][1]
        t = t0 + 1 + j
        p = (t + qj) % cj
        if p != 0:
            return False
    return True

for ti in range(1000000):
    if search(ti):
        print("search", ti)
        break

