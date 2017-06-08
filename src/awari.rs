use std::ops::{Index,IndexMut};
use std::fmt::{Debug,Formatter,Result};
use std::cmp::min;
use std::hash::{Hash,Hasher};
use std::iter::Iterator;

#[cfg(test)]
use quickcheck::{Arbitrary,Gen};

use utils::{binom,binom_maxinv};


#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Board4([u8; 8]);


impl Index<usize> for Board4 {
    type Output = u8;
    fn index(&self, i: usize) -> &u8 {
        let Board4(ref b) = *self;
        &b[i]
    }
}


impl Debug for Board4 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "\n-----------\n")?;
        write!(f, "{:2}{:3}{:3}{:3}\n", self[7], self[6], self[5], self[4])?;
        write!(f, "{:2}{:3}{:3}{:3}\n", self[0], self[1], self[2], self[3])?;
        write!(f, "-----------\n")
    }
}


impl Hash for Board4 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.encode());
    }
}


impl Board4 {
    pub fn unwrap(self) -> [u8; 8] {
        let Board4(b) = self;
        return b;
    }

    pub fn iter_config(n: usize) -> impl Iterator<Item=Self> {
        (binom(8, 7 + n)..binom(8, 8+n)).map(Board4::decode)
    }

    pub fn encode(&self) -> usize {
        let (mut n, mut c) = (0, 0);
        for (i, &x) in self.unwrap().into_iter().enumerate() {
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
            n -= b;
            brd[i] = x as u8;
        }
        let mut c = brd[0];
        for i in 1..8 {
            brd[i] = c;
            c += brd[i] + 1;
        }
        return Board4(brd);
    }

    pub fn rotate(&mut self) {
        let mut b = self.unwrap();
        for i in 0..4 {
            b.swap(i, i+4);
        }
    }

    pub fn is_valid(&self, i: usize) -> bool {
        let n = self[i];
        if i >= 4 || n == 0 { return false; }
        else {
            let (q, r) = (n / 7, (n % 7) as usize);
            let j = (i + r + 1) % 8;
            let mut take = true;
            for k in (4..8).rev() {
                if k > j {
                    if self[k] > q { return true; }
                } else {
                    take = take && (self[k] == 2 || self[k] == 3);
                    if !take && self[k] > q+1 { return true; }
                }
            }
            return false;
        }
    }

    fn sow(&mut self, i: usize) -> usize {
        assert!(i < 4, "pit index out of bounds");
        assert!(self[i] > 0, "no seeds in pit");
        let n = self[i];
        let mut b = self.unwrap();
        b[i] = 0;
        let (q, r) = (n / 7, (n % 7) as usize);
        for j in 1..r+1 {
            b[(i+j) % 8] += q+1;
        }
        for j in r+1..8 {
            b[(i+j) % 8] += q;
        }
        return (i+r+1) % 8;
    }

    fn unsow(&mut self, i: usize, n: u8) -> usize {
        // compute the rightmost min cell on my side, left of i
        let mut j = min(3, i);
        while j > 0 && self[j] != 0 {
            j -= 1;
        }
        assert!(self[j] == 0, "invalid unsowing");
        let mut b = self.unwrap();
        for k in 0..j {
            assert!(b[k] >= n, "invalid unsowing");
            b[k] -= n;
        }
        for k in j..i+1 {
            assert!(b[k] >= n+1, "invalid unsowing");
            b[k] -= n + 1;
        }
        for k in i+1..8 {
            assert!(b[k] >= n, "invalid unsowing");
            b[k] -= n;
        }
        return j;
    }

    fn collect(&mut self, i: usize) -> u8 {
        let mut j = i;
        let mut n = 0;
        let mut b = self.unwrap();
        while j >= 4 && (b[j] == 2 || b[j] == 3) {
            n += b[j];
            b[j] = 0;
            j -= 1;
        }
        return n;
    }

    pub fn play(&mut self, i: usize) -> u8 {
        let j = self.sow(i);
        let k = self.collect(j);
        self.rotate();
        return k;
    }
}


#[cfg(test)]
impl Arbitrary for Board4 {
    fn arbitrary<G: Gen>(g: &mut G) -> Board4 {
        let mut b = [0; 8];
        for i in 1..8 {
            b[i] = g.gen_range(0u8, 24);
        }
        b.sort();
        for i in 0..7 {
            b[i] = b[i+1] - b[i];
        }
        b[7] = 24 - b[7];
        return Board4(b);
    }
}


#[quickcheck]
fn coding_bijective(b: Board4) -> bool {
    println!("{:?}\n{}\n{:?}", b, b.encode(), Board4::decode(b.encode()));
    b == Board4::decode(b.encode())
}
