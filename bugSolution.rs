use std::sync::{Arc, Mutex};

fn main() {
    let x = Arc::new(Mutex::new(5));
    
    let y = Arc::clone(&x);
    let z = Arc::clone(&x);

    let t1 = std::thread::spawn(move || {
        let mut x_locked = y.lock().unwrap();
        *x_locked = 10;
    });

    let t2 = std::thread::spawn(move || {
        let x_locked = z.lock().unwrap();
        println!("x = {}", *x_locked);
    });
    
    t1.join().unwrap();
    t2.join().unwrap();
} 