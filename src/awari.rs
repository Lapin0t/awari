use std::ops::{Index,IndexMut};
use std::fmt;
use std::fmt::{Debug,Formatter};
use std::hash::{Hash,Hasher};
use std::iter::Iterator;
use std::vec::Vec;

use utils::{binom,binom_maxinv,divmod};


#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Board4(pub [u8; 8]);


impl Index<usize> for Board4 {
    type Output = u8;
    fn index(&self, i: usize) -> &u8 {
        let Board4(ref b) = *self;
        &b[i]
    }
}


impl IndexMut<usize> for Board4 {
    fn index_mut(&mut self, i: usize) -> &mut u8 {
        return self.0.index_mut(i);
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


impl Hash for Board4 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.encode());
    }
}


impl Board4 {
    pub fn iter_config(n: usize) -> impl Iterator<Item=Self> {
        (binom(8, 7+n)..binom(8, 8+n)).map(Board4::decode)
    }

    pub fn encode(&self) -> usize {
        let (mut n, mut c) = (0, 0);
        for (i, &x) in self.0.into_iter().enumerate() {
            c += x as usize;
            n += binom(i + 1, c + i);
        }
        return n;
    }

    pub fn decode(n : usize) -> Self {
        let mut n = n;
        let mut brd = [0; 8];
        for i in (0..8).rev() {
            let (x, b) = binom_maxinv(i + 1, n);
            brd[i] = x as u8;
            n -= b;
        }
        let mut c = brd[0];
        for i in 1..8 {
            let d = brd[i];
            brd[i] = d - c - 1;
            c = d;
        }
        return Board4(brd);
    }

    pub fn rotate(&mut self) {
        for i in 0..4 {
            self.0.swap(i, i+4);
        }
    }

    pub fn valid_sow(&self, i: usize) -> bool {
        info!("valid_sow, i={}", i);
        let n = self[i];
        if i >= 4 || n == 0 { return false; }
        else {
            let (q, r) = divmod(n, 7);
            let j = (i + r) % 8;
            info!("q={}, r={}, j={}", q, r, j);
            if j < i { return true; }  // at least one everywhere, no capture
            let mut take = true;
            for k in (4..8).rev() {
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

    pub fn valid_unsow(&self, i: usize, n: u8) -> bool {
        if i >= 4 && (self[i] == 2 || self[i] == 3) { return false; }
        if (4..8).all(|k| self[k] == 0) { return false; }
        match (1..8).rev().find(|&r| { let a = (i+r) % 8; a < 4 && self[a] == 0 }) {
            None => false,
            Some(r) => {
                (1..r).all(|k| self[(i+k) % 8] >= n) &&
                (r+1..9).all(|k| self[(i+k) % 8] >= n + 1)
            }
        }
    }

    pub fn sow(&mut self, i: usize) -> (usize, u8) {
        assert!(i < 4, "pit index out of bounds");
        assert!(self[i] > 0, "no seeds in pit");
        let n = self[i];
        self[i] = 0;
        let (q, r) = (n / 7, (n % 7) as usize);
        ////info!("q = {}, r = {}", q, r);
        for j in 1..r+1 {
            self[(i+j) % 8] += q+1;
        }
        for j in r+1..8 {
            self[(i+j) % 8] += q;
        }
        if r == 0 {
            return ((i+7) % 8, q as u8 - 1);
        } else {
            return ((i+r) % 8, q as u8);
        }
    }

    pub fn unsow(&mut self, i: usize, n: u8) -> usize {
        //info!("unsow: j: {}, n: {}", i, n);

        let r = (1..8).rev().find(|&r| { let a = (i+r) % 8; a < 4 && self[a] == 0 }).unwrap();
        //info!("r: {} (i = {})", r, (i + r) % 8);

        for k in 1..r {
            //info!("{} -= {}", (i+k) % 8, n);
            assert!(self[(i+k) % 8] >= n);
            self[(i+k) % 8] -= n;
        }
        for k in r+1..9 {
            //info!("{} -= {}", (i+k) % 8, n+1);
            assert!(self[(i+k) % 8] >= n+1);
            self[(i+k) % 8] -= n + 1;
        }
        let j = (i + r) % 8;
        //info!("{} += {}", j, (8 - r) as u8 + 7*n);
        self[j] += (8 - r) as u8 + 7*n;
        return j;
    }

    fn collect(&mut self, i: usize) -> u8 {
        let mut j = i;
        let mut n = 0;
        while j >= 4 && (self[j] == 2 || self[j] == 3) {
            n += self[j];
            self[j] = 0;
            j -= 1;
        }
        return n;
    }

    pub fn play(&mut self, i: usize) -> u8 {
        let (j, _) = self.sow(i);
        let k = self.collect(j);
        self.rotate();
        return k;
    }

    pub fn predecessors(&self) -> Vec<Self> {
        let mut cpy = *self;
        cpy.rotate();
        
        let mut v = Vec::new();
        for i in 0..8 {
            for n in 0..4 {
                if cpy.valid_unsow(i, n) {
                    let mut s = cpy;
                    s.unsow(i, n);
                    v.push(s);
                }
            }
        }
        return v;
    }

    pub fn successors(&self) -> Vec<(Self, u8)> {
        let mut v = Vec::new();
        for i in 0..4 {
            if self.valid_sow(i) {
                let mut s = *self;  // copy
                let k = s.play(i);
                v.push((s, k));
            }
        }
        return v;
    }
}
