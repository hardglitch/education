use education::queue_with_priorities::{PQueue, Priorities};

fn main() {
    let mut pq = PQueue::<u32>::default();
    pq.enqueue(Priorities::Normal, 23);
    pq.enqueue(Priorities::Low, 55);
    pq.enqueue(Priorities::High, 27);
    pq.enqueue(Priorities::Low, 58);
    pq.enqueue(Priorities::Low, 21);

    println!("{:?}", pq.dequeue());
    println!("{:?}", pq.dequeue());
    println!("{:?}", pq.dequeue());
    println!("{:?}", pq.dequeue());
    println!("{:?}", pq.dequeue());
    println!("{:?}", pq.dequeue());
}