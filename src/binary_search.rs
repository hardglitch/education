use std::cmp::Ordering;
use std::fmt::Display;

pub fn binary_search<T>(arr: &[T], elem: T) where T: Ord + Display {
    let mut left = 0;
    let mut right = arr.len();

    if Some(&elem) > arr.last() || Some(&elem) < arr.first() {
        return println!("Nothing found");
    }

    loop {
        let mid = (right - left)/2 + left;

        match elem.cmp(&arr[mid]) {
            Ordering::Less => right = mid,
            Ordering::Greater => left = mid,
            Ordering::Equal => {
                println!("Gotcha: {elem}=={}", arr[mid]);
                break;
            }
        }
    }
}
