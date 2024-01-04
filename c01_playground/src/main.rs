use std::{
    borrow::BorrowMut,
    cell::{Cell, RefCell},
    marker::PhantomData,
    rc::Rc,
    sync::{Arc, Mutex},
    thread, time::Duration,
};

static X: [i32; 3] = [1, 2, 3];

fn f() -> bool {
    println!("hello from another thread!");
    let id = thread::current().id();

    let binding = thread::current();
    let name = binding.name();
    println!("from child id: {id:?} and {name:?}");
    return false;
}

// in this example, a and b can not be the same reference
fn f2(a: &i32, b: &mut i32) {
    let before = *a;
    *b += 1;
    let after = *a;
    if before != after {
        x(); // never happens
    }
}

fn x() {}

// in this example, a and b can be the same reference
// in the same thread
fn f3(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();
    if before != after {
        x(); //might happen
    }
}

// move out the value from cell and replace it
// cell does not allow you to borrow its content
fn f4(v: &Cell<Vec<i32>>) {
    let mut v2 = v.take(); //replace with empty
    v2.push(1);
    v.set(v2);
}

fn f5(v: &RefCell<Vec<i32>>) {
    v.borrow_mut().push(1); // only in single thread
}

struct X {
    handle: i32,
    _not_sync: PhantomData<Cell<()>>,
}

struct XTwo {
    p: *mut i32,
}

unsafe impl Send for XTwo {}
unsafe impl Sync for XTwo {}

fn main() {
    // test 1
    let h1 = thread::spawn(f);
    let h2 = thread::spawn(f);
    let h3 = thread::Builder::new()
        .name("Motherfucker".into())
        .spawn(f)
        .unwrap();

    println!("Hello from main thread!");

    let a = h1.join().unwrap();
    let b = h2.join().unwrap();
    let c = h3.join().unwrap();
    println!("result of a = {a}, and b = {b}, and c = {c}");

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

    // scoped thread
    let numbers = vec![1, 2, 3, 4];
    thread::scope(|s| {
        s.spawn(|| {
            println!("length: {}", numbers.len());
        });

        s.spawn(|| {
            for n in &numbers {
                println!("{n}");
            }
        });
    });

    // scoped thread can not modify captured variables
    // let mut numbers = vec![1,2,3,4];
    // thread::scope(|s|{
    //     s.spawn(|| {
    //         numbers.push(1);
    //     });
    //     s.spawn(|| {
    //         numbers.push(2);
    //     });
    // })

    // non owned static var; no ownership from the beginning
    let a = thread::spawn(|| dbg!(&X));
    let b = thread::spawn(|| dbg!(&X));
    a.join();
    b.join();

    // leaking heap pointer for non owned variable; give up ownership after creating it
    let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));
    let a = thread::spawn(move || dbg!(x));
    let b = thread::spawn(move || dbg!(x));
    let _ = a.join();
    let _ = b.join();

    // Reference counting; still retain ownership
    let a = Rc::new([1, 2, 3]);
    let b = a.clone();
    dbg!(a.as_ptr());
    dbg!(b.as_ptr());
    assert_eq!(a.as_ptr(), b.as_ptr());

    // but cannot be sent between thread
    // thread::spawn(move || dbg!(b));

    // use atomic ref counting instead
    let a = Arc::new([1, 2, 3]);
    let b = a.clone();
    thread::spawn(move || dbg!(a)).join().unwrap();
    thread::spawn(move || dbg!(b)).join().unwrap();

    // naming
    let a = Arc::new([1, 2, 3]);
    let b = a.clone();
    let bhandle = thread::spawn(move || {
        dbg!(b);
    });
    dbg!(a);
    bhandle.join().unwrap();

    // this pattern is better, shadowing variable declaration
    let a = Arc::new([1, 2, 3]);
    let innerhandle = thread::spawn({
        let a = a.clone();
        move || {
            dbg!(a);
        }
    });
    dbg!(a);
    innerhandle.join().unwrap();

    let a = [123, 456, 789];
    let b = unsafe { a.get_unchecked(5) };
    println!("b is random value outside of the length of a: {b}");

    // let a = Rc::new(123);
    // Rc is not 'Send'
    // thread::spawn(move || {
    //     dbg!(a);
    // }).join();


    // thread A lock -> Success -> Returns MutexGuard (impl DerefMut) -> Ops -> Drop MutexGuard -> unlocked -> woke other thread
    // thread B lock -> Blocked ------------------------------------------------------------------------------------------------> lock -> Returns MutexGuard -> ...
    // let n = Mutex::new(0);
    // thread::scope(|s| {
    //     for _ in 0..10 {
    //         s.spawn(|| {
    //             let mut guard = n.lock().unwrap();
    //             let threadid = thread::current().id();
    //             for _ in 0..100 {
    //                 // println!("updating inner 'n' from thread id : {threadid:?}");
    //                 *guard += 1;
    //             }
    //
    //             thread::sleep(Duration::from_secs(1));
    //             // unlock the guard after this line 
    //         });
    //     }
    // });
    // assert_eq!(n.into_inner().unwrap(), 1000);


    let n = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap();
                let threadid = thread::current().id();
                for _ in 0..100 {
                    println!("updating inner 'n' from thread id : {threadid:?}");
                    *guard += 1;
                }

                drop(guard); // drop guard before delaying, unlock the mutex here
                // the sleeping and unlock + wake other thread is happening at the same time
                thread::sleep(Duration::from_secs(1));
                // unlock the guard after this line 
            });
        }
    });
    assert_eq!(n.into_inner().unwrap(), 1000);
}
