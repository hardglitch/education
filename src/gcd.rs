pub fn gcd(mut u: isize, mut v: isize) -> isize
{
    if u == 0 { return v; }
    else if v == 0 { return u; }

    let i = u.trailing_zeros();  u >>= i;
    let j = v.trailing_zeros();  v >>= j;
    let k = std::cmp::min(i, j);

    loop {
        if u > v { std::mem::swap(&mut u, &mut v); }
        v -= u;
        if v == 0 { return u << k; }
        v >>= v.trailing_zeros();
    }
}