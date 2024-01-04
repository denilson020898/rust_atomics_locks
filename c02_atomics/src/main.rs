use std::{sync::atomic::{AtomicBool, AtomicUsize}, thread, time::Duration};
use std::sync::atomic::Ordering::Relaxed;

const TO_PROCESS: usize = 50;


fn main() {
    // static STOP: AtomicBool = AtomicBool::new(false);
    // let background_thread = thread::spawn(|| {
    //     while !STOP.load(std::sync::atomic::Ordering::Relaxed) {
    //         some_work();
    //     }
    // });
    // for line in std::io::stdin().lines() {
    //     match line.unwrap().as_str() {
    //         "help" => println!("commands: help, stop"),
    //         "stop" => break,
    //         cmd => println!("unknown command: {cmd:?}"),
    //     }
    // }
    // STOP.store(true, std::sync::atomic::Ordering::Relaxed);
    // background_thread.join().unwrap();

    let num_done = AtomicUsize::new(0);

    let main_thread = thread::current();

    thread::scope(|s| {
        s.spawn(|| {
            for i in 0..TO_PROCESS {
                process_item(i); // take some time
                num_done.store(i + 1, Relaxed);
                main_thread.unpark(); // wake up the main thread
            }
        });

        loop {
            let n = num_done.load(Relaxed);
            if n == TO_PROCESS { break; }
            println!("working.. {n}/{TO_PROCESS} done");
            // thread::sleep(Duration::from_secs(1));
            thread::park_timeout(Duration::from_secs(1));
        }

    });

    println!("completed");
}

fn some_work() {
    // println!("some work");
    // thread::sleep(Duration::from_secs(1));
}

fn process_item(inp: usize) -> usize {
    std::thread::sleep(Duration::from_secs(2));
    inp + 1
}
