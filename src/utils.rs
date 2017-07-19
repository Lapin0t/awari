use {FPITS,SEEDS,BINOM_TBL};


/// Compute a binomial coefficient (using in-memory precomputed table).
#[inline]
pub fn binom(k: usize, n: usize) -> usize {
    debug_assert!(0 < k && k <= FPITS && n <= FPITS+SEEDS);
    BINOM_TBL[n*FPITS + k - 1]
}


/// Compute the largest ``n`` such that ``\binom{n}{k} <= x``.
#[inline]
pub fn binom_maxinv(k : usize , x: usize ) -> (usize , usize ) {
    debug_assert!(k != 0);
    let (mut a, mut b) = (k-1, FPITS+SEEDS+1);

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


/// Compute the euclidean division and remainder where the remainder is
/// in range [1,n].
#[inline]
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
