use std::fmt::Debug;
use rand::{Rng, thread_rng};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Sort<'a, T> {
    data: &'a mut[T]
}
impl<'a, T> Sort<'a, T> where T: PartialOrd + Debug + Copy + Serialize {

    pub fn new(arr: &'a mut[T]) -> Self {
        Self { data: arr }
    }

    pub fn bubble(&mut self) {
        let mut i: usize = 0;
        let mut t = true;
        let l = self.data.len();

        while t {
            t = false;
            for j in 0..l-i-2 {
                if self.data[j] > self.data[j+1] {
                    self.data.swap(j, j+1);
                    t = true;
                }
            }
            i += 1;
        }
        println!("Bubble Sort {:?}", self.data);
    }

    pub fn selection(&mut self) {
        for i in 0..self.data.len() {
            let mut min_idx = i;
            for j in i+1..self.data.len() {
                if self.data[min_idx] > self.data[j] {
                    min_idx = j
                }
            }
            self.data.swap(i, min_idx);
        }
        println!("Selection Sort {:?}", self.data);
    }

    pub fn insertion(&mut self) {
        for i in 1..self.data.len() {
            let mut j = i;
            while j > 0 && self.data[j] < self.data[j-1] {
                self.data.swap(j, j-1);
                j -= 1;
            }
        }
        println!("Insertion Sort {:?}", self.data);
    }

    pub fn heapsort(&mut self) {
        let l = self.data.len();

        for i in (0..l/2).rev() {
            Self::_heapify(self.data, l, i);
        }

        for i in (1..l).rev() {
            self.data.swap(0, i);
            Self::_heapify(self.data, i, 0);
        }

        println!("Heapsort {:?}", self.data);
    }
    fn _heapify(arr: &mut [T], l: usize, i: usize) {
        let mut largest = i;
        let l_ch = 2 * i + 1;
        let r_ch = 2 * i + 2;

        if l_ch < l && arr[l_ch] > arr[largest] {
            largest = l_ch;
        }
        if r_ch < l && arr[r_ch] > arr[largest] {
            largest = r_ch;
        }

        if largest != i {
            arr.swap(i, largest);
            Sort::_heapify(arr, l, largest);
        }
    }

    pub fn quicksort(&mut self) {
        let l = self.data.len() - 1;
        Self::_quicksort(self.data, 0, l as isize);
        println!("Quicksort {:?}", self.data);
    }
    fn _quicksort(arr: &mut [T], left: isize, right: isize) {
        if left >= right { return; }
        let p = Self::_partition(arr, left, right);
        Self::_quicksort(arr, left, p - 1);
        Self::_quicksort(arr, p + 1, right);
    }
    fn _partition(arr: &mut [T], left: isize, right: isize) -> isize {
        let pivot = arr[right as usize];
        let mut i = left - 1;

        for j in left..=right {
            if arr[j as usize] > pivot { continue; }
            i += 1;
            if i <= right { arr.swap(i as usize, j as usize); }
            else { break; }
        }
        i
    }

    pub fn mergesort(&mut self) {
        let l = self.data.len() - 1;
        Self::_mergesort(self.data, 0, l as isize);
        println!("Mergesort {:?}", self.data);
        // let mut file = File::create("result.txt").unwrap();
        // let res = serde_json::to_string(self.data).expect("Some Error");
        // file.write_all(res.as_bytes()).unwrap();
    }
    fn _mergesort(arr: &mut [T], left: isize, right: isize) {
        if left >= right { return; }
        let mid = left + (right - left)/2;
        Self::_mergesort(arr, left, mid);
        Self::_mergesort(arr, mid+1, right);
        Self::_merge(arr, left, mid, right);
    }

    fn _merge(arr: &mut [T], left: isize, mid: isize, right: isize) {
        let mut out = Vec::<T>::new();
        let mut a = left;
        let mut b = mid + 1;

        while a <= mid && b <= right {
            if arr[a as usize] < arr[b as usize] {
                if let Some(e) = arr.get(a as usize) {
                    out.push(*e);
                }
                a += 1;
            } else {
                if let Some(e) = arr.get(b as usize) {
                    out.push(*e);
                }
                b += 1;
            }
        }
        while a <= mid {
            if let Some(e) = arr.get(a as usize) {
                out.push(*e);
            }
            a += 1;
        }
        while b <= right {
            if let Some(e) = arr.get(b as usize) {
                out.push(*e);
            }
            b += 1;
        }
        for j in left..=right {
            if let Some(e) = out.get((j-left) as usize) {
                arr[j as usize] = *e;
            }
        }
    }
}


pub fn gen_arr(len: usize) -> Vec<usize> {
    let mut v = Vec::<usize>::with_capacity(len);
    for _ in 0..len {
        let mut rng = thread_rng();
        let a = rng.gen_range(0..len);
        v.push(a);
    }
    // println!("Generated Array {:?}", v);
    v
}
