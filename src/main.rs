//use std::collections::HashMap;
use std::thread;

fn main() {
    //let s: HashMap = HashMap::new();

    thread::spawn(|| println!("Hello, world!"))
        .join()
        .unwrap();
}
