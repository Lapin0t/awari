pub fn binom(k: u64, n: u64) -> u64 {
    if n < k {
        return 0;
    }
    let a : u64 = (n-k+1..n+1).product();
    let b : u64 = (2..k+1).product();
    return a / b;
}

pub fn binom_maxinv(k : u64, x: u64) -> (u64, u64) {
    let mut b = 1;
    let mut n = k;

    while b <= x {
        n += 1;
        b *= n;
        b /= n - k;
    }
    return (n - 1, b * (n-k) / n);
}
