// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! Set operations for Advent of Code 2016 solutions.
//!
//! These are specifically `std::collections::BTreeSet`
//! operations, because `std::collections::HashSet` is not
//! hashable (why?) and thus cannot be contained in a
//! `HashSet`.
//! 
//! It would nice to have a generic `std::collections::Set`
//! trait to parameterize the stuff that doesn't care.

use std::collections::BTreeSet;

/// Make a set from a slice.
pub fn make_set<T>(elems: &[T]) -> BTreeSet<T>
  where T: Clone + Ord {
      elems.iter().cloned().collect()
}
/*
    let mut s = BTreeSet::new();
    for v in elems {
        s.insert((*v).clone());
    };
    s
*/

/// Consruct the set of all choices of `n` items from a given
/// set of items.
pub fn choose<T>(source: &BTreeSet<T>, n: usize)
  -> BTreeSet<BTreeSet<T>> where T: Clone + Ord {
    if source.len() < n {
        let empty = BTreeSet::new();
        return empty;
    };
    let mut r = BTreeSet::new();
    if n == 0 {
        let empty = BTreeSet::new();
        r.insert(empty);
        return r;
    };
    let mut es = source.clone();
    for e in source.into_iter().cloned() {
        es.remove(&e);
        let cs = *Box::new(choose(&es, n-1));
        for mut c in cs {
            c.insert(e.clone());
            r.insert(c);
        }
    };
    r
}


/// Consruct the set of all choices of `0..n` items from a given
/// set of items.
pub fn choose_le<T>(source: &BTreeSet<T>, n: usize)
  -> BTreeSet<BTreeSet<T>> where T: Clone + Ord {
    let mut r = choose(source, 0);
    for i in 1..n+1 {
        let s = choose(source, i);
        r = r.union(&s).cloned().collect();
    };
    r
}


#[cfg(test)] mod tests {

    use std::collections::BTreeSet;
    use super::*;

    /// Make a set of sets from a slice of slices.
    fn make_set_set<T>(elems: &[&[T]]) -> BTreeSet<BTreeSet<T>>
    where T: Copy + Clone + Ord {
        // There should be an implementation of this
        // using `collect()`, but I can't find it.
        let mut s = BTreeSet::new();
        for v in elems {
            s.insert(make_set(v));
        };
        s
    }
    
    #[test] fn choose_zero() {
        let s = make_set(&[1, 2, 3]);
        let cs = choose(&s, 0);
        let mut xs = BTreeSet::new();
        xs.insert(BTreeSet::new());
        assert!(cs == xs);
    }

    #[test] fn choose_too_many() {
        let s = make_set(&[1, 2, 3]);
        let cs = choose(&s, 4);
        let xs = BTreeSet::new();
        assert!(cs == xs);
    }

    #[test] fn three_choose_one() {
        let s = make_set(&[1, 2, 3]);
        let cs = choose(&s, 1);
        let t = make_set_set(&[&[1], &[2], &[3]]);
        assert!(cs == t);
    }

    #[test] fn three_choose_two() {
        let s = make_set(&[1, 2, 3]);
        let cs = choose(&s, 2);
        let t = make_set_set(&[&[1, 2], &[2, 3], &[1, 3]]);
        assert!(cs == t);
    }

    #[test] fn four_choose_two() {
        let s = make_set(&[1, 2, 3, 4]);
        let cs = choose(&s, 2);
        let t = make_set_set(&[
            &[1, 2], &[1, 3], &[1, 4],
            &[2, 3], &[2, 4],
            &[3, 4]
        ]);
        assert!(cs == t);
    }

}
