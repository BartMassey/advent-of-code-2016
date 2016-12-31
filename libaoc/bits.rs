// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! Bit operations for Advent of Code 2016 solutions.

/// This popcount uses ivide-and-conquer with a quaternary
/// stage to reduce masking and provide mostly power-of-two
/// shifts. See <http://github.com/BartMassey/popcount> for
/// details.  This is "popcount_4" there, modified for
/// 64-bit.  This code will be slower than it needs to be on
/// narrower inputs.
///
/// # Examples
///
/// ```rust
/// assert_eq!(aoc::popcount(0x5555u64 << 32), 8);
/// ```

#[inline]
pub fn popcount<T: Into<u64>>(x0: T) -> usize {
    let mut x: u64 = x0.into();
    let m1 = 0x5555555555555555;
    let m2 = 0x0303030303030303;
    x -= (x >> 1) & m1;
    x = (x & m2) + ((x >> 2) & m2) + ((x >> 4) & m2) + ((x >> 6) & m2);
    x += x >> 8;
    x += x >> 16;
    ((x + (x >> 32)) & 0xff) as usize
}

#[cfg(test)]
mod test {

    extern crate rand;

    use self::rand::{Rand, SeedableRng, XorShiftRng};
    use super::popcount;

    /// The obvious naive implementation of popcount.
    fn popcount_naive(mut x: u64) -> usize {
        let mut count = 0;
        for _ in 0..64 {
            count += (x & 1) as usize;
            x >>= 1;
        };
        count
    }

    #[test]
    fn test_popcount_basics() {
        assert!(popcount(0u64) == 0);
        assert!(popcount(1u64) == 1);
        assert!(popcount(0xffffffffffffffffu64) == 64);
        assert!(popcount(0x5555555555555555u64) == 32);
        assert!(popcount(0xaaaaaaaaaaaaaaaau64) == 32);
        assert!(popcount(0x8000000000000000u64) == 1);
    }

    #[test]
    fn test_popcount_random() {
        let mut rng = XorShiftRng::from_seed([0xaa, 0xbb, 0xcc, 0xdd]);
        let mut x: u64;
        for _ in 0..1000 {
            x = Rand::rand(&mut rng);
            let p = popcount(x);
            let pn = popcount_naive(x);
            if p != pn {
                panic!(format!("popcount mismatch: x={}, p={}, px={}",
                               x, p, pn));
            };
        };
    }
}
