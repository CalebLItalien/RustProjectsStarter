use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    //handle.join().unwrap();

    let (tx, rx) = mpsc::channel(); // mpsc = multiple producer, single consumer
    let tx1 = tx.clone();
    thread::spawn(move || {
        // let val = String::from("hi");
        // tx.send(val).unwrap(); // unwrwap causes panic in case of an error
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("Got: {}", received);
    }
    // let received = rx.recv().unwrap();
    // // rx is iterable
    // println!("Got: {}", received);
}
