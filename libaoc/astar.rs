// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

//! A\* for Advent of Code 2016 solutions.
//!
//! Note that the memory use here is non-trivial.  There is
//! a lot of cloning of states. Also, paths are saved
//! per-state rather than being reconstructed at the end,
//! which is quite expensive.

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
    fn eq(&self, other: &PQElem<S>) -> bool {
        other.fcost == self.fcost && other.cost == self.cost
    }
}

impl <S: SearchState> Eq for PQElem<S> {}

impl <S: SearchState> PartialOrd for PQElem<S> {
    fn partial_cmp(&self, other: &PQElem<S>) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl <S: SearchState> Ord for PQElem<S> {
    /// Break ties in ordering by smallest heuristic cost
    /// using largest actual cost.
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

    /// Returns a label for this node as part of the path tracking.
    ///
    /// Because default types are unstable, it will be
    /// necessary to define this function even if no path
    /// tracking is needed.  The obvious implementation in
    /// this case is to set `SearchState::Label` to `()` and
    /// then have `label()` return `()`.
    fn label(&self) -> Self::Label;

    /// Return an iterator that delivers neighbors of this
    /// state in the search space, each annotated with the
    /// cost of reaching it. May use the given global
    /// information to calculate its result.
    fn neighbors(&self, global: &Self::Global)
    -> Box<Iterator<Item=(usize, Box<Self>)>>;

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
        fcost: start.hcost(&global),
        path: if save_path { Some(Vec::new()) } else { None }
    });
    loop {
        match pq.pop() {
            Some(PQElem{cost, fcost: _, state, path}) => {
                if state.is_goal(&global) {
                    return Some((cost, path));
                };
                match stop_list.insert(state.clone()) {
                    false => { continue; },
                    true => {
                        for nb in state.neighbors(&global) {
                            let (g_cost, ref next_state) = nb;
                            let h = next_state.hcost(&global);
                            let g = cost + g_cost;
                            let next_path =
                                match path {
                                    None => None,
                                    Some(ref labels) => {
                                        let mut p = labels.clone();
                                        p.push(state.label());
                                        Some(p)
                                    }
                                };
                            let neighbor = PQElem {
                                fcost: g + h,
                                cost: g,
                                state: (**next_state).clone(),
                                path: next_path
                            };
                            pq.push(neighbor);
                        }
                    }
                };
            },
            None => {
                return None
            }
        }
    };
}
