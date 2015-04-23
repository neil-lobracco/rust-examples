use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};

fn mutex_approach() {
    let data = Arc::new(Mutex::new(vec![0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));

    for _ in 0..11 {
        let data = data.clone();
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            for d in data.iter_mut() {
                *d += 1;
            }
        });
    }
    
    thread::sleep_ms(100);
    println!("The vector is now: {:?}",*data.lock().unwrap());
}

fn atomic_approach() {
    let data: Arc<Vec<AtomicUsize>> = Arc::new((0..11).map(|i| AtomicUsize::new(i)).collect());
    for _ in 0..11 {
        let data = data.clone();
        thread::spawn(move || {
            for d in data.iter() {
                d.fetch_add(1, Ordering::SeqCst);
            }
        });
    }
    thread::sleep_ms(100);
    let raw_nums: Vec<usize> = data.iter().map(|au: &AtomicUsize| au.load(Ordering::SeqCst)).collect();
    println!("The vector is now: {:?}",raw_nums);
}

fn main() {
    mutex_approach();
    atomic_approach();
}
