use std::cmp::max;
use std::option::Option;
use std::default::Default;
use std::mem::transmute;
use std::convert::AsRef;
use std::path::Path;

use SEEDS;
use awari::Awari;

pub use storage::*;


/// State storing the current computed best score for a given game
/// configuration.
#[derive(Copy,Clone,Debug)]
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
        debug_assert!(
            match *self {
                State::Unstable(v, _) => sat_lvl >= max(v, -up),
                _ => true
            }
        );

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

    #[inline]
    pub fn serialize(&self, buf: &mut [u8]) {
        match *self {
            State::Stable(v) => {
                buf[0] = unsafe { transmute(v) };
                buf[1] = 1;
            }
            State::Unstable(v, n) => {
                buf[0] = unsafe { transmute(v) };
                buf[1] = n << 1;
            }
        }
    }

    #[inline]
    pub fn deserialize(buf: &[u8]) -> Self {
        if buf[1] & 1 == 1 {
            State::Stable(unsafe { transmute(buf[0]) })
        } else {
            State::Unstable(unsafe { transmute(buf[0]) }, buf[1] >> 1)
        }
    }
}

impl Default for State {
    fn default() -> Self { State::Unstable(-(SEEDS as i8), 0) }
}


pub trait Table {
    fn new<T: AsRef<Path>>(T) -> Self;

    fn finish_hook(&mut self);

    fn pre_hook(&mut self, usize);

    fn post_hook(&mut self, usize);

    fn insert(&mut self, usize, State);

    fn index_mut(&mut self, usize) -> &mut State;

    /// Update the given state with the final score of one of its successors.
    /// Propagate it recursively whenever it flips the state to a final score.
    fn propagate(&mut self, u: Awari, up: i8, sat_lvl: i8) {
        let mut stack = vec![(u, up)];
        while let Some((u, a)) = stack.pop() {
            if let Some(b) = self.index_mut(u.encode()).update(a, sat_lvl) {
                debug_assert!(-sat_lvl <= b && b <= sat_lvl);
                // if update changed to final value, propagate further
                for v in u.predecessors() {
                    stack.push((v, b));
                }
            }
        }
    }

    /// Construct the optimal score table! Yay!
    fn build(&mut self) {
        // first iteration
        self.pre_hook(0);
        self.insert(0, State::Stable(0));
        self.post_hook(0);

        // don't compute the second to last iteration
        for n in 1..SEEDS-1 {
            self.iteration(n);
        }
        self.iteration(SEEDS);

        info!("The END!");
        self.finish_hook();
    }

    fn iteration(&mut self, n: usize) {
        info!("start of iteration {}", n);
        self.pre_hook(n);

        info!("initialization");
        for (c, u) in Awari::iter_config(n) {
            let (mut score, mut nsucc) = (-(n as i8), 0);
            for (v, k) in u.successors() {
                if k > 0 {
                    score = max(score, k as i8 - self.index_mut(v.encode()).value());
                }
                nsucc += 1;
            }
            self.insert(c, State::Unstable(score, nsucc));
        }

        info!("convergence");
        for l in 0..(n+1)/2 {
            info!("step {}", 2*l);
            let sat_lvl = (n - 2*l) as i8;
            for (c, u) in Awari::iter_config(n) {
                // yup, temporary lifetimes have struck again..
                if let Some(x) = { let ref mut tmp = self.index_mut(c);
                                   tmp.try_stabilize(sat_lvl) } {
                    debug_assert!(-sat_lvl <= x && x <= sat_lvl);
                    for v in u.predecessors() {
                        self.propagate(v, x, sat_lvl);
                    }
                }
            }
        }
        if n & 1 == 0 {
            info!("step {}", n);
            for (c, _) in Awari::iter_config(n) {
                let spot = self.index_mut(c);
                if let State::Unstable(_, _) = *spot {
                    *spot = State::Stable(0);
                }
            }
        }
    }
}
