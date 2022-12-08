#![allow(unused_variables)]

fn main() {
    let a: i32 = 40;
    let b: Box<i32> = Box::new(60);

    let result = a + *b;

    println!("{} + {} = {}", a, b, result);
}
