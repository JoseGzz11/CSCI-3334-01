use std::thread;

fn main() {
    let mut handles = Vec::new();

    for i in 1..=3 {
        let handle = thread::spawn(move || {
            println!("Thread {}", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread failed to complete.");
    }

    println!("All threads completed.");
}
