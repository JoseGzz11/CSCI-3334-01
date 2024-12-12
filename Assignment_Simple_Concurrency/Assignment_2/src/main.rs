use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Shared counter wrapped in Arc and Mutex
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    // Spawn 5 threads
    for _ in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the final value of the counter
    let final_count = *counter.lock().unwrap();
    println!("Final counter value: {}", final_count);
}
