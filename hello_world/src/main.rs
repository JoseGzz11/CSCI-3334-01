fn sending_data_across_threads() {
    extern crate rand; // 0.8.5

    use std::thread;
    // multiproducer, single consumer
    use std::sync::mpsc::channel;

    let (sender,reciever) = channel();

    for i in 0..10 {
        let sender = sender.clone();
        thread::spawn(move || {
            println!("sending: {}",i);
            sender.send(i).unwrap(); // any data could be passed to reciever
            // as well as sending could fail
        });
    }

    for _ in 0..10 {
        let msg = reciever.recv().unwrap();
        println!(" recieved {}", msg );
    }
    // what is important to notice, data will be send and recieved in random order
    // but you will get them in exact order, just be aware of potential queue

    // basically CPU whim

}