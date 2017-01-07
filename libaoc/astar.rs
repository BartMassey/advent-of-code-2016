// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! A\* for Advent of Code 2016 solutions.
//!
//! This is a framework for running a standard [A\*
//! search][1].  To use it, implement the `SearchState`
//! trait for your search state and call `a_star()`. The
//! global state for the problem instance is passed
//! separately, and can be whatever the problem
//! requires. (For AoC 2016, it's usually some
//! representation of a maze.) Note that the memory use here
//! is non-trivial: there is a lot of cloning of states.
//!
//! The framework includes support for state labels, so that
//! the search can report paths rather than just distances.
//! Labels currently require the `Copy` trait, which is a
//! bit restrictive, but covers some obvious use cases.
//! Paths are saved with each state rather than being
//! reconstructed at the end, which is quite expensive of
//! memory. (This doesn't matter for AoC 2016, since none of
//! the A\* problems ask for a path.)
//!
//! [1]: http://en.wikipedia.org/wiki/A*_search_algorithm
//!
//! # Examples
//!
//! ```rust
//! struct Steps {
//!     steps: Vec<(usize, Box<Fn(isize) -> isize>)>,
//!     goal: isize
//! }
//!
//! #[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
//! struct State {
//!     value: isize
//! }
//!
//! impl aoc::SearchState for State {
//!     type Global = Steps;
//!     type Label = usize;
//!     fn label(&self) -> usize {
//!         self.value as usize
//!     }
//!     fn is_goal(&self, global: &Steps) -> bool {
//!         self.value == global.goal
//!     }
//!     fn neighbors(&self, global: &Steps) -> Vec<(usize, Box<State>)> {
//!         let mut result = Vec::new();
//!         for &(cost, ref step) in global.steps.iter() {
//!             let next_state = State{ value: step(self.value) };
//!             result.push((cost, Box::new(next_state)));
//!         };
//!         result
//!     }
//!     fn hcost(&self, global: &Steps) -> usize {
//!         (self.value - global.goal as isize).abs() as usize / 2
//!     }
//! }
//!
//! let steps = Steps {
//!     steps: vec![
//!         (2, Box::new(|n| { n + 1 })),
//!         (2, Box::new(|n| { n - 1 })),
//!         (3, Box::new(|n| { n * 2 }))],
//!     goal: 23
//! };
//! let start = State { value: 0 };
//! match aoc::a_star(&steps, &start, true) {
//!     Some((cost, path)) => {
//!         assert_eq!(cost, 17);
//!         assert_eq!(path, Some(vec![0, 1, 2, 3, 6, 12, 24, 23]));
//!     },
//!     None => { panic!("no solution") }
//! }
//! ```

use std::cmp::*;
use std::collections::{BTreeSet, BinaryHeap};

/// Node with state `S` in A\* search.
#[derive(Clone, Debug)]
struct PQElem<S:SearchState> {
    /// Cost so far.
    cost: usize,
    /// Total heuristic cost.
    fcost: usize,
    /// Actual state.
    state: S,
    /// Sequence of labels from start to current.
    path: Option<Vec<S::Label>>
}

impl <S: SearchState> PartialEq for PQElem<S> {
    /// From the search point of view, two nodes are
    /// the same if their heuristic cost and actual
    /// cost are both the same.
    fn eq(&self, other: &PQElem<S>) -> bool {
        other.fcost == self.fcost && other.cost == self.cost
    }
}

/// It would be nice to just derive this from `PartialEq`,
/// but Rust derive does the other thing.
impl <S: SearchState> Eq for PQElem<S> {}

