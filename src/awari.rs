use std::ops::{Deref,DerefMut};
use std::iter::Iterator;
use std::fmt;
use std::cmp::min;

use {PITS,FPITS};
use utils::{binom,binom_maxinv,divmod};


#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Awari([u8; FPITS]);


pub struct Iter {
    curr: usize,
    last: usize,
    big: usize,
}


// use deref-coercion to provide all array (and slice) goodies on Awari
impl Deref for Awari {
    type Target = [u8; FPITS];

    fn deref(&self) -> &[u8; FPITS] {
        let &Awari(ref v) = self;
        return v;
    }
}


impl DerefMut for Awari {
    fn deref_mut(&mut self) -> &mut [u8; FPITS] {
        let &mut Awari(ref mut v) = self;
        return v;
    }
}


impl fmt::Debug for Awari {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n+")?;
        for _ in 0..PITS {
            write!(f, "--+")?;
        }
        write!(f, "\n|")?;
        for i in (PITS..FPITS).rev() {
            write!(f, "{:2}|", self[i])?;
        }
        write!(f, "\n+")?;
        for _ in 0..PITS {
            write!(f, "--+")?;
        }
        write!(f, "\n|")?;
        for i in 0..PITS {
            write!(f, "{:2}|", self[i])?;
        }
        write!(f, "\n+")?;
        for _ in 0..PITS {
            write!(f, "--+")?;
        }
        return Ok(());
    }
}


impl Awari {
    pub fn new() -> Self {
        Awari([0; FPITS])
    }

    pub fn iter_config(n: usize) -> Iter {
        let x = 1 << FPITS - 1;
        return Iter { curr: x - 1,
                      last: (x - 1) << n,
                      big: x << n }
    }

    pub fn encode(&self) -> usize {
        let (mut g, mut c) = (0, 0);
        for i in 0..FPITS {
            c += 1 + self[i] as usize;
            g += binom(i + 1, c - 1);
        }
        return g;
    }

    pub fn decode(g: usize) -> Self {
        let mut g = g;
        let mut s = Self::new();
        for i in (0..FPITS).rev() {
            let (x, b) = binom_maxinv(i + 1, g);
            s[i] = x as u8;
            g -= b;
        }
        for i in (1..FPITS).rev() {
            s[i] = s[i] - s[i-1] - 1;
        }
        return s;
    }

    pub fn predecessors(&self) -> Vec<Self> {
        let mut cpy = *self;
        cpy.rotate();
        
        let mut v = Vec::new();
        
        if (PITS..FPITS).all(|k| cpy[k] == 0) {
            return v;
        }

        let mut cmin = [0; FPITS-1];
        for i in 0..PITS {
            if cpy[i] == 0 {
                let mut m = cpy[i+1];
                cmin[0] = m;
                for r in 1..FPITS-1 {
                    let x = cpy[(i+r+1) % FPITS];
                    if m > x {
                        m = x;
                    }
                    cmin[r] = m;
                }
                let last = cmin[FPITS-2]+1;
                for r in 0..FPITS-1 {
                    if ((i+r+1) % FPITS < PITS) || (cpy[(i+r+1) % FPITS] != 2 && cpy[(i+r+1) % FPITS] != 3) {
                        for n in 0..min(cmin[r], last) {
                            let mut s = cpy;
                            s.unsow(i, r + 1, n);
                            v.push(s);
                        }
                    }
                }
            }
        }
        return v;
    }

    pub fn successors(&self) -> Vec<(Self, u8)> {
        let mut v = Vec::new();
        for i in 0..PITS {
            if self.valid_sow(i) {
                let mut s = *self;
                let k = s.play(i);
                v.push((s, k));
            }
        }
        return v;
    }

    fn rotate(&mut self) {
        for i in 0..PITS {
            self.swap(i, i + PITS);
        }
    }

    fn valid_sow(&self, i: usize) -> bool {
        let n = self[i];
        if i >= PITS || n == 0 {
            return false;
        } else {
            let (q, r) = divmod(n, (FPITS - 1) as u8);
            let j = (i + r) % FPITS;
            if j < i { return true; }  // at least one everywhere, no capture
            let mut take = true;
            for k in (PITS..FPITS).rev() {
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
    
    fn sow(&mut self, i: usize) -> (usize, u8) {
        debug_assert!(i < PITS, "pit index out of bounds");
        debug_assert!(self[i] > 0, "no seeds in pit");
        let n = self[i];
        self[i] = 0;
        let (q, r) = divmod(n, (FPITS - 1) as u8);
        for j in 1..r+1 {
            self[(i+j) % FPITS] += q+1;
        }
        for j in r+1..FPITS {
            self[(i+j) % FPITS] += q;
        }
        return ((i+r) % FPITS, q as u8);
    }

    fn unsow(&mut self, i: usize, r: usize, n: u8) {
        for k in 0..r {
            debug_assert!(self[(i+k+1) % FPITS] >= n + 1);
            self[(i+k+1) % FPITS] -= n + 1;
        }
        for k in r..FPITS-1 {
            debug_assert!(self[(i+k+1) % FPITS] >= n);
            self[(i+k+1) % FPITS] -= n;
        }
        self[i] += ((FPITS - 1) as u8)*n + r as u8;
    }

    fn collect(&mut self, i: usize) -> u8 {
        let mut j = i;
        let mut n = 0;
        while j >= PITS && (self[j] == 2 || self[j] == 3) {
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
    
}


impl Iterator for Iter {
    type Item = Awari;

    fn next(&mut self) -> Option<Awari> {
        if self.curr > self.last {
            return None;
        } else {
            // extract the board
            let mut x = self.curr | self.big;
            let mut s = Awari::new();

            for i in 0..FPITS {
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

#[cfg(test)]
mod tests {
    use quickcheck::{Arbitrary,Gen};
    use super::{FPITS,SEEDS,Awari};

    
    impl Arbitrary for Awari {
        fn arbitrary<G: Gen>(g: &mut G) -> Awari {
            let mut b = Awari::new();

            let n = g.gen_range(0, SEEDS) + 1;
            for i in 1..FPITS {
                b[i] = g.gen_range(0, n);
            }
            (*b).sort();
            for i in 0..FPITS-1 {
                b[i] = b[i+1] - b[i];
            }
            b[FPITS-1] = n - b[FPITS-1];
            return b;
        }
    }


    #[quickcheck]
    fn coding_bijective(u: Awari) -> bool {
        u == Awari::decode(u.encode())
    }

    #[quickcheck]
    fn all_succ_in_pred(u: Awari) -> bool {
        info!("new board:{:?}", u);
        u.successors()
          .into_iter()
          .inspect(|&(v, k)| if k == 0 { info!("new suc:{:?}", v); } )
          .all(|(v, k)| k > 0 || v.predecessors()
                                   .into_iter()
                                   .inspect(|&w| info!("pred of suc:{:?}", w) )
                                   .any(|w| u == w ))
    }

    #[quickcheck]
    fn all_pred_in_succ(u: Awari) -> bool {
        info!("new board:{:?}", u);
        u.predecessors()
          .into_iter()
          .inspect(|&v| info!("new pred:{:?}", v) )
          .all(|v| v.successors()
                     .into_iter()
                     .inspect(|&(w, k)| if k == 0 { info!("suc of pred:{:?}", w); } )
                     .any(|(w, _)| u == w))
    }
}
