// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! Advent of Code Day 14.

extern crate aoc;
extern crate md5;

use md5::{Digest, Md5};

/// "Stretched" hasher for part 2.
struct StretchedMd5(Md5);

impl StretchedMd5 {
    /// Just box up a plain old hasher.
    fn new() -> StretchedMd5 {
        Self(Md5::new())
    }

    fn reset(&mut self) {
        self.0.reset();
    }

    fn update(&mut self, bytes: &[u8]) {
        self.0.update(bytes);
    }

    /// Run the underlying MD5 hasher 2017 times to
    /// produce the stretched hash.
    fn finalize_into(self, hash: &mut [u8; 16]) {
        let mut out = self.0.finalize();
        for _ in 0..2016 {
            let mut h = Md5::new();
            let s = aoc::hex_string(out.as_slice());
            let out_hex = &(s.as_bytes());
            h.update(out_hex);
            h.finalize_into(&mut out);
        }
        hash.copy_from_slice(out.as_slice());
    }
}

/// Look for `reps` consecutive repetitions of a `target`
/// digit in a given `hash`. If `target` is `None`,
/// repetitions of any digit count.
fn has_repeat(hash: &[u8; 16], target: Option<u8>, reps: usize) -> Option<u8> {
    // Split up the input bytes into digits.
    let mut nybbles = [0u8; 32];
    #[allow(clippy::needless_range_loop)]
    for i in 0..16 {
        let j = i << 1;
        nybbles[j] = (hash[i] >> 4) & 0xf;
        nybbles[j + 1] = hash[i] & 0xf;
    }

    // Look for repetitions of target. O(*mn*) where
    // *m* is the size of the input and *n* is the
    // desired repetition count.
    for i in 0..32 - reps {
        // Look for repetitions at position *i*.
        // Implemented as a function to allow early return
        // for convenience.
        let has_reps = || {
            // Identify or select starting value.
            let b0 = match target {
                Some(b) => b,
                None => nybbles[i],
            };

            // Check for consecutive matches of value.
            for j in 0..reps {
                if nybbles[i + j] != b0 {
                    return false;
                }
            }
            true
        };

        // Wrap the answer.
        if has_reps() {
            return Some(nybbles[i]);
        };
    }
    None
}

#[cfg(test)]
mod test_has_repeat {

    use super::*;

    fn make_hash(i: usize, output: &mut [u8; 16]) {
        let mut hasher = Md5::new();
        hasher.update("abc".to_string().as_bytes());
        hasher.update(i.to_string().as_bytes());
        hasher.md_finalize_into(output);
    }

    fn has_rep(i: usize, t: Option<u8>, n: usize) -> Option<u8> {
        let mut output = [0u8; 16];
        make_hash(i, &mut output);
        super::has_repeat(&output, t, n)
    }

    #[test]
    fn check_reps() {
        assert!(has_rep(17, None, 3) == None);
        assert!(has_rep(18, None, 3) == Some(0x8));
        assert!(has_rep(19, Some(0x8), 3) == None);
        assert!(has_rep(38, None, 3) == None);
        assert!(has_rep(39, Some(0xe), 3) == Some(0xe));
        assert!(has_rep(816, Some(0xe), 5) == Some(0xe));
        assert!(has_rep(92, Some(0x9), 3) == Some(0x9));
        assert!(has_rep(200, Some(0x9), 5) == Some(0x9));
    }

    fn make_stretched_hash(i: usize, output: &mut [u8; 16]) {
        let mut hasher = super::StretchedMd5::new();
        hasher.update("abc".to_string().as_bytes());
        hasher.update(i.to_string().as_bytes());
        hasher.finalize_into(output);
    }

    fn has_rep_stretched(i: usize, t: Option<u8>, n: usize) -> Option<u8> {
        let mut output = [0u8; 16];
        make_stretched_hash(i, &mut output);
        super::has_repeat(&output, t, n)
    }

    #[test]
    fn check_stretched_md5() {
        let mut output = [0u8; 16];
        make_hash(0, &mut output);
        if aoc::hex_string(&output) != "577571be4de9dcce85a041ba0410f29f" {
            panic!("{}", aoc::hex_string(&output));
        };
        make_stretched_hash(0, &mut output);
        if aoc::hex_string(&output) != "a107ff634856bb300138cac6568c0f24" {
            panic!("{}", aoc::hex_string(&output));
        };
        assert!(has_rep_stretched(0, None, 3) == None);
    }
}

pub trait MiniDigest {
    fn md_new() -> Self;
    fn md_update(&mut self, bytes: &[u8]);
    fn md_reset(&mut self);
    fn md_finalize_into(self, hash: &mut [u8; 16]);
}

impl MiniDigest for Md5 {
    fn md_new() -> Self {
        Self::new()
    }
    fn md_update(&mut self, bytes: &[u8]) {
        self.update(bytes);
    }
    fn md_reset(&mut self) {
        self.reset()
    }
    fn md_finalize_into(self, hash: &mut [u8; 16]) {
        let result = self.finalize();
        hash.copy_from_slice(result.as_slice());
    }
}

impl MiniDigest for StretchedMd5 {
    fn md_new() -> Self {
        Self::new()
    }
    fn md_update(&mut self, bytes: &[u8]) {
        self.update(bytes);
    }
    fn md_reset(&mut self) {
        self.reset()
    }
    fn md_finalize_into(self, hash: &mut [u8; 16]) {
        self.finalize_into(hash);
    }
}

fn solve<T: MiniDigest>(k: &[u8]) {
    // Set up state.
    let mut n: isize = 64;
    let mut output = [0; 16];
    let mut hashbuf: Vec<[u8; 16]> = Vec::new();

    // This loop should never finish.
    for i in 0..std::usize::MAX {
        // If hash is not already cached, cache it.
        if i >= hashbuf.len() {
            let mut hasher = T::md_new();
            hasher.md_update(k);
            hasher.md_update(i.to_string().as_bytes());
            hasher.md_finalize_into(&mut output);
            hashbuf.push(output);
        } else {
            output = hashbuf[i];
        }

        // If we've found a repetition, look ahead up to 1001
        // steps for another of the same.
        if let Some(rep) = has_repeat(&output, None, 3) {
            let mut output = [0; 16];
            for j in 1..1001 {
                // If hash is not already cached, cache it.
                if i + j >= hashbuf.len() {
                    let mut hasher = T::md_new();
                    hasher.md_update(k);
                    hasher.md_update((i + j).to_string().as_bytes());
                    hasher.md_finalize_into(&mut output);
                    hashbuf.push(output);
                } else {
                    output = hashbuf[i + j];
                };

                // If we've found the second repeat, process it.
                if let Some(xrep) = has_repeat(&output, Some(rep), 5) {
                    assert!(xrep == rep);
                    n -= 1;
                    if n == 1 {
                        println!("{}", i);
                        return;
                    };
                    break;
                };
            }
        };
    }
    panic!("no solution found");
}

/// Run the problem.
pub fn main() {
    let (part, args) = aoc::get_part_args();
    assert!(args.len() == 1);
    let k = &(args[0].as_bytes());
    if part == 1 {
        solve::<Md5>(k);
    } else if part == 2 {
        solve::<StretchedMd5>(k);
    } else {
        panic!("unknown part");
    };
}
