use std::ops::{Deref,DerefMut};
use std::iter::Iterator;
use std::fmt;
use std::cmp::min;

use {SEEDS,START_SEEDS,PITS,FPITS};
use utils::{binom,divmod,n_boards,enc_min};


/// Representation of an awari board configuration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Awari([u8; FPITS]);


// use deref-coercion to provide all array (and slice) goodies on Awari
impl Deref for Awari {
    type Target = [u8; FPITS];

    #[inline]
    fn deref(&self) -> &[u8; FPITS] {
        let &Awari(ref v) = self;
        return v;
    }
}


impl DerefMut for Awari {
    #[inline]
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


impl Default for Awari {
    /// Instanciate the canonical starting board configuration.
    fn default() -> Self {
        Awari([START_SEEDS as u8; FPITS])
    }
}


impl Awari {
    /// Iterate on every board configuration with a given number of seeds.
    pub fn iter_config(n: usize) -> Iter {
        let x = 1 << FPITS - 1;
        return Iter { curr: x - 1,
                      last: (x - 1) << n,
                      mask: x << n,
                      code: enc_min(n) - 1 }
    }

    /// Return a compact encoding of an awari board as an integer.
    #[inline]
    pub fn encode(&self) -> usize {
        let (mut g, mut c) = (0, 0);
        let mut xs = [0; FPITS];
        for i in 0..FPITS {
            c += self[i] as usize;
            xs[i] = c;
            g += binom(i + 1, c + i);
        }

        // configs with SEEDS-1 seeds are not accessible
        if xs[FPITS-1] == SEEDS {
            g -= n_boards(SEEDS - 1);
        }

        let mut brk = 0;
        let mut last = xs[PITS-1];
        for i in PITS..FPITS {
            if last >= xs[i] || xs[i] <= i - PITS {
                xs[i] += 1;
                brk = i;
            }
            last = xs[i];
        }

        debug_assert!(brk > 0);

        for i in brk..FPITS {
            g -= binom(i + 1, xs[i] + PITS - 1);
        }
        return g;
    }

    /// Compute every legal predecessor that has the same score (only 0-valued
    /// back-moves are allowed taken into account).
    pub fn predecessors(&self) -> Vec<Self> {
        let mut cpy = *self;
        cpy.rotate();
        
        let mut v = Vec::new();

        if cpy[PITS..FPITS].iter().all(|&x| x == 0) {
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
                            if cpy[PITS..FPITS].iter().any(|&x| x == 0) {
                                v.push(s);
                            }
                        }
                    }
                }
            }
        }
        return v;
    }

    /*pub fn predecessors_nz(&self, n: u8) -> Vec<Self> {
        debug_assert!(n > 0);
        
        let mut cpy = *self;
        cpy.rotate();
        
        let mut v = Vec::new();

        let mut i = PITS;
        while i < FPITS && self[i] > 0 { i += 1; }
        let a = i;
        while i < FPITS && self[i] == 0 { i += 1; }
        let b = i;

        i = PITS - 1;
        while i >= 0
        let mut m = SEEDS;
        while i >= 0 && self[i] > 0 {
            m = min(m, self[i]);
            i -= 1;
        }

        for c in (n+2)/3..1+min(n/2, (b - a) as u8) {
            let mut x = (1 << (n - 2*c)) - 1;
            let y = x << (3*c - n);
            while x <= y {
                let mut cpy = self;
                for i in 0..c {
                    cpy[a + i] = 2 + ((x >> i) & 1);
                }
                let t = x | (x - 1);
                x = (t+1) | (((!t & (t+1)) - 1) >> (x.trailing_zeros() + 1));
            }
        }
        return v;
    }*/

    /// Compute every legal successors configuration of the current
    /// board together with the reward of the move.
    #[inline]
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

    /// Rotate the board (simulating a player change).
    #[inline]
    fn rotate(&mut self) {
        for i in 0..PITS {
            self.swap(i, i + PITS);
        }
    }

    /// Test if `i` is a valid move.
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

    /// Do the "sowing" part of the move, starting from pit `i`. Panics if
    /// this is not a valid move.
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

    /// Undo the sowing that was done starting from pit `i`, that contained
    /// `(FPITS-1)*n + r` seeds. Panics if this is not a valid predecessor.
    #[inline]
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

    /// Finish the move that ended in pit `i` by collecting the needed seeds .
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

    /// Play the pit `i` and return the number of captured seeds. Panics if
    /// this is not a valid move (`self[i] == 0`).
    fn play(&mut self, i: usize) -> u8 {
        let (j, _) = self.sow(i);
        let k = self.collect(j);
        self.rotate();
        return k;
    }
}


