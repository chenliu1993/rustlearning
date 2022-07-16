use std::sync::Mutex;
use std::thread;
use std::sync::Arc;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];
    for _ in 0..9 {
        let counter = Arc::clone(&counter);
        let handler = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handlers.push(handler);
    }

    for handler in handlers {
        handler.join().unwrap();
    }

    println!("The result is {}", *counter.lock().unwrap());

}
