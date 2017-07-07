use std::cmp::max;
use std::option::Option;
use std::default::Default;

use SEEDS;
use awari::Awari;
use storage::{Backend,Storage};


/// State storing the current computed best score for a given game
/// configuration.
#[derive(Serialize,Deserialize,Copy,Clone,Debug)]
pub enum State {
    Stable(i8),
    Unstable(i8, u8),
}


impl State {
    /// Get the current value of the board. This shouldn't be needed anymore.
    #[inline]
    pub fn value(&self) -> i8 {
        match *self {
            State::Stable(s) => s,
            State::Unstable(s, _) => s,
        }
    }

    /// Update the value of a board using the final value `up` from a successor.
    /// If it flipped the board to a stable state return the final value of the
    /// board, else return `None`.
    #[inline]
    pub fn update(&mut self, up: i8, sat_lvl: i8) -> Option<i8> {
        debug_assert!(sat_lvl >= max(self.value(), -up));

        match *self {
            State::Stable(s) => {
                // nothing to do, just test for consistency
                debug_assert!(s >= up);
                return Option::None;
            },
            State::Unstable(s, 1) => {
                let s = max(s, -up);
                *self = State::Stable(s);
                return Option::Some(s);
            },
            State::Unstable(s, n) if (s == sat_lvl || -up == sat_lvl) => {
                debug_assert!(n > 0);
                *self = State::Stable(sat_lvl);
                return Option::Some(sat_lvl);
            },
            State::Unstable(ref mut s, ref mut n) => {
                debug_assert!(*n > 0);
                *n -= 1;
                if *s < -up {
                    *s = -up;
                }
                return Option::None;
            },
        }
    }

    /// If the board has no more successor to wait for, flip it to stable and
    /// return the final value, else do nothing.
    #[inline]
    pub fn try_stabilize(&mut self, sat_lvl: i8) -> Option<i8> {
        match *self {
            State::Unstable(s, n) if s == sat_lvl || n == 0 => {
                *self = State::Stable(s);
                Option::Some(s)
            },
            _ => Option::None,
        }
    }
}

impl Default for State {
    fn default() -> Self { State::Unstable(-(SEEDS as i8), 0) }
}


/// Update the given state with the final score of one of its successors.
/// Propagate it recursively whenever it flips the state to a final score.
fn propagate<B: Backend<State>>(table: &mut Storage<State, B>, u: Awari,
                                up: i8, sat_lvl: i8) {
    let mut stack = vec![(u, up)];
    while let Some((u, a)) = stack.pop() {
        if let Some(b) = table.index_mut(u.encode()).update(a, sat_lvl) {
            // if update changed to final value, propagate further
            for v in u.predecessors() {
                stack.push((v, b));
            }
        }
    }
}


/// Construct the optimal score table for up to ``max_iter`` pieces on the
/// board.
pub fn analyze<B: Backend<State>>(max_iter: usize) -> Storage<State, B> {
    let mut table: Storage<State, B> = Default::default();
    //table.pre_row_hook(0);
    *table.index_mut(0) = State::Stable(0);
    
    for n in 1..max_iter+1 {
        println!("\n%%%%% seed num {} %%%%%", n);
        //table.pre_row_hook(n);

        // initialization
        for u in Awari::iter_config(n) {
            let (mut score, mut nsucc) = (-(n as i8), 0);
            for (v, k) in u.successors() {
                if k > 0 {
                    score = max(score, k as i8 - table.index(v.encode()).value());
                }
                nsucc += 1;
            }
            *table.index_mut(u.encode()) = State::Unstable(score, nsucc);
        }

        // convergence
        for l in 0..n+1 {
            info!("iteration {}", l);
            let sat_lvl = (n - l) as i8;
            for u in Awari::iter_config(n) {
                // yup, temporary lifetimes have struck again..
                if let Some(x) = { let ref mut tmp = table.index_mut(u.encode());
                                   tmp.try_stabilize(sat_lvl) } {
                    for v in u.predecessors() {
                        propagate(&mut table, v, x, sat_lvl);
                    }
                }
            }
        }
    }

    return table;
}
