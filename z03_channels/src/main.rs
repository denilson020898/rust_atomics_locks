use std::{sync::mpsc, thread, time::Duration};

fn main() {
    // let (tx, rx) = mpsc::channel();
    // let t1 = tx.clone();
    // let t2 = tx.clone();
    // let t3 = tx.clone();
    //
    // let th1 = thread::spawn(move || {
    //     thread::sleep(Duration::from_secs(5));
    //     t1.send(50).unwrap();
    //     println!("thread 1 completed: 50");
    // });
    //
    // let th2 = thread::spawn(move || {
    //     thread::sleep(Duration::from_secs(2));
    //     t2.send(123).unwrap();
    //     println!("thread 2 completed: 123");
    // });
    //
    // let th3 = thread::spawn(move || {
    //     thread::sleep(Duration::from_secs(10));
    //     t3.send(666).unwrap();
    //     println!("thread 3 completed: 666");
    // });
    //
    // let th4 = thread::spawn(move || {
    //     let result = rx.recv().unwrap() + rx.recv().unwrap() + rx.recv().unwrap();
    //     println!("total: {result}");
    // });
    //
    // th1.join().unwrap();
    // th2.join().unwrap();
    // th3.join().unwrap();
    // th4.join().unwrap();
    //
    // println!("All threads completed!");

    thread::scope(|scope| {
        let (tx, rx) = mpsc::channel();
        let t1 = tx.clone();
        let t2 = tx.clone();
        let t3 = tx.clone();

        scope.spawn(move || {
            thread::sleep(Duration::from_secs(5));
            t1.send(50).unwrap();
            println!("thread 1 completed: 50");
        });

        scope.spawn(move || {
            thread::sleep(Duration::from_secs(2));
            t2.send(123).unwrap();
            println!("thread 2 completed: 123");
        });

        scope.spawn(move || {
            thread::sleep(Duration::from_secs(10));
            t3.send(666).unwrap();
            println!("thread 3 completed: 666");
        });

        scope.spawn(move || {
            let result = rx.recv().unwrap() + rx.recv().unwrap() + rx.recv().unwrap();
            println!("total: {result}");
        });
    });
    println!("All threads completed!");

    let add = |x, y| x + y;
    let res = add(1,2);

    let report = || println!("the result of the closure is {}", res);
    report();
}
