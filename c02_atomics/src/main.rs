use core::panic;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;
use std::time::Instant;
use std::{
    sync::atomic::{AtomicBool, AtomicU64, AtomicUsize},
    thread,
    time::Duration,
};

const TO_PROCESS: usize = 100;

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

    // let num_done = AtomicUsize::new(0);
    //
    // let main_thread = thread::current();
    //
    // thread::scope(|s| {
    //     s.spawn(|| {
    //         for i in 0..TO_PROCESS {
    //             process_item(i); // take some time
    //             num_done.store(i + 1, Relaxed);
    //             main_thread.unpark(); // wake up the main thread
    //         }
    //     });
    //
    //     loop {
    //         let n = num_done.load(Relaxed);
    //         if n == TO_PROCESS { break; }
    //         println!("working.. {n}/{TO_PROCESS} done");
    //         // thread::sleep(Duration::from_secs(1));
    //         thread::park_timeout(Duration::from_secs(1));
    //     }
    //
    // });
    //
    // println!("completed");

    // let num_done = &AtomicUsize::new(0);
    // thread::scope(|s| {
    //     for t in 0..4 {
    //         s.spawn(move || {
    //             for i in 0..25 {
    //                 process_item(t * 25 + i); // take some time
    //                 num_done.fetch_add(1, Relaxed);
    //             }
    //         });
    //     }
    //
    //     loop {
    //         let n = num_done.load(Relaxed);
    //         if n == TO_PROCESS {
    //             break;
    //         }
    //         println!("working.. {n}/{TO_PROCESS} done");
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });
    // let n = num_done.load(Relaxed);
    // println!("completed with {}", n);

    let num_done = &AtomicUsize::new(0);
    let total_time = &AtomicU64::new(0);
    let max_time = &AtomicU64::new(0);
    thread::scope(|scope| {
        for t in 0..4 {
            scope.spawn(move || {
                for i in 0..25 {
                    let start = Instant::now();
                    process_item(t * 25 + i);
                    let time_taken = start.elapsed().as_micros() as u64;
                    num_done.fetch_add(1, Relaxed);
                    total_time.fetch_add(time_taken, Relaxed);
                    max_time.fetch_max(time_taken, Relaxed);
                }
            });
        }

        loop {
            let total_time = Duration::from_micros(total_time.load(Relaxed));
            let max_time = Duration::from_micros(max_time.load(Relaxed));
            let n = num_done.load(Relaxed);
            if n == TO_PROCESS {
                break;
            }
            if n == 0 {
                println!("gabut bos");
            } else {
                println!(
                    "Working.. {n}/100 done, {:?} average, {:?} peak",
                    total_time / n as u32,
                    max_time
                );
            }
            thread::sleep(Duration::from_secs(1));
        }
    });
    let n = num_done.load(Relaxed);
    println!("completed with {}", n);
}

fn some_work() {
    // println!("some work");
    // thread::sleep(Duration::from_secs(1));
}

fn process_item(inp: usize) -> usize {
    dbg!(inp);
    std::thread::sleep(Duration::from_secs(1));
    inp + 1
}

fn get_x() -> u64 {
    static X: AtomicU64 = AtomicU64::new(0);
    let mut x = X.load(Relaxed);
    if x == 0 {
        x = calculate_x();
        X.store(x, Relaxed);
    }
    x
}

fn calculate_x() -> u64 {
    todo!()
}

fn allocate_new_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let id = NEXT_ID.fetch_add(1, Relaxed);
    if id >= 1000 {
        NEXT_ID.fetch_sub(1, Relaxed);
        panic!("too many ids!");
    }
    return id;
}

fn increment(a: &AtomicU32) {
    let mut current = a.load(Relaxed);
    loop {
        let new = current + 1;
        match a.compare_exchange(current, new, Relaxed, Relaxed) {
            Ok(_) => return,
            Err(v) => current = v,
        }
    }
}
