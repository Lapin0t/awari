use std::cmp::max;
use std::option::Option;

use awari::Awari;


#[derive(Serialize,Deserialize,Copy,Clone,Debug)]
pub enum State {
    Stable(i8),
    Unstable(i8, u8),
}


impl State {
    /// Get the current value of the board. This shouldn't be needed anymore.
    pub fn value(&self) -> i8 {
        match *self {
            State::Stable(s) => s,
            State::Unstable(s, _) => s,
        }
    }

    /// Update the value of a board using the final value `up` from a successor.
    /// If it flipped the board to a stable state return the final value of the
    /// board, else return `None`.
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
    pub fn try_stabilize(&mut self) -> Option<i8> {
        match *self {
            State::Unstable(s, 0) => {
                *self = State::Stable(s);
                Option::Some(s)
            },
            _ => Option::None,
        }
    }
}


/// An abstraction over different storage models for the database (RAM, disk,
/// hybrid, ...).
pub trait Storage  {
    /// Create a new storage handle.
    fn new() -> Self;

    /// This hook can be used to implement some specific logic when switching
    /// to a new iteration of the RA
    fn pre_row_hook(&mut self, usize);

    /// Initialize a record; this method should be called before any
    /// subsequent one on a particular index.
    fn set(&mut self, usize, State);

    /// See `State::update`.
    fn update(&mut self, usize, i8, i8) -> Option<i8>;

    /// See `State::try_stabilize`.
    fn try_stabilize(&mut self, usize) -> Option<i8>;

    /// See `State::value`.
    fn value(&self, usize) -> i8;
}


fn propagate<T: Storage>(table: &mut T, u: Awari, up: i8, sat_lvl: i8) {
    let mut stack = vec![(u, up)];
    while let Some((u, a)) = stack.pop() {
        if let Some(b) = table.update(u.encode(), a, sat_lvl) {
            // if update changed to final value, propagate further
            for v in u.predecessors() {
                stack.push((v, b));
            }
        }
    }
}


pub fn analyze<T: Storage>(max_iter: usize) -> T {
    let mut table = T::new();
    table.pre_row_hook(0);
    table.set(0, State::Stable(0));
    
    for n in 1..max_iter+1 {
        println!("\n%%%%% seed num {} %%%%%", n);
        table.pre_row_hook(n);

        // initialization
        for u in Awari::iter_config(n) {
            let (mut score, mut nsucc) = (-(n as i8), 0);
            for (v, k) in u.successors() {
                if k > 0 {
                    score = max(score, k as i8 - table.value(v.encode()));
                }
                nsucc += 1;
            }
            table.set(u.encode(), State::Unstable(score, nsucc));
        }

        // convergence
        for l in 0..n+1 {
            info!("iteration {}", l);
            let sat_lvl = (n - l) as i8;
            for u in Awari::iter_config(n) {
                if let Some(x) = table.try_stabilize(u.encode()) {
                    for v in u.predecessors() {
                        propagate(&mut table, v, x, sat_lvl);
                    }
                }
            }
        }
    }
    
    return table;
}
