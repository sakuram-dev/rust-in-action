use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(100));

    let data_clone1 = Arc::clone(&data);
    thread::spawn(move || {
        let mut data = data_clone1.lock().unwrap();
        *data = 500;
    });

    let data_clone2 = Arc::clone(&data);
    thread::spawn(move || {
        let mut data = data_clone2.lock().unwrap();
        *data = 1000;
    });

    thread::sleep(std::time::Duration::from_millis(50));

    println!("{}", *data.lock().unwrap());
}

