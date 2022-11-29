#![allow(unused_variables)]

use listing::message::{self, Messager};

use std::cell::RefCell;

fn main() {
    // let x = 5;
    // let y = &mut x;

    // let mut x = 5;
    // let bx = &x;
    // let mx = &mut x;
    // println!("{}, {}", bx, mx);

    // let s = RefCell::new(String::from("hello world!"));
    // let s1 = s.borrow();
    // let s2 = s.borrow_mut();
    //
    // println!("{}, {}", s1, s2);

    let mq = MsgQueue {
        msg_cache: RefCell::new(Vec::new()),
    };
    mq.send("hello world!".to_string());
}

struct MsgQueue {
    msg_cache: RefCell<Vec<String>>,
}

impl message::Messager for MsgQueue {
    fn send(&self, msg: String) {
        self.msg_cache.borrow_mut().push(msg);
    }
}
