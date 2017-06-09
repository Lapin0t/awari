use quickcheck::{Arbitrary,Gen};

use awari::Board4;
use utils::{binom,binom_maxinv};


impl Arbitrary for Board4 {
    fn arbitrary<G: Gen>(g: &mut G) -> Board4 {
        let mut b : [u8; 8] = [0; 8];
        for i in 1..8 {
            b[i] = g.gen_range(0, 24);
        }
        b.sort();
        for i in 0..7 {
            b[i] = b[i+1] - b[i];
        }
        b[7] = 24 - b[7];
        return Board4::wrap(b);
    }
}


#[quickcheck]
fn coding_bijective(b: Board4) -> bool {
    b == Board4::decode(b.encode())
}

#[quickcheck]
fn binom_rel1(k: usize, n: usize) -> bool {
    let (k, n) = (k >> 59, n >> 59);
    k == 0 || k >= n || binom(k, n) == binom(k-1, n-1) + binom(k, n-1)
}

#[quickcheck]
fn binom_maxinv_rel1(k: usize, x: usize) -> bool {
    let (k, x) = (k >> 59, x >> 58);
    if (k, x) == (0, 0) { return true; }  // discard test
    let (n, b) = binom_maxinv(k, x);
    return b == binom(k, n);
}

#[quickcheck]
fn binom_maxinv_rel2(k: usize, x: usize) -> bool {
    let (k, x) = (k >> 59, x >> 58);
    if (k, x) == (0, 0) { return true; }  // discard test
    let (_, b) = binom_maxinv(k, x);
    return b <= x;
}
