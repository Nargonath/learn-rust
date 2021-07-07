use std::sync::{Arc, Mutex};
use std::{thread, time};

fn main() {
    let counter1 = Arc::new(Mutex::new(0));
    let counter2 = Arc::new(Mutex::new(0));

    let counter_temp_1 = Arc::clone(&counter1);
    let counter_temp_2 = Arc::clone(&counter2);
    let handle = thread::spawn(move || {
        let mut num = counter_temp_1.lock().unwrap();
        *num += 1;

        // gives enough time for the other thread to start
        // and lock
        thread::sleep(time::Duration::from_millis(3000));

        let num2 = counter_temp_2.lock().unwrap();
        println!("Total thread1 = {}", *num + *num2);
    });

    let handle2 = thread::spawn(move || {
        let mut num2 = counter2.lock().unwrap();
        *num2 += 1;

        // gives enough time for the other thread to start
        // and lock
        thread::sleep(time::Duration::from_millis(3000));

        let num = counter1.lock().unwrap();
        println!("Total thread2 = {}", *num + *num2);
    });

    handle.join().unwrap();
    handle2.join().unwrap();
}
