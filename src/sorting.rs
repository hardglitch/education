use std::fmt::Debug;
use rand::{Rng, thread_rng};

#[derive(Debug)]
pub struct Sort<'a, T> {
    data: &'a mut[T]
}
impl<'a, T> Sort<'a, T> where T: PartialOrd + Debug + Copy {

    pub fn new(arr: &'a mut[T]) -> Self {
        Self { data: arr }
    }

    pub fn bubble(&'a mut self) {
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

    pub fn selection(&'a mut self) {
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

    pub fn insertion(&'a mut self) {
        for i in 1..self.data.len() {
            let mut j = i;
            while j > 0 && self.data[j] < self.data[j-1] {
                self.data.swap(j, j-1);
                j -= 1;
            }
        }
        println!("Insertion Sort {:?}", self.data);
    }

    pub fn heapsort(&'a mut self) {
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
        let p = Self::partition(arr, left, right);
        Self::_quicksort(arr, left, p - 1);
        Self::_quicksort(arr, p + 1, right);
    }
    fn partition(arr: &mut [T], left: isize, right: isize) -> isize {
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
}


pub fn gen_arr(len: usize) -> Vec<usize> {
    let mut v = Vec::<usize>::with_capacity(len);
    for _ in 0..len {
        let mut rng = thread_rng();
        let a = rng.gen_range(0..1000);
        v.push(a);
    }
    // println!("Generated Array {:?}", v);
    v
}
