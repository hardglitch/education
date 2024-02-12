pub fn ipow(mut base: f64, mut pow: i128) -> f64 {
    let mut res = 1e_0;
    while pow > 1 {
        if pow % 2 == 1 { res *= base; }
        base *= base;
        pow /= 2;
    }
    if pow > 0 { res *= base; }
    res
}