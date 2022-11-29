#![allow(unused_variables)]

use std::sync::Arc;
use std::thread;

fn main() {
    let s = Arc::new(String::from("multiple threads!!"));
    for _ in 0..10 {
        let s = Arc::clone(&s);
        let handle = thread::spawn(move || {
            println!("{}", s);
        });
    }
}
