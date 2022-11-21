use std::mem::size_of;

fn main() {
    let ptr = 42 as *const Vec<String>;

    println!("{}", size_of::<Vec<String>>());
    println!("{}", size_of::<Vec<i8>>());

    unsafe {
        let new_addr = ptr.offset(4);
        println!("{:p} -> {:p}", ptr, new_addr);
    }
}
