// use std::thread;
// use std::rc::Rc;

// struct MyCounter
// {
//     count: i32
// }

// fn wont_work()
// {
//     let mut counter = Rc::new(MyCounter {count: 0});
//     thread::spawn(move || // new thread -- fails to compile here
//     {
//         counter.count += 1;
//     });
//     println!("{}", counter.count);
// }


use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

struct MyCounter {
    count: AtomicU32,
}

fn main() {
    let counter = Arc::new(MyCounter { count: AtomicU32::new(0) });

    let counter_clone = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        counter_clone.count.fetch_add(1, Ordering::SeqCst);
    });

    handle.join().unwrap();
    
    println!("{}", counter.count.load(Ordering::SeqCst));
}
