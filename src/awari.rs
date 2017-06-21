use std::ops::{Index,IndexMut};
use std::fmt;
use std::fmt::{Debug,Formatter};
use std::iter::Iterator;
use std::vec::Vec;
use std::marker::PhantomData;

use utils::{binom,binom_maxinv,divmod};



pub struct Iter<T: Awari> {
    curr: usize,
    last: usize,
    big: usize,
    _marker: PhantomData<T>,
}

pub trait Awari : Index<usize,Output=u8> + IndexMut<usize> + Copy {
    const N : usize;
    const M : usize = 2*Self::N;

    fn new() -> Self;

    fn iter_config(n: usize) -> Iter<Self> {
        let x = 1 << Self::M - 1;
        return Iter { curr: x - 1,
                      last: (x - 1) << n,
                      big: x << n,
                      _marker: PhantomData }
    }

    fn encode(&self) -> usize {
        let (mut n, mut c) = (0, 0);
        for i in 0..Self::M {
            c += self[i] as usize;
            n += binom(i + 1, c + i);
        }
        return n;
    }

    fn decode(n: usize) -> Self {
        let mut n = n;
        let mut s = Self::new();
        for i in (0..Self::M).rev() {
            let (x, b) = binom_maxinv(i + 1, n);
            s[i] = x as u8;
            n -= b;
        }
        for i in (1..Self::M).rev() {
            s[i] = s[i] - s[i-1] - 1
        }
        return s;
    }

    fn rotate(&mut self) {
        for i in 0..Self::N {
            let x = self[i];
            self[i] = self[i + Self::N];
            self[i + Self::N] = x;
        }
    }

    fn valid_sow(&self, i: usize) -> bool {
        info!("valid_sow, i={}", i);
        let n = self[i];
        if i >= Self::N || n == 0 { return false; }
        else {
            let (q, r) = divmod(n, (Self::M - 1) as u8);
            let j = (i + r) % Self::M;
            info!("q={}, r={}, j={}", q, r, j);
            if j < i { return true; }  // at least one everywhere, no capture
            let mut take = true;
            for k in (Self::N..Self::M).rev() {
                if k > j {
                    if self[k] + q > 0 { return true; }
                } else {
                    take = take && (self[k] + q == 1 || self[k] + q == 2);
                    if !take { return true; }
                }
            }
            return false;
        }
    }

    fn valid_unsow(&self, i: usize, n: u8) -> bool {
        if i >= Self::N && (self[i] == 2 || self[i] == 3) {
            return false;
        }
        if (Self::N..Self::M).all(|k| self[k] == 0) {
            return false;
        }
        match (1..Self::M).rev()
                  .find(|&r| { let a = (i+r) % Self::M;
                               a < Self::N && self[a] == 0 }) {
            None => false,
            Some(r) => {
                (1..r).all(|k| self[(i+k) % Self::M] >= n) &&
                (r+1..Self::M+1).all(|k| self[(i+k) % Self::M] >= n + 1)
            }
        }
    }

    fn sow(&mut self, i: usize) -> (usize, u8) {
        assert!(i < Self::N, "pit index out of bounds");
        assert!(self[i] > 0, "no seeds in pit");
        let n = self[i];
        self[i] = 0;
        let (q, r) = divmod(n, (Self::M - 1) as u8);
        for j in 1..r+1 {
            self[(i+j) % Self::M] += q+1;
        }
        for j in r+1..Self::M {
            self[(i+j) % Self::M] += q;
        }
        return ((i+r) % Self::M, q as u8);
    }

    fn unsow(&mut self, i: usize, n: u8) -> usize {
        //info!("unsow: j: {}, n: {}", i, n);

        let r = (1..Self::M).rev()
            .find(|&r| { let a = (i+r) % Self::M;
                         a < Self::N && self[a] == 0 })
            .unwrap();

        //info!("r: {} (i = {})", r, (i + r) % 8);

        for k in 1..r {
            //info!("{} -= {}", (i+k) % 8, n);
            assert!(self[(i+k) % Self::M] >= n);
            self[(i+k) % Self::M] -= n;
        }
        for k in r+1..Self::M+1 {
            //info!("{} -= {}", (i+k) % 8, n+1);
            assert!(self[(i+k) % Self::M] >= n+1);
            self[(i+k) % Self::M] -= n + 1;
        }
        let j = (i + r) % Self::M;
        //info!("{} += {}", j, (8 - r) as u8 + 7*n);
        self[j] += (Self::M - r) as u8 + (Self::M as u8 - 1) * n;
        return j;
    }

