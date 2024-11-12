// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 22 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. Because of the difference between the
// spawned threads' sleep time, and the waiting threads sleep time, when you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::sync::Mutex;

fn main() {
    let counter = Arc::new(Mutex::new(0)); 
    let status_shared = Arc::clone(&counter);
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            let mut num = status_shared.lock().unwrap();
            *num += 1;
        }
    });
    while *counter.lock().unwrap() < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}
