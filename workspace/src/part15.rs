// Rust-101, Part 15: Mutex, Interior Mutability (cont.), Sync
// ===========================================================

use std::sync::{Arc, Mutex};
use std::thread;


// The derived `Clone` implementation will clone the `Arc`, so all clones will actually talk about the same counter.
#[derive(Clone)]
struct ConcurrentCounter(Arc<Mutex<usize>>);

impl ConcurrentCounter {
    // The constructor just wraps the constructors of `Arc` and `Mutex`.
    pub fn new(val: usize) -> Self {
        unimplemented!()
    }

    pub fn increment(&self, by: usize) {
        // `lock` on a mutex returns a *guard*, giving access to the data contained in the mutex.
        let mut counter = self.0.lock().unwrap();
        *counter = *counter + by;
    }

    // The function `get` returns the current value of the counter.
    pub fn get(&self) -> usize {
        unimplemented!()
    }
}

// Now our counter is ready for action.
pub fn main() {
    let counter = ConcurrentCounter::new(0);

    // We clone the counter for the first thread, which increments it by 2 every 15ms.
    let counter1 = counter.clone();
    let handle1 = thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep_ms(15);
            counter1.increment(2);
        }
    });

    // The second thread increments the counter by 3 every 20ms.
    let counter2 = counter.clone();
    let handle2 = thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep_ms(20);
            counter2.increment(3);
        }
    });

    // Now we watch the threads working on the counter.
    for _ in 0..50 {
        thread::sleep_ms(5);
        println!("Current value: {}", counter.get());
    }

    // Finally, we wait for all the threads to finish to be sure we can catch the counter's final value.
    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("Final value: {}", counter.get());
}

// **Exercise 14.1**: Besides `Mutex`, there's also [`RwLock`](http://doc.rust-lang.org/stable/std/sync/struct.RwLock.html), which
// provides two ways of locking: One that grants only read-only access, to any number of concurrent readers, and another one
// for exclusive write access. (Notice that this is the same pattern we already saw with shared vs. mutable borrows.) Change
// the code above to use `RwLock`, such that multiple calls to `get` can be executed at the same time.
// 
// **Exercise 14.2**: Add an operation `compare_and_inc(&self, test: usize, by: usize)` that increments the counter by
// `by` *only if* the current value is `test`.


// FIXME TODO some old outdated explanation FIXME TODO