    fn collect(&mut self, i: usize) -> u8 {
        let mut j = i;
        let mut n = 0;
        while j >= Self::N && (self[j] == 2 || self[j] == 3) {
            n += self[j];
            self[j] = 0;
            j -= 1;
        }
        return n;
    }

    fn play(&mut self, i: usize) -> u8 {
        let (j, _) = self.sow(i);
        let k = self.collect(j);
        self.rotate();
        return k;
    }

    fn predecessors(&self) -> Vec<Self> {
        let mut cpy = *self;
        cpy.rotate();
        
        let mut v = Vec::new();
        for i in 0..Self::M {
            for n in 0..4 {
                info!("pred: testing for i={}, n={}", i, n);
                if cpy.valid_unsow(i, n) {
                    let mut s = cpy;
                    s.unsow(i, n);
                    v.push(s);
                } else { info!("invalid") }
            }
        }
        return v;
    }

    fn successors(&self) -> Vec<(Self, u8)> {
        let mut v = Vec::new();
        for i in 0..Self::N {
            if self.valid_sow(i) {
                let mut s = *self;  // copy
                let k = s.play(i);
                v.push((s, k));
            }
        }
        return v;
    }
}


impl<T: Awari> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.curr > self.last {
            return None;
        } else {

            // extract the board
            let mut x = self.curr | self.big;
            let mut s = T::new();

            for i in 0..T::M {
                let tz = x.trailing_zeros();
                s[i] = tz as u8;
                x >>= tz + 1;
            }

            // increment state
            let c = self.curr;
            let t = c | (c - 1);
            self.curr = (t+1) | (((!t & (t+1)) - 1) >> (c.trailing_zeros() + 1));

            return Some(s);
        }
    }
}


#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Board4(pub [u8; 8]);


impl Index<usize> for Board4 {
    type Output = u8;
    fn index(&self, i: usize) -> &u8 {
        self.0.index(i)
    }
}


impl IndexMut<usize> for Board4 {
    fn index_mut(&mut self, i: usize) -> &mut u8 {
        self.0.index_mut(i)
    }
}


impl Debug for Board4 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "\n+--+--+--+--+")?;
        write!(f, "\n|{:2}|{:2}|{:2}|{:2}|",
              self[7], self[6], self[5], self[4])?;
        write!(f, "\n+--+--+--+--+")?;
        write!(f, "\n|{:2}|{:2}|{:2}|{:2}|",
              self[0], self[1], self[2], self[3])?;
        write!(f, "\n+--+--+--+--+")
    }
}

impl Awari for Board4 {
    const N : usize = 4;
    fn new() -> Self { Board4([0; 8]) }
}


#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Board6(pub [u8; 12]);


impl Index<usize> for Board6 {
    type Output = u8;
    fn index(&self, i: usize) -> &u8 {
        self.0.index(i)
    }
}


impl IndexMut<usize> for Board6 {
    fn index_mut(&mut self, i: usize) -> &mut u8 {
        self.0.index_mut(i)
    }
}


impl Debug for Board6 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "\n+--+--+--+--+--+--+")?;
        write!(f, "\n|{:2}|{:2}|{:2}|{:2}|{:2}|{:2}|",
              self[11], self[10], self[9], self[8], self[7], self[6])?;
        write!(f, "\n+--+--+--+--+--+--+")?;
        write!(f, "\n|{:2}|{:2}|{:2}|{:2}|{:2}|{:2}|",
              self[0], self[1], self[2], self[3], self[4], self[5])?;
        write!(f, "\n+--+--+--+--+--+--+")
    }
}

impl Awari for Board6 {
    const N : usize = 6;
    fn new() -> Self { Board6([0; 12]) }
}
