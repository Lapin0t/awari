use {FPITS,SEEDS};


const TBL_LEN : usize = FPITS*(FPITS + SEEDS + 2);


fn binom_slow(k: usize, n: usize) -> usize {
    if n < k {
        return 0;
    }
    let mut p = 1;
    for i in 0..k {
        p *= n - i;
        p /= i + 1;
    }
    return p;
}


fn mk_tbl() -> [usize; TBL_LEN] {
    let mut tbl = [0; TBL_LEN];
    for k in 1..FPITS+1 {
        tbl[k] = 0;
    }
    for n in 1..FPITS+SEEDS+1 {
        for k in 1..FPITS+1 {
            tbl[n*FPITS + k] = binom_slow(k, n);
        }
    }
    return tbl;
}


lazy_static! {
    static ref BINOM_TBL : [usize; TBL_LEN] = mk_tbl();
}


#[inline(always)]
pub fn binom(k: usize, n: usize) -> usize {
    BINOM_TBL[n*FPITS + k]
}


pub fn binom_maxinv(k : usize , x: usize ) -> (usize , usize ) {
    debug_assert!(k != 0);
    let (mut a, mut b) = (k-1, 61);

    while b - a > 1 {
        let c = (a + b + 1) / 2;
        if binom(k, c) <= x {
            a = c;
        } else {
            b = c;
        }
    }
    return (a, binom(k, a));
}


pub fn divmod(n: u8, d: u8) -> (u8, usize) {
    ((n - 1) / d, ((n - 1) % d + 1) as usize)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn binom_rel1(k: usize, n: usize) -> bool {
        let (k, n) = (k >> 59, n >> 59);
        k == 0 || k >= n || binom(k, n) == binom(k-1, n-1) + binom(k, n-1)
    }
    
    #[quickcheck]
    fn binom_maxinv_rel1(k: usize, x: usize) -> bool {
        let (k, x) = (k >> 59, x >> 58);
        if k == 0 { return true; }
        let (n, b) = binom_maxinv(k, x);
        return b == binom(k, n);
    }

    #[quickcheck]
    fn binom_maxinv_rel2(k: usize, x: usize) -> bool {
        let (k, x) = (k >> 59, x >> 58);
        if k == 0 { return true; }
        let (_, b) = binom_maxinv(k, x);
        return b <= x;
    }
}
