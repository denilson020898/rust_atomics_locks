use std::thread;

fn main() {
    let h1 = thread::spawn(f);
    let h2 = thread::spawn(f);

    println!("Hello from main thread!");

    let a = h1.join().unwrap();
    let b = h2.join().unwrap();

    println!("result of a = {a}, and b = {b}");
}

fn f() -> bool {
    println!("hello from another thread!");
    let id = thread::current().id();
    println!("from child id: {id:?}");
    return false;
}
