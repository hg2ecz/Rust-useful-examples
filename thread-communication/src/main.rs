use std::thread;
mod buffer;

fn producer_thread(buffer: &buffer::Buffer) {
    for i in 0..50 {
        println!("p: {}", i);
        buffer.insert(i);
    }
}

fn consumer_thread(buffer: &buffer::Buffer) {
    for _ in 0..50 {
        let val = buffer.remove();
        println!("c: {}", val);
    }
}

fn main() {
    let buff = buffer::newbuffer(); // Arc::new()

    let buff_clone = buff.clone(); // clone & move
    let p = thread::spawn(move || {
        producer_thread(&buff_clone);
    });

    let c = thread::spawn(move || {
        consumer_thread(&buff);
    });

    p.join().expect("Producer had an error");
    c.join().expect("Consumer had an error");
}
