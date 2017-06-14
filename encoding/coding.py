def binom_inv(k, bound):
    """Compute the largest ``n`` such that ``binom(k, n) <= bound``."""

    binom = 1
    n = k
    while binom <= bound:
        n += 1
        binom *= n
        binom //= n-k
    return n-1


def binom(k, n):
    """k choose n"""

    if n < k:
        return 0
    p = 1
    for i in range(k):
        p *= n - i
        p //= i + 1
    return p


def encode(xs):
    """Encode an awari board as an integer."""

    n, c = 0, 0
    for i, x in enumerate(xs):
        c += x
        n += binom(i + 1, c + i)
    return n


def decode(n, n_pits=8):
    """Decode an awari board."""

    s = []
    for i in reversed(range(n_pits)):
        x = binom_inv(i+1, n)
        n -= binom(i+1, x)
        s.append(x)
    return set2list(list(reversed(s)))


def enc_min(n, n_pits=8):
    """Smallest code for boards with ``n`` seeds."""

    return binom(n_pits, n_pits+n-1)


def iter_boards(n, n_pits=8):
    """Iter on all boards with ``n`` seeds."""

    return map(decode2, range(enc_min(n, n_pits), enc_min(n+1, n_pits)))


def list2set(xs):
    """Set-theoretic encoding of lists as sets.

    The set will be represented as a python list in sorted order."""

    s, c = [], -1
    for x in xs:
        c += x + 1
        s.append(c)
    return s


def set2list(s):
    """Decode a set describing a list.

    The set must be represented as a python list in sorted order."""

    xs, c = [s[0]], s[0]
    for x in s[1:]:
        xs.append(x - c - 1)
        c = x
    return xs
