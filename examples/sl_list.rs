use education::sl_list::List;

fn main() {
    let mut list: List<i32> = List::default();
    list.push(23);
    list.push(34);
    println!("{:?}", list.pop());
    println!("{:?}", list.pop());
    println!("{:?}", list.pop());
}