/// It would be nice to just derive this from `Ord`,
/// but Rust derive does the other thing.
impl <S: SearchState> PartialOrd for PQElem<S> {
    fn partial_cmp(&self, other: &PQElem<S>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl <S: SearchState> Ord for PQElem<S> {
    /// A node is better than another if its heuristic
    /// cost is smaller. Ties are broken by preferring
    /// nodes with larger confirmed cost, since these
    /// are farther along the path to a solution.
    fn cmp(&self, other: &PQElem<S>) -> Ordering {
        match other.fcost.cmp(&self.fcost) {
            Ordering::Equal =>
                self.cost.cmp(&other.cost),
            c => c
        }
    }
}

/// Trait for nodes in a search space.
pub trait SearchState {
    /// Type of state label. The state label is used solely for
    /// tracking and returning the least-cost path.
    ///
    /// Because default types are unstable, it will be
    /// necessary to define this type even if no path
    /// tracking is needed.  The obvious implementation in
    /// this case is to set it to `()` and then have
    /// `SearchState::label()` return `()`.
    type Label: Copy;

    /// Type of global information used during the search.
    /// This information is passed in on start and passed to
    /// the `SearchGoals::hcost()` and
    /// `SearchState::neighbors()` methods in case it is
    /// needed. Use `()` if global data is not needed.
    type Global;

    /// Returns a label for this node as part of the path
    /// tracking.
    ///
    /// Because default types are unstable, it will be
    /// necessary to define this function even if no path
    /// tracking is needed.  The obvious implementation in
    /// this case is to set `SearchState::Label` to `()` and
    /// then have `label()` return `()`.
    fn label(&self) -> Self::Label;

    /// Return the neighbors of this state in the search
    /// space, each annotated with the cost of reaching
    /// it. May use the given global information to
    /// calculate its result.
    ///
    /// This should arguably return an iterator rather than
    /// a vector so that it can be lazy. However, the
    /// inconvenience of dealing with Rust iterators
    /// ultimately made this untenable.
    fn neighbors(&self, global: &Self::Global) -> Vec<(usize, Box<Self>)>;

    /// Return true if this is a goal state,
    /// given global information.
    fn is_goal(&self, global: &Self::Global) -> bool;
    
    /// Return an [admissible][1] heuristic cost of reaching
    /// the least-cost goal node from the given state. The default
    /// implementation causes A\* search (as provided by
    /// `astar::a_star()`) to degenerate to the special case
    /// of [Dijkstra's Algorithm][2].
    ///
    /// Arguments are the source state and the global
    /// information.
    ///
    /// [1]: http://en.wikipedia.org/wiki/Admissible_heuristic
    /// [2]: https://en.wikipedia.org/wiki/Dijkstra's_algorithm
    fn hcost(&self, _: &Self::Global) -> usize {
        0
    }
}

/// Generic [A\* search][1] for a least-cost path from the
/// given start state to some given goal, using
/// supplied global data. The return value is the cost
/// and path (sequence of states) if a path is found.
pub fn a_star<S>(global: &S::Global, start: &S, save_path: bool)
-> Option<(usize, Option<Vec<S::Label>>)>
where S: Clone + PartialEq + Eq + PartialOrd + Ord + SearchState {
    let mut stop_list = BTreeSet::new();
    let mut pq = BinaryHeap::new();
    pq.push(PQElem{
        state: start.clone(),
        cost: 0,
        fcost: start.hcost(global),
        path: if save_path { Some(Vec::new()) } else { None }
    });
    loop {
        match pq.pop() {
            Some(PQElem{cost, state, path, ..}) => {
                let next_path =
                    match path {
                        None => None,
                        Some(ref labels) => {
                            let mut p = labels.clone();
                            p.push(state.label());
                            Some(p)
                        }
                    };
                if state.is_goal(global) {
                    return Some((cost, next_path));
                };
                if stop_list.insert(state.clone()) {
                    for nb in state.neighbors(global) {
                        let (g_cost, ref next_state) = nb;
                        let h = next_state.hcost(global);
                        let g = cost + g_cost;
                        let neighbor = PQElem {
                            fcost: g + h,
                            cost: g,
                            state: (**next_state).clone(),
                            path: next_path.clone()
                        };
                        pq.push(neighbor);
                    }
                } else {
                    continue;
                };
            },
            None => {
                return None
            }
        }
    };
}