/// Iterator for awari board configurations with a given number of seeds.
/// This is really fast thanks to bitwise tricks.
pub struct Iter {
    curr: usize,
    last: usize,
    mask: usize,
    code: usize
}


impl Iterator for Iter {
    type Item = (usize, Awari);

    #[inline]
    fn next(&mut self) -> Option<(usize, Awari)> {
        if self.curr > self.last {
            return None;
        } else {
            // extract the board
            let mut x = self.curr | self.mask;
            let mut s: Awari = Default::default();

            for i in 0..FPITS {
                let tz = x.trailing_zeros();
                s[i] = tz as u8;
                x >>= tz + 1;
            }

            // increment state
            let c = self.curr;
            let t = c | (c - 1);
            self.curr = (t+1) | (((!t & (t+1)) - 1) >> (c.trailing_zeros() + 1));

            if s[PITS..FPITS].iter().all(|&a| a > 0) {
                return self.next();
            } else {
                self.code += 1;
                debug_assert!(self.code == s.encode());
                return Some((self.code, s));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use test::{black_box,Bencher};
    use rand::{Rng,thread_rng};
    use quickcheck::{Arbitrary,Gen,StdGen};

    use {FPITS,SEEDS,NBOARDS};
    use super::Awari;

    
    impl Arbitrary for Awari {
        fn arbitrary<G: Gen>(g: &mut G) -> Awari {
            let mut b: Awari = Default::default();

            b[0] = 0;
            let n = g.gen_range(0, SEEDS as u8) + 1;
            for i in 1..FPITS {
                b[i] = g.gen_range(0, n);
            }
            b.sort();
            for i in 0..FPITS-1 {
                b[i] = b[i+1] - b[i];
            }
            b[FPITS-1] = n - b[FPITS-1];
            return b;
        }
    }


    /*#[quickcheck]
    fn coding_bijective(u: Awari) -> bool {
        u == Awari::decode(u.encode())
    }*/

    #[quickcheck]
    fn all_succ_in_pred(u: Awari) -> bool {
        u.successors()
          .into_iter()
          .all(|(v, k)| k > 0 || v.predecessors()
                                   .into_iter()
                                   .any(|w| u == w ))
    }

    #[quickcheck]
    fn all_pred_in_succ(u: Awari) -> bool {
        u.predecessors()
          .into_iter()
          .all(|v| v.successors()
                     .into_iter()
                     .any(|(w, _)| u == w))
    }

    #[bench]
    fn bench_encode_100(b: &mut Bencher) {
        let mut gen = StdGen::new(thread_rng(), 100);
        let board = Awari::arbitrary(&mut gen);
        b.iter(|| { for _ in 0..100 { black_box(board.encode()); } });
    }

    /*#[bench]
    fn bench_decode_100(b: &mut Bencher) {
        let mut rng = thread_rng();
        let n = rng.gen_range(0, NBOARDS);
        b.iter(|| { for _ in 0..100 { black_box(Awari::decode(n)); } });
    }*/

    #[bench]
    fn bench_successors_100(b: &mut Bencher) {
        let mut gen = StdGen::new(thread_rng(), 100);
        let board = Awari::arbitrary(&mut gen);
        b.iter(|| { for _ in 0..100 { black_box(board.successors()); } });
    }

    #[bench]
    fn bench_predecessors_100(b: &mut Bencher) {
        let mut gen = StdGen::new(thread_rng(), 100);
        let board = Awari::arbitrary(&mut gen);
        b.iter(|| { for _ in 0..100 { black_box(board.predecessors()); } });
    }

    #[bench]
    fn bench_iterconfig_100(b: &mut Bencher) {
        let mut iter = Awari::iter_config(24);
        b.iter(|| { for _ in 0..100 { black_box(iter.next()); } });
    }
}
