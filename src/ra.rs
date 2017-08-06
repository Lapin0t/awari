use std::cmp::max;
use std::option::Option;
use std::ops::IndexMut;

use core::nonzero::NonZero;

use SEEDS;
use awari::Awari;

pub use storage::{NaiveRAM,MMaped};


#[derive(Eq,PartialEq)]
pub struct State {
    val: i8,
    nsuc: Option<NonZero<u8>>,
}

impl State {
    pub fn new(v: i8, n: u8) -> Self {
        State {
            val: v,
            nsuc: unsafe { Option::Some(NonZero::new_unchecked(n + 1)) },
        }
    }

    /// Update the value of a board using the final value `up` from a successor.
    /// If it flipped the board to a stable state return the final value of the
    /// board, else return `None`.
    #[inline]
    pub fn update(&mut self, up: i8, sat_lvl: i8) -> Option<i8> {
        debug_assert!(self.nsuc == Option::None || sat_lvl >= max(self.val, -up));

        match self.nsuc {
            Option::None => {
                debug_assert!(self.val >= up);
                Option::None
            },
            Option::Some(n) => {
                debug_assert!(n.get() >= 2);
                if n.get() == 2 {
                    self.nsuc = Option::None;
                    Option::Some(max(self.val, -up))
                } else if self.val == sat_lvl || -up == sat_lvl {
                    self.nsuc = Option::None;
                    self.val = sat_lvl;
                    Option::Some(sat_lvl)
                } else {
                    self.nsuc = unsafe {
                        Option::Some(NonZero::new_unchecked(n.get()-1))
                    };
                    if self.val < -up {
                        self.val = -up;
                    }
                    Option::None
                }
            }
        }
    }

    /// If the board has no more successor to wait for, flip it to stable and
    /// return the final value, else do nothing.
    #[inline]
    pub fn try_stabilize(&mut self, sat_lvl: i8) -> Option<i8> {
        if sat_lvl == 0 {
            // little hack: if sat_lvl == 0 this is the last branch
            if self.nsuc != Option::None {
                self.val = 0;
                self.nsuc = Option::None;
            }
            Option::None
        } else {
            match self.nsuc {
                Option::Some(n) if self.val == sat_lvl || n.get() == 1 => {
                    self.nsuc = Option::None;
                    Option::Some(self.val)
                },
                _ => Option::None,
            }
        }
    }
}


pub trait Table: IndexMut<usize, Output=State> {
    fn insert(&mut self, usize, State);
    fn pre_hook(&mut self, usize);
    fn post_hook(&mut self, usize);
    fn finish_hook(&mut self);
}


/// Update the given state with the final score of one of its successors.
/// Propagate it recursively whenever it flips the:w
// state to a final score.
fn propagate<T: Table>(table: &mut T, u: Awari, up: i8, sat_lvl: i8) {
    let mut stack = vec![(u, up)];
    while let Some((u, a)) = stack.pop() {
        if let Some(b) = table[u.encode()].update(a, sat_lvl) {
            debug_assert!(-sat_lvl <= b && b <= sat_lvl);
            // if update changed to final value, propagate further
            for v in u.predecessors() {
                stack.push((v, b));
            }
        }
    }
}

fn iteration<T: Table>(table: &mut T, n: usize) {
    info!("start of iteration {}", n);
    table.pre_hook(n);

    info!("initialization");
    for (c, u) in Awari::iter_config(n) {
        let (mut score, mut nsucc) = (-(n as i8), 0);
        for (v, k) in u.successors() {
            if k > 0 {
                score = max(score, k as i8 - table[v.encode()].val);
            }
            nsucc += 1;
        }
        table.insert(c, State::new(score, nsucc));
    }

    info!("convergence");
    for l in 0..(n+1)/2 {
        info!("step {}", 2*l);
        let sat_lvl = (n - 2*l) as i8;
        for (c, u) in Awari::iter_config(n) {
            // yup, temporary lifetimes have struck again..
            if let Some(x) = { let ref mut tmp = table[c];
                               tmp.try_stabilize(sat_lvl) } {
                debug_assert!(-sat_lvl <= x && x <= sat_lvl);
                for v in u.predecessors() {
                    propagate(table, v, x, sat_lvl);
                }
            }
        }
    }
    if n & 1 == 0 {
        info!("step {}", n);
        for (c, _) in Awari::iter_config(n) {
            let ref mut tmp = table[c];
            if tmp.nsuc != Option::None {
                tmp.val = 0;
                tmp.nsuc = Option::None;
            }
        }
    }
}


/// Construct the optimal score table! Yay!
pub fn build<T: Table>(table: &mut T) {
    // first iteration
    table.pre_hook(0);
    table.insert(0, State { val: 0, nsuc: Option::None });
    table.post_hook(0);

    // don't compute the second to last iteration
    for n in 1..SEEDS-1 {
        iteration(table, n);
    }
    iteration(table, SEEDS);

    table.finish_hook();
}
