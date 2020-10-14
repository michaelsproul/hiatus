use hiatus::step;
use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;

// Two threads use step counters to coordinate ordered access to a shared vector.
fn main() {
    // Enable step points.
    hiatus::enable();

    let vector = Arc::new(Mutex::new(vec![]));

    let v1 = vector.clone();
    let v2 = vector.clone();

    let t1 = thread::spawn(move || {
        let s1 = step(1);
        v1.lock().push(1);
        drop(s1);

        let s3 = step(3);
        thread::sleep(std::time::Duration::from_secs(1));
        v1.lock().push(3);
        drop(s3);
    });
    let t2 = thread::spawn(move || {
        let s2 = step(2);
        v2.lock().push(2);
        drop(s2);

        let s4 = step(4);
        v2.lock().push(4);
        drop(s4);
    });
    t1.join().unwrap();
    t2.join().unwrap();

    assert_eq!(vector.lock().as_slice(), &[1, 2, 3, 4]);
}
