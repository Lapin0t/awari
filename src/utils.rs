pub fn binom(k: usize, n: usize) -> usize {
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


pub fn binom_maxinv(k : usize , x: usize ) -> (usize , usize ) {
    assert!((k, x) != (0, 0));  // result should have been (-1, 0)
    if x == 0 { return (k - 1, 0); }
    let mut b = 1;
    let mut n = k;

    while b <= x {
        n += 1;
        b *= n;
        b /= n - k;
    }
    return (n - 1, b * (n-k) / n);
}
