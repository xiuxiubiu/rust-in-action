#![allow(unused_variables)]
#![allow(dead_code)]

use std::rc::Rc;
use List::*;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

struct Node<'a> {
    num: i32,
    next: NewList<'a>,
}

enum NewList<'a> {
    N(&'a Node<'a>),
    Nil,
}

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    let na = Node {
        num: 5,
        next: NewList::N(&Node {
            num: 10,
            next: NewList::Nil,
        }),
    };
    let nla = NewList::N(&na);

    let nb = NewList::N(&Node {
        num: 3,
        next: NewList::N(&na),
    });
    let nc = NewList::N(&Node {
        num: 4,
        next: NewList::N(&na),
    });

    let mut four = 4;
    let rc_four = Rc::new(four);
    println!("{}", Rc::clone(&rc_four));
    four = 10;
    println!("{}", four)
}
