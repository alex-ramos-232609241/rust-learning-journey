use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from another thread");
    });

    handle.join().unwrap();
}
