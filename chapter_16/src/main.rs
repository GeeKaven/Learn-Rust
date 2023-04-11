use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("tread {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    //Rust 程序的主线程结束时，新线程也会结束，而不管其是否执行完毕
    for i in 1..5 {
        println!("local tread : {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    // 使用 `move` 将所有权移动至线程
    let v = vec![1, 2, 3];
    let handle1 = thread::spawn(move || {
        println!("thread vec: {:?}", v);
    });
    handle1.join().unwrap();

    // 信道传递数据
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("message"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result : {}", counter.lock().unwrap());
}
