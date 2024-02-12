pub fn simple_numbers<T>(max: T) where T: Copy + Into<i64> {
    println!("1");
    let is_simple = &mut true;
    for i in 2..max.into() {
        *is_simple = true;
        for n in 2..i+1 {
            if i%n != 0 { continue; }
            if i/n != 1 {
                *is_simple = false;
                continue;
            }
            else if *is_simple { println!("{i}") }
        }
    }
}
