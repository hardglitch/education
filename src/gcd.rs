pub fn gcd(mut u: i64, mut v: i64) -> i64
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