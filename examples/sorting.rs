use education::sorting::{gen_arr, Sort};

fn main() {
    let mut arr = gen_arr(30);
    let mut s = Sort::new(&mut arr);
    s.bubble();

    arr = gen_arr(30);
    s = Sort::new(&mut arr);
    s.selection();

    arr = gen_arr(30);
    s = Sort::new(&mut arr);
    s.insertion();

    arr = gen_arr(30);
    s = Sort::new(&mut arr);
    s.heapsort();

    arr = gen_arr(30);
    s = Sort::new(&mut arr);
    s.quicksort();
}
