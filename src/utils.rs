use std::cmp::min;


const TBL_LEN : usize = 841;

/// Perfect hash function for binomial coefficient. The table will contain
/// binom(k, n) for each n >= 4 and 2 <= k <= n/2. The coefficient are stored
/// in increasing order of n and then k. */
fn idx(k: usize, n: usize) -> usize {
    assert!(n >= k && k <= n - k && k >= 2, "idx: bad argument");
    let q = (n - 2) >> 1;
    if (n - 2) & 1 == 0 {
        q*(q - 1) + k - 2
    } else {
        q*q + k - 2
    }
}


/// Compute the table for n up to 60 (incl)
fn mk_tbl() -> [usize; TBL_LEN] {
    let mut tbl = [0; TBL_LEN];
    tbl[0] = 6;
    tbl[1] = 10;
    for n in 6..61 {
        tbl[idx(2, n)] = n - 1 + tbl[idx(2, n-1)];
        for k in 3..n/2 {
            tbl[idx(k, n)] = tbl[idx(k, n-1)] + tbl[idx(k-1, n-1)];
        }
        tbl[idx(n/2, n)] = tbl[idx((n-1)/2, n-1)] + tbl[idx(n/2-1, n-1)];
    }
    return tbl;
}


lazy_static! {
    static ref BINOM_TBL : [usize; TBL_LEN] = mk_tbl();
}


pub fn binom(k: usize, n: usize) -> usize {
    if n < k {
        return 0;
    }
    let k = min(k, n - k);
    if k == 0 {
        return 1;
    } else if k == 1 {
        return n;
    } else {
        let id = idx(k, n);
        assert!(id < TBL_LEN, "cannot compute this binomial coefficient");
        return BINOM_TBL[id];
    }
}


pub fn binom_maxinv(k : usize , x: usize ) -> (usize , usize ) {
    assert!(k != 0);
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
    #[quickcheck]
    fn binom_rel1(k: usize, n: usize) -> bool {
        let (k, n) = (k >> 59, n >> 59);
    }
}
