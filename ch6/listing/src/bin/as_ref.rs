#![allow(dead_code)]

use std::fmt::Debug;

#[derive(Debug)]
struct Boom {
    inner: String,
}

impl AsRef<str> for Boom {
    fn as_ref(&self) -> &str {
        self.inner.as_str()
    }
}

fn as_ref<T: AsRef<str> + Debug>(b: T) {
    println!("boom: {:?}", b);
}

fn main() {
    as_ref(&Boom {
        inner: "abc".to_string(),
    });
}
