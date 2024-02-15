use std::collections::HashMap;

fn is_prime<T>(num: T, cache: &[T]) -> bool where T: Copy + Into<isize> + From<isize> {
    if (1..num.into().abs().isqrt())
        .all(|i| cache.get(i as usize)
            .is_some_and(|&n| num.into() % n.into() != 0))
    { return true }
    false
}

pub fn count_primes<T>(num: T) -> usize where T: Copy + Into<isize> + From<isize> {
    let c = &mut 0_usize;
    let mut cache = vec![];

    for i in 1..=num.into().abs() {
        if is_prime(i, &cache) {
            *c += 1;
            cache.push(i);
        }
    }
    *c - 1
}

pub fn get_primes<T>(num: T) -> Vec<usize> where T: Copy + Into<isize> + From<isize> {
    let num_max: usize = num.into().unsigned_abs();
    let mut lp = HashMap::<usize, usize>::with_capacity(num_max);
    let mut pr = vec![];

    for k in 2..=num_max {
        if lp.get(&k).is_none() {
            lp.insert(k, k);
            pr.push(k);
        }
        for p in pr.iter() {
            if lp.get(&k).is_some_and(|i| p <= i && p*i <= num_max) {
                lp.insert(p*k, *p);
            }
        }
    }
    pr
}
