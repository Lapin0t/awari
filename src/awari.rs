use std::ops::{Index,IndexMut};
use std::fmt;
use std::cmp;

#[cfg(test)]
use quickcheck::{Arbitrary,Gen};

use utils;


#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Board4([u8; 8]);


impl Index<usize> for Board4 {
    type Output = u8;
    fn index(&self, i: usize) -> &u8 {
        let Board4(ref b) = *self;
        &b[i]
    }
}

impl fmt::Debug for Board4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n-----------\n")?;
        write!(f, "{:2}{:3}{:3}{:3}\n", self[7], self[6], self[5], self[4])?;
        write!(f, "{:2}{:3}{:3}{:3}\n", self[0], self[1], self[2], self[3])?;
        write!(f, "-----------\n")
    }
}

impl Board4 {
    pub fn unwrap(self) -> [u8; 8] {
        let Board4(b) = self;
        b
    }

    pub fn iter_config(n: u8) -> impl Iterator<Item=Self> {
        (0..65739375).map(Self::decode)
    }

    pub fn encode(&self) -> u64 {
        let mut c = self[0] as u64;
        let mut g = utils::binom(1, c);
        for i in 1..7 {
            c += 1 + self[i] as u64;
            g += utils::binom(i as u64 + 1, c);
        }
        let n = (0..8).fold(0, |a, i| a + self[i]) as u64;
        n + 25 * g
    }

    pub fn decode(g : u64) -> Self {
        let b = &mut [0; 8];
        let n : u64 = g % 25;
        let mut g : u64 = g / 25;

        let mut s = 0;
        let (mut c1, d) = utils::binom_maxinv(7, g);
        g -= d;
        for k in (0..6).rev() {
            let (c0, d) = utils::binom_maxinv(k as u64 + 1, g);
            g -= d;
            let a = c1 - c0 - 1;
            s += a;
            b[k+1] = a as u8;
            c1 = c0;
        }
        b[0] = c1 as u8;
        b[7] = (n - s - c1) as u8;
        return Board4(*b);
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
        (i+r+1) % 8
    }

    fn unsow(&mut self, i: usize, n: u8) -> usize {
        // compute the rightmost min cell on my side, left of i
        let mut j = cmp::min(3, i);
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
        j
    }

    fn collect(&self, i: usize) -> usize {
        let mut j = i;
        while j >= 4 && (self[j] == 2 || self[j] == 3) {
            j -= 1;
        }
        j
    }
}


//TODO: make the generator more uniform...
#[cfg(test)]
impl Arbitrary for Board4 {
    fn arbitrary<G: Gen>(g: &mut G) -> Board4 {
        let b = &mut [0; 8];
        let mut s = 24u8;
        for i in 0..8 {
            let x = g.gen_range(0u8, s);
            b[i] = x;
            s -= x;
        }
        Board4(*b)
    }
}


#[quickcheck]
fn coding_bijective(b: Board4) -> bool {
    b == Board4::decode(b.encode())
}
