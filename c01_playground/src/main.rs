use std::thread;



fn main() {
    // test 1
    let h1 = thread::spawn(f);
    let h2 = thread::spawn(f);

    println!("Hello from main thread!");

    let a = h1.join().unwrap();
    let b = h2.join().unwrap();
    println!("result of a = {a}, and b = {b}");

    // test 2
    let numbers = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        for n in numbers {
            println!("{n}");
        }
    });
    handle.join().unwrap();

    // test 3
    let numbers = Vec::from_iter(0..=1000);
    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.iter().sum::<usize>();
        sum / len
    });
    let average = t.join().unwrap();
    println!("average is {average}");
}

fn f() -> bool {
    println!("hello from another thread!");
    let id = thread::current().id();
    println!("from child id: {id:?}");
    return false;
}
